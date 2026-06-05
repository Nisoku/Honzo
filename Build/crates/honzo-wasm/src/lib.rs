use honzo_chunks::data::css::validate_css as validate_css_impl;
use honzo_chunks::data::font::{
    guess_font_format as guess_font_format_impl, validate_font as validate_font_impl,
};
use honzo_chunks::data::math::{
    latex_to_mathml as latex_to_mathml_impl, render_math as render_math_impl,
    validate_mathml as validate_mathml_impl,
};
use honzo_chunks::data::sidx::normalize_search_term as normalize_search_term_impl;
use honzo_chunks::extra::{anno, sync};
use honzo_core::{Compression, CoverType, MarkupType, MathType};
use honzo_io::*;
use wasm_bindgen::prelude::*;

/// A DRM key pair for building/reading encrypted Honzo files.
/// Generated externally (X25519 keys, 32 bytes each).
#[wasm_bindgen]
pub struct DrmKeyPair {
    recipient_public_key: Vec<u8>,
    private_key: Vec<u8>,
}

#[wasm_bindgen]
impl DrmKeyPair {
    #[wasm_bindgen(constructor)]
    pub fn new(recipient_public_key: Vec<u8>, private_key: Vec<u8>) -> Self {
        Self {
            recipient_public_key,
            private_key,
        }
    }

    pub fn recipient_public_key(&self) -> Vec<u8> {
        self.recipient_public_key.clone()
    }

    pub fn private_key(&self) -> Vec<u8> {
        self.private_key.clone()
    }
}

#[wasm_bindgen]
pub struct HonzoWasm {
    buf: Vec<u8>,
    reader_version: u16,
    meta: Vec<u8>,
    data_start: usize,
    toc: Vec<WasmTocEntry>,
    cek: Option<[u8; 32]>,
}

#[allow(dead_code)]
struct WasmTocEntry {
    chunk_type: [u8; 4],
    chunk_id: u32,
    offset: u64,
    size_compressed: u32,
    size_raw: u32,
    compression: honzo_core::Compression,
    content_type_kind: u8,
    content_type_value: u8,
    cover_type: honzo_core::CoverType,
    flags: u8,
    crc32: u32,
    alt_text: Option<String>,
    font_embedding: Option<honzo_core::FontEmbedding>,
    font_license_url: Option<String>,
}

#[wasm_bindgen]
impl HonzoWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(buf: &[u8], reader_version: u16) -> Result<HonzoWasm, JsValue> {
        Self::new_inner(buf, reader_version, None)
    }

    /// Create a reader with an X25519 private key (32 bytes) for DRM decryption.
    #[wasm_bindgen]
    pub fn with_private_key(
        buf: &[u8],
        reader_version: u16,
        private_key: &[u8],
    ) -> Result<HonzoWasm, JsValue> {
        Self::new_inner(buf, reader_version, Some(private_key))
    }

    fn new_inner(
        buf: &[u8],
        reader_version: u16,
        private_key: Option<&[u8]>,
    ) -> Result<HonzoWasm, JsValue> {
        let p = honzo_core::HonzoParser::new(buf, reader_version)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        let meta = p
            .meta_bytes()
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?
            .to_vec();

        let head = p.head();
        let data_start = (52 + head.toc_size) as usize;

        let toc = p
            .toc_entries()
            .map(|e| WasmTocEntry {
                chunk_type: e.chunk_type,
                chunk_id: e.chunk_id,
                offset: e.offset,
                size_compressed: e.size_compressed,
                size_raw: e.size_raw,
                compression: e.compression,
                content_type_kind: e.content_type_kind,
                content_type_value: e.content_type_value,
                cover_type: e.cover_type,
                flags: e.flags,
                crc32: e.crc32,
                alt_text: e.alt_text.map(|s| s.to_string()),
                font_embedding: e.font_embedding,
                font_license_url: e.font_license_url.map(|s| s.to_string()),
            })
            .collect();

        // Parse DRM envelope and unwrap CEK if private key is provided
        let cek = if head.has_drm() {
            if let Some(pk) = private_key {
                let extra = p
                    .extra_bytes()
                    .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
                let entries = honzo_io::parse_extra(extra)
                    .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
                let entry = honzo_io::find_extra(&entries, honzo_chunks::extra::drm::NAMESPACE)
                    .ok_or_else(|| JsValue::from_str("DRM flag set but no DRM extra entry"))?;
                let envelope = honzo_chunks::extra::drm::parse_drm(&entry.body)
                    .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
                let cek = honzo_io::crypto::unwrap_cek(&envelope.key_envelope, pk)
                    .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
                Some(cek)
            } else {
                None
            }
        } else {
            None
        };

        Ok(HonzoWasm {
            buf: buf.to_vec(),
            reader_version,
            meta,
            data_start,
            toc,
            cek,
        })
    }

    pub fn chunk_count(&self) -> u32 {
        self.toc.len() as u32
    }

    pub fn version_major(&self) -> u8 {
        self.buf.get(4).copied().unwrap_or(0)
    }

    pub fn version_minor(&self) -> u8 {
        self.buf.get(5).copied().unwrap_or(0)
    }

    pub fn min_reader_version(&self) -> u16 {
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&self.buf[6..8]);
        u16::from_le_bytes(bytes)
    }

    pub fn flags(&self) -> u32 {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.buf[8..12]);
        u32::from_le_bytes(bytes)
    }

    pub fn toc_size(&self) -> u64 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.buf[16..24]);
        u64::from_le_bytes(bytes)
    }

    pub fn data_size(&self) -> u64 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.buf[24..32]);
        u64::from_le_bytes(bytes)
    }

    pub fn extra_size(&self) -> u64 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.buf[32..40]);
        u64::from_le_bytes(bytes)
    }

    pub fn meta_size(&self) -> u64 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.buf[40..48]);
        u64::from_le_bytes(bytes)
    }

    pub fn layout_mode(&self) -> u8 {
        honzo_core::HonzoParser::new(&self.buf, self.reader_version)
            .map(|p| p.head().layout_mode() as u8)
            .unwrap_or(0)
    }

    pub fn has_drm(&self) -> bool {
        honzo_core::HonzoParser::new(&self.buf, self.reader_version)
            .map(|p| p.head().has_drm())
            .unwrap_or(false)
    }

    pub fn has_sidx(&self) -> bool {
        honzo_core::HonzoParser::new(&self.buf, self.reader_version)
            .map(|p| p.head().has_sidx())
            .unwrap_or(false)
    }

    pub fn has_annotations(&self) -> bool {
        self.flags() & 0x40 != 0
    }

    pub fn has_sync(&self) -> bool {
        self.flags() & 0x80 != 0
    }

    pub fn layout_mode_name(&self) -> String {
        match (self.layout_mode() >> 2) & 3 {
            0 => "Reflowable".to_string(),
            1 => "Fixed".to_string(),
            2 => "Scroll".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    pub fn compression_name(&self) -> String {
        match self.layout_mode() & 3 {
            0 => "None".to_string(),
            1 => "Lz4".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    pub fn compression_name_for_chunk(&self, index: u32) -> String {
        self.toc
            .get(index as usize)
            .map(|e| match e.compression {
                Compression::None => "None".to_string(),
                Compression::Lz4 => "Lz4".to_string(),
            })
            .unwrap_or_else(|| "Unknown".to_string())
    }

    pub fn content_type_name_for_chunk(&self, index: u32) -> String {
        self.toc
            .get(index as usize)
            .map(|e| {
                if e.content_type_kind == 1 {
                    // Markup
                    match e.content_type_value {
                        0 => "Markdown".to_string(),
                        1 => "Html".to_string(),
                        _ => "Unknown".to_string(),
                    }
                } else if e.content_type_kind == 2 {
                    // Math
                    match e.content_type_value {
                        0 => "MathML".to_string(),
                        1 => "LaTeX".to_string(),
                        _ => "Unknown".to_string(),
                    }
                } else {
                    "Unknown".to_string()
                }
            })
            .unwrap_or_else(|| "Unknown".to_string())
    }

    pub fn get_chunk(&self, index: u32) -> Result<Vec<u8>, JsValue> {
        let entry = self
            .toc
            .get(index as usize)
            .ok_or_else(|| JsValue::from_str("chunk index out of bounds"))?;

        let start = self.data_start + entry.offset as usize;
        let end = start + entry.size_compressed as usize;
        if end > self.buf.len() {
            return Err(JsValue::from_str("chunk data truncated"));
        }

        let raw = &self.buf[start..end];

        if entry.flags & 0x01 != 0 {
            if let Some(ref cek) = self.cek {
                let compressed = honzo_io::crypto::decrypt_chunk(raw, cek)
                    .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
                return decompress(&compressed, entry.compression, entry.size_raw)
                    .map_err(|e| JsValue::from_str(&format!("{:?}", e)));
            }
            return Err(JsValue::from_str("chunk is encrypted"));
        }

        decompress(raw, entry.compression, entry.size_raw)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    pub fn get_meta(&self) -> Result<Vec<u8>, JsValue> {
        Ok(self.meta.clone())
    }

    pub fn get_meta_parsed(&self) -> Result<JsValue, JsValue> {
        let meta: HonzoMeta = rmp_serde::from_slice(&self.meta)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        serde_wasm_bindgen::to_value(&meta).map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    pub fn get_extra(&self) -> Result<Vec<u8>, JsValue> {
        let parser = honzo_core::HonzoParser::new(&self.buf, self.reader_version)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        parser
            .extra_bytes()
            .map(|b| b.to_vec())
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    pub fn get_annotations(&self) -> Result<JsValue, JsValue> {
        let parser = honzo_core::HonzoParser::new(&self.buf, self.reader_version)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        let extra = parser
            .extra_bytes()
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        let entries =
            honzo_io::parse_extra(extra).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        let entry = honzo_io::find_extra(&entries, anno::NAMESPACE)
            .ok_or_else(|| JsValue::from_str("no annotations in extra"))?;
        let annotations =
            anno::parse_anno(&entry.body).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        serde_wasm_bindgen::to_value(&annotations)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    pub fn get_sync_cues(&self) -> Result<JsValue, JsValue> {
        let parser = honzo_core::HonzoParser::new(&self.buf, self.reader_version)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        let extra = parser
            .extra_bytes()
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        let entries =
            honzo_io::parse_extra(extra).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        let entry = honzo_io::find_extra(&entries, sync::NAMESPACE)
            .ok_or_else(|| JsValue::from_str("no sync cues in extra"))?;
        let cues =
            sync::parse_sync(&entry.body).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        serde_wasm_bindgen::to_value(&cues).map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    pub fn get_toc(&self) -> Result<JsValue, JsValue> {
        #[derive(serde::Serialize)]
        struct TocOut {
            chunk_type: String,
            chunk_id: u32,
            offset: u64,
            size_compressed: u32,
            size_raw: u32,
            compression: u8,
            content_type_kind: u8,
            content_type_value: u8,
            cover_type: u8,
            flags: u8,
            crc32: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            alt_text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            font_embedding: Option<u8>,
            #[serde(skip_serializing_if = "Option::is_none")]
            font_license_url: Option<String>,
        }
        let js_entries: Vec<TocOut> = self
            .toc
            .iter()
            .map(|e| TocOut {
                chunk_type: std::str::from_utf8(&e.chunk_type)
                    .unwrap_or("????")
                    .to_string(),
                chunk_id: e.chunk_id,
                offset: e.offset,
                size_compressed: e.size_compressed,
                size_raw: e.size_raw,
                compression: e.compression as u8,
                content_type_kind: e.content_type_kind,
                content_type_value: e.content_type_value,
                cover_type: e.cover_type as u8,
                flags: e.flags,
                crc32: e.crc32,
                alt_text: e.alt_text.clone(),
                font_embedding: e.font_embedding.map(|e| e as u8),
                font_license_url: e.font_license_url.clone(),
            })
            .collect();
        serde_wasm_bindgen::to_value(&js_entries)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    pub fn get_chapters_text(&self) -> Result<Vec<String>, JsValue> {
        let mut texts = Vec::new();
        let chapter_entries: Vec<_> = self
            .toc
            .iter()
            .filter(|entry| {
                matches!(
                    std::str::from_utf8(&entry.chunk_type).unwrap_or("????"),
                    "CHAP" | "NOTE" | "MATH"
                )
            })
            .collect();

        for entry in &chapter_entries {
            let bytes = self.get_chunk(entry.chunk_id)?;
            texts.push(chapter_text_for_entry(entry, &bytes));
        }

        Ok(texts)
    }

    pub fn get_chapter_text(&self, index: u32) -> Result<String, JsValue> {
        let chapter_entries: Vec<_> = self
            .toc
            .iter()
            .filter(|entry| {
                matches!(
                    std::str::from_utf8(&entry.chunk_type).unwrap_or("????"),
                    "CHAP" | "NOTE" | "MATH"
                )
            })
            .collect();

        let entry = chapter_entries
            .get(index as usize)
            .ok_or_else(|| JsValue::from_str("chapter index out of bounds"))?;
        let bytes = self.get_chunk(entry.chunk_id)?;
        Ok(chapter_text_for_entry(entry, &bytes))
    }
}

fn chapter_text_for_entry(entry: &WasmTocEntry, bytes: &[u8]) -> String {
    let raw = String::from_utf8_lossy(bytes);
    let chunk_type = std::str::from_utf8(&entry.chunk_type).unwrap_or("????");
    let is_html = entry.content_type_kind == 1 && entry.content_type_value == 1;

    if is_html && matches!(chunk_type, "CHAP" | "NOTE") {
        strip_html_for_text(&raw)
    } else {
        raw.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn strip_html_for_text(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    let mut tag_buf = String::new();
    let mut last_was_space = false;

    for c in html.chars() {
        if in_tag {
            if c == '>' {
                in_tag = false;
                let tag = tag_buf.trim().trim_start_matches('/').to_ascii_lowercase();
                if tag.starts_with('p')
                    || tag.starts_with("div")
                    || tag.starts_with("br")
                    || tag.starts_with('h')
                    || tag.starts_with("li")
                    || tag.starts_with("blockquote")
                    || tag.starts_with("tr")
                    || tag.starts_with("td")
                    || tag.starts_with("th")
                {
                    out.push('\n');
                    last_was_space = false;
                }
                tag_buf.clear();
            } else {
                tag_buf.push(c);
            }
        } else if c == '<' {
            in_tag = true;
            tag_buf.clear();
        } else if c == '&' {
            out.push('&');
            last_was_space = false;
        } else if c.is_whitespace() {
            if !last_was_space {
                out.push(' ');
                last_was_space = true;
            }
        } else {
            out.push(c);
            last_was_space = false;
        }
    }

    out.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

#[wasm_bindgen]
pub fn honzo_build(spec: JsValue) -> Result<Vec<u8>, JsValue> {
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct ChunkSpec {
        tag: String,
        data: Vec<u8>,
        #[serde(default)]
        compression: u8,
        content_type_kind: u8,
        content_type_value: u8,
        #[serde(default)]
        cover_type: u8,
        #[serde(default)]
        alt_text: Option<String>,
        #[serde(default)]
        font_embedding: Option<u8>,
        #[serde(default)]
        font_license_url: Option<String>,
    }

    #[derive(Deserialize)]
    struct BuildSpec {
        chunks: Vec<ChunkSpec>,
        meta: Option<serde_json::Value>,
        extra: Option<Vec<u8>>,
        annotations: Option<Vec<u8>>,
        sync_cues: Option<Vec<u8>>,
        #[serde(default = "default_language")]
        language: String,
        #[serde(default = "default_auto_sidx")]
        auto_sidx: bool,
        #[serde(default = "default_auto_covt")]
        auto_covt: bool,
        #[serde(default)]
        layout: u8,
        #[serde(default)]
        flags: u32,
        #[serde(default = "default_min_reader_version")]
        min_reader_version: u16,
        #[serde(default)]
        drm: Option<DrmBuildSpec>,
    }

    #[derive(Deserialize)]
    struct DrmBuildSpec {
        /// Chunk IDs to encrypt
        encrypt_chunk_ids: Vec<u32>,
        /// X25519 public key (32 bytes)
        #[serde(alias = "public_key_der")]
        recipient_public_key: Vec<u8>,
        /// Optional license URL
        #[serde(default)]
        license_url: Option<String>,
        /// Optional expiry timestamp
        #[serde(default)]
        expires_at: Option<u64>,
    }

    fn default_language() -> String {
        "en".to_string()
    }

    fn default_auto_sidx() -> bool {
        true
    }

    fn default_auto_covt() -> bool {
        true
    }

    fn default_min_reader_version() -> u16 {
        1
    }

    let spec: BuildSpec =
        serde_wasm_bindgen::from_value(spec).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

    let mut builder = HonzoBuilder::new()
        .set_language(&spec.language)
        .set_auto_sidx(spec.auto_sidx)
        .set_auto_covt(spec.auto_covt);

    if spec.flags != 0 {
        builder = builder.set_flags(spec.flags);
    }

    if spec.min_reader_version != 1 {
        builder = builder.set_min_reader_version(spec.min_reader_version);
    }

    let layout = match spec.layout {
        0 => honzo_core::LayoutMode::Reflowable,
        1 => honzo_core::LayoutMode::Fixed,
        2 => honzo_core::LayoutMode::Scroll,
        _ => return Err(JsValue::from_str("invalid layout mode")),
    };
    builder = builder.set_layout(layout);

    for chunk in &spec.chunks {
        if chunk.tag.len() != 4 {
            return Err(JsValue::from_str(&format!(
                "invalid chunk tag: {}",
                chunk.tag
            )));
        }
        let mut tag_arr = [0u8; 4];
        tag_arr.copy_from_slice(chunk.tag.as_bytes());
        let compression = match chunk.compression {
            0 => Compression::None,
            1 => Compression::Lz4,
            _ => return Err(JsValue::from_str("invalid compression")),
        };
        // interpret content type according to new two-byte format
        if &tag_arr == b"MATH" {
            if chunk.content_type_kind != 2 {
                return Err(JsValue::from_str("invalid content_type_kind for MATH"));
            }
            let m = match chunk.content_type_value {
                0 => MathType::MathML,
                1 => MathType::LaTeX,
                _ => return Err(JsValue::from_str("invalid content_type_value for MATH")),
            };
            builder = builder.add_math_chunk(&chunk.data, m, compression);
        } else {
            if chunk.content_type_kind != 1 {
                return Err(JsValue::from_str(
                    "invalid content_type_kind for markup chunk",
                ));
            }
            let markup = match &tag_arr {
                b"CHAP" | b"NOTE" => match chunk.content_type_value {
                    0 => MarkupType::Markdown,
                    1 => MarkupType::Html,
                    _ => return Err(JsValue::from_str("invalid content_type_value")),
                },
                _ => {
                    if chunk.content_type_value != 0 {
                        return Err(JsValue::from_str(
                            "content_type_value must be 0 for this chunk type",
                        ));
                    }
                    MarkupType::Markdown
                }
            };
            let cover = match chunk.cover_type {
                0 => CoverType::Front,
                1 => CoverType::Back,
                2 => CoverType::FullSpread,
                _ => CoverType::Front,
            };
            let font_embedding = chunk.font_embedding.map(|v| match v {
                0 => honzo_core::FontEmbedding::Allowed,
                1 => honzo_core::FontEmbedding::PrintOnly,
                2 => honzo_core::FontEmbedding::NoModify,
                3 => honzo_core::FontEmbedding::NoEmbed,
                _ => honzo_core::FontEmbedding::Allowed,
            });
            builder = builder.add_chunk(
                tag_arr,
                &chunk.data,
                compression,
                markup,
                cover,
                chunk.alt_text.as_deref(),
                font_embedding,
                chunk.font_license_url.as_deref(),
            );
        }
    }

    if let Some(ref meta_value) = spec.meta {
        let meta: HonzoMeta = serde_json::from_value(meta_value.clone())
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        let msgpack =
            rmp_serde::to_vec(&meta).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        builder = builder.set_meta(&msgpack);
    }

    if let Some(ref extra) = spec.extra {
        builder = builder.set_extra(extra);
    }

    if let Some(ref anno_bytes) = spec.annotations {
        let annotations: Vec<anno::Annotation> = rmp_serde::from_slice(anno_bytes)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        builder = builder.add_annotation(&annotations);
    }

    if let Some(ref sync_bytes) = spec.sync_cues {
        let cues: Vec<sync::SyncCue> = rmp_serde::from_slice(sync_bytes)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        builder = builder.add_sync_cue(&cues);
    }

    if let Some(ref drm_spec) = spec.drm {
        let config = DrmConfig {
            encrypt_chunk_ids: drm_spec.encrypt_chunk_ids.clone(),
            recipient_public_key: drm_spec.recipient_public_key.clone(),
            license_url: drm_spec.license_url.clone(),
            expires_at: drm_spec.expires_at,
        };
        builder = builder.set_drm_config(config);
    }

    builder
        .finalize()
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
}

#[wasm_bindgen]
pub fn convert_epub(bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    honzo_convert::from_epub(bytes).map_err(|e| JsValue::from_str(&format!("{:?}", e)))
}

#[wasm_bindgen]
pub fn normalize_search_term(term: &str, lang: &str) -> String {
    normalize_search_term_impl(term, lang)
}

#[wasm_bindgen]
pub fn validate_mathml(bytes: &[u8]) -> bool {
    validate_mathml_impl(bytes).is_ok()
}

#[wasm_bindgen]
pub fn latex_to_mathml(bytes: &[u8]) -> Result<String, JsValue> {
    latex_to_mathml_impl(bytes).map_err(|e| JsValue::from_str(&format!("{:?}", e)))
}

#[wasm_bindgen]
pub fn render_math(bytes: &[u8], math_type: u8) -> Result<String, JsValue> {
    let math_type =
        MathType::from_u8(math_type).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    render_math_impl(bytes, math_type).map_err(|e| JsValue::from_str(&format!("{:?}", e)))
}

#[wasm_bindgen]
pub fn validate_css(bytes: &[u8]) -> bool {
    validate_css_impl(bytes).is_ok()
}

#[wasm_bindgen]
pub fn validate_font(bytes: &[u8]) -> bool {
    validate_font_impl(bytes).is_ok()
}

#[wasm_bindgen]
pub fn guess_font_format(bytes: &[u8]) -> Option<String> {
    guess_font_format_impl(bytes).map(|s| s.to_string())
}
