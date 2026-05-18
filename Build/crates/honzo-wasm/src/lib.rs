use honzo_std::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct HonzoWasm {
    buf: Vec<u8>,
    meta: Vec<u8>,
    toc: Vec<WasmTocEntry>,
    chunks: Vec<Vec<u8>>,
}

#[allow(dead_code)]
struct WasmTocEntry {
    chunk_type: [u8; 4],
    chunk_id: u32,
    offset: u64,
    size_compressed: u32,
    size_raw: u32,
    compression: honzo_core::Compression,
    markup_type: honzo_core::MarkupType,
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
        let p = honzo_core::HonzoParser::new(buf, reader_version)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        let meta = p
            .meta_bytes()
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?
            .to_vec();

        let entries: Vec<_> = p.toc_entries().collect();
        let mut chunks = Vec::with_capacity(entries.len());

        for entry in &entries {
            if entry.is_encrypted() {
                chunks.push(Vec::new());
                continue;
            }
            let raw = p
                .chunk_bytes(entry)
                .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
            let decompressed = decompress(raw, entry.compression, entry.size_raw)
                .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
            chunks.push(decompressed);
        }

        let toc = entries
            .iter()
            .map(|e| WasmTocEntry {
                chunk_type: e.chunk_type,
                chunk_id: e.chunk_id,
                offset: e.offset,
                size_compressed: e.size_compressed,
                size_raw: e.size_raw,
                compression: e.compression,
                markup_type: e.markup_type,
                cover_type: e.cover_type,
                flags: e.flags,
                crc32: e.crc32,
                alt_text: e.alt_text.map(|s| s.to_string()),
                font_embedding: e.font_embedding,
                font_license_url: e.font_license_url.map(|s| s.to_string()),
            })
            .collect();

        Ok(HonzoWasm {
            buf: buf.to_vec(),
            meta,
            toc,
            chunks,
        })
    }

    pub fn chunk_count(&self) -> u32 {
        self.chunks.len() as u32
    }

    pub fn layout_mode(&self) -> u8 {
        honzo_core::HonzoParser::new(&self.buf, 1)
            .map(|p| p.head().layout_mode() as u8)
            .unwrap_or(0)
    }

    pub fn has_drm(&self) -> bool {
        honzo_core::HonzoParser::new(&self.buf, 1)
            .map(|p| p.head().has_drm())
            .unwrap_or(false)
    }

    pub fn has_sidx(&self) -> bool {
        honzo_core::HonzoParser::new(&self.buf, 1)
            .map(|p| p.head().has_sidx())
            .unwrap_or(false)
    }

    pub fn get_chunk(&self, index: u32) -> Result<Vec<u8>, JsValue> {
        self.chunks
            .get(index as usize)
            .cloned()
            .ok_or_else(|| JsValue::from_str("chunk index out of bounds"))
    }

    pub fn get_meta(&self) -> Result<Vec<u8>, JsValue> {
        Ok(self.meta.clone())
    }

    pub fn get_meta_parsed(&self) -> Result<JsValue, JsValue> {
        let meta: HonzoMeta = rmp_serde::from_slice(&self.meta)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        serde_wasm_bindgen::to_value(&meta).map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    pub fn get_toc(&self) -> Result<JsValue, JsValue> {
        #[derive(serde::Serialize)]
        struct TocOut {
            chunk_type: String,
            chunk_id: u32,
            size_compressed: u32,
            size_raw: u32,
            compression: u8,
            markup_type: u8,
            #[serde(skip_serializing_if = "Option::is_none")]
            alt_text: Option<String>,
        }
        let js_entries: Vec<TocOut> = self
            .toc
            .iter()
            .map(|e| TocOut {
                chunk_type: std::str::from_utf8(&e.chunk_type)
                    .unwrap_or("????")
                    .to_string(),
                chunk_id: e.chunk_id,
                size_compressed: e.size_compressed,
                size_raw: e.size_raw,
                compression: e.compression as u8,
                markup_type: e.markup_type as u8,
                alt_text: e.alt_text.clone(),
            })
            .collect();
        serde_wasm_bindgen::to_value(&js_entries)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }
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
        #[serde(default)]
        markup_type: u8,
        #[serde(default)]
        alt_text: Option<String>,
    }

    #[derive(Deserialize)]
    struct BuildSpec {
        chunks: Vec<ChunkSpec>,
        meta: Option<serde_json::Value>,
        extra: Option<Vec<u8>>,
    }

    let spec: BuildSpec =
        serde_wasm_bindgen::from_value(spec).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

    let mut builder = HonzoBuilder::new();

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
        let markup = match chunk.markup_type {
            0 => MarkupType::Hmd,
            1 => MarkupType::Html,
            _ => return Err(JsValue::from_str("invalid markup_type")),
        };
        builder = builder.add_chunk(
            tag_arr,
            &chunk.data,
            compression,
            markup,
            CoverType::Front,
            chunk.alt_text.as_deref(),
            None,
            None,
        );
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

    builder
        .finalize()
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
}
