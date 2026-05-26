use honzo_chunks::data::covr::generate_covt;
use honzo_chunks::data::sidx::build_sidx;
use honzo_core::{
    Compression, CoverType, FontEmbedding, HonzoError, LayoutMode, MarkupType, MathType, PmapEntry,
};
use std::vec::Vec;

fn strip_html_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    let mut tag_buf = String::new();
    let mut last_was_newline = false;

    for c in s.chars() {
        if c == '<' {
            tag_buf.clear();
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
            let tag = tag_buf.trim().trim_start_matches('/').to_ascii_lowercase();
            let is_block = tag.is_empty()
                || tag.starts_with('p')
                || tag.starts_with("div")
                || tag.starts_with("br")
                || tag.starts_with("li")
                || tag.starts_with('h')
                || tag.starts_with("blockquote")
                || tag.starts_with("tr")
                || tag.starts_with("td");
            if is_block && !last_was_newline {
                out.push('\n');
                last_was_newline = true;
            }
        } else if in_tag {
            tag_buf.push(c);
        } else if c == '&' {
            let mut entity = String::new();
            for ec in s.chars().skip(1) {
                if ec == ';' {
                    break;
                }
                entity.push(ec);
            }
            let decoded = match entity.as_str() {
                "amp" => "&",
                "lt" => "<",
                "gt" => ">",
                "quot" => "\"",
                "apos" => "'",
                "nbsp" => " ",
                _ => "",
            };
            if !decoded.is_empty() {
                out.push_str(decoded);
                last_was_newline = false;
            }
        } else if c.is_alphanumeric() || c.is_whitespace() || c.is_ascii_punctuation() {
            if c.is_whitespace() && c != '\n' {
                if !last_was_newline {
                    out.push(' ');
                    last_was_newline = false;
                }
            } else {
                out.push(c);
                last_was_newline = false;
            }
        }
    }

    out.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

const MAGIC: &[u8; 4] = b"HONO";

#[derive(Clone)]
struct ChunkSpec {
    tag: [u8; 4],
    raw_data: Vec<u8>,
    compression: Compression,
    content_type_kind: u8,
    content_type_value: u8,
    cover_type: CoverType,
    alt_text: Option<String>,
    font_embedding: Option<FontEmbedding>,
    font_license_url: Option<String>,
}

#[derive(Clone)]
pub struct HonzoBuilder {
    layout: LayoutMode,
    flags: u32,
    chunks: Vec<ChunkSpec>,
    pmap: Vec<PmapEntry>,
    meta: Vec<u8>,
    extra: Vec<u8>,
    auto_sidx: bool,
    auto_covt: bool,
    language: String,
}

impl HonzoBuilder {
    pub fn new() -> Self {
        Self {
            layout: LayoutMode::Reflowable,
            flags: 0,
            chunks: Vec::new(),
            pmap: Vec::new(),
            meta: Vec::new(),
            extra: Vec::new(),
            auto_sidx: true,
            auto_covt: true,
            language: "en".to_string(),
        }
    }

    pub fn set_auto_covt(mut self, enable: bool) -> Self {
        self.auto_covt = enable;
        self
    }

    pub fn set_auto_sidx(mut self, enable: bool) -> Self {
        self.auto_sidx = enable;
        self
    }

    pub fn set_language(mut self, lang: &str) -> Self {
        self.language = lang.to_string();
        self
    }

    pub fn set_layout(mut self, layout: LayoutMode) -> Self {
        self.layout = layout;
        self
    }

    pub fn set_flags(mut self, flags: u32) -> Self {
        self.flags = flags;
        self
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_chunk(
        mut self,
        tag: [u8; 4],
        data: &[u8],
        compression: Compression,
        markup_type: MarkupType,
        cover_type: CoverType,
        alt_text: Option<&str>,
        font_embedding: Option<FontEmbedding>,
        font_license_url: Option<&str>,
    ) -> Self {
        self.chunks.push(ChunkSpec {
            tag,
            raw_data: data.to_vec(),
            compression,
            content_type_kind: 1,
            content_type_value: markup_type as u8,
            cover_type,
            alt_text: alt_text.map(String::from),
            font_embedding,
            font_license_url: font_license_url.map(String::from),
        });
        self
    }

    pub fn add_math_chunk(
        mut self,
        data: &[u8],
        math_type: MathType,
        compression: Compression,
    ) -> Self {
        self.chunks.push(ChunkSpec {
            tag: *b"MATH",
            raw_data: data.to_vec(),
            compression,
            content_type_kind: 2,
            content_type_value: math_type as u8,
            cover_type: CoverType::Front,
            alt_text: None,
            font_embedding: None,
            font_license_url: None,
        });
        self
    }

    pub fn add_pmap_entry(mut self, entry: PmapEntry) -> Self {
        self.pmap.push(entry);
        self
    }

    pub fn set_meta(mut self, msgpack: &[u8]) -> Self {
        self.meta = msgpack.to_vec();
        self
    }

    pub fn set_extra(mut self, extra: &[u8]) -> Self {
        self.extra = extra.to_vec();
        self
    }

    pub fn finalize(self) -> Result<Vec<u8>, HonzoError> {
        let mut final_builder = self;
        if final_builder.auto_sidx {
            let mut chapters = Vec::new();
            for (id, chunk) in final_builder.chunks.iter().enumerate() {
                if chunk.tag == *b"CHAP" {
                    chapters.push((id as u32, chunk.raw_data.clone()));
                }
            }
            if !chapters.is_empty() {
                let chapter_texts: Vec<String> = chapters
                    .iter()
                    .map(|(_, data)| strip_html_tags(std::str::from_utf8(data).unwrap_or("")))
                    .collect();
                let chapters_refs: Vec<(u32, &str)> = chapters
                    .iter()
                    .zip(chapter_texts.iter())
                    .map(|((id, _), text)| (*id, text.as_str()))
                    .collect();

                let sidx_data = build_sidx(&chapters_refs, &final_builder.language)?;
                final_builder.flags |= 0x20;
                final_builder = final_builder.add_chunk(
                    *b"SIDX",
                    &sidx_data,
                    Compression::Lz4,
                    MarkupType::Markdown,
                    CoverType::Front,
                    None,
                    None,
                    None,
                );
            }
        }

        if !final_builder.extra.is_empty() {
            let _ = crate::validate_extra(&final_builder.extra)
                .map_err(|e| eprintln!("Warning: extra data has unrecognised namespace: {:?}", e));
        }

        if final_builder.auto_covt {
            let has_covr = final_builder.chunks.iter().any(|c| c.tag == *b"COVR");
            let has_covt = final_builder.chunks.iter().any(|c| c.tag == *b"COVT");
            if has_covr && !has_covt {
                if let Some(covr) = final_builder.chunks.iter().find(|c| c.tag == *b"COVR") {
                    if let Ok(covt_data) = generate_covt(&covr.raw_data) {
                        final_builder = final_builder.add_chunk(
                            *b"COVT",
                            &covt_data,
                            Compression::Lz4,
                            MarkupType::Markdown,
                            CoverType::Front,
                            None,
                            None,
                            None,
                        );
                    }
                }
            }
        }

        let chunk_count = final_builder.chunks.len() as u32;
        let mut toc_bytes = Vec::new();
        let mut data_bytes = Vec::new();

        toc_bytes.extend_from_slice(&chunk_count.to_le_bytes());

        for (chunk_id, chunk) in final_builder.chunks.iter().enumerate() {
            let (compressed, size_compressed, size_raw, crc32) = prepare_chunk(chunk)?;

            toc_bytes.extend_from_slice(&chunk.tag);
            toc_bytes.extend_from_slice(&(chunk_id as u32).to_le_bytes());
            let offset = data_bytes.len() as u64;
            toc_bytes.extend_from_slice(&offset.to_le_bytes());
            toc_bytes.extend_from_slice(&size_compressed.to_le_bytes());
            toc_bytes.extend_from_slice(&size_raw.to_le_bytes());
            toc_bytes.push(chunk.compression as u8);
            toc_bytes.push(chunk.content_type_kind);
            toc_bytes.push(chunk.content_type_value);
            toc_bytes.push(chunk.cover_type as u8);
            toc_bytes.push(0);
            toc_bytes.extend_from_slice(&crc32.to_le_bytes());

            if let Some(text) = &chunk.alt_text {
                toc_bytes.extend_from_slice(&(text.len() as u16).to_le_bytes());
                toc_bytes.extend_from_slice(text.as_bytes());
            } else {
                toc_bytes.extend_from_slice(&0u16.to_le_bytes());
            }

            if chunk.tag == *b"FONT" {
                let embedding = chunk.font_embedding.unwrap_or(FontEmbedding::Allowed) as u8;
                toc_bytes.push(embedding);
                if let Some(url) = &chunk.font_license_url {
                    toc_bytes.extend_from_slice(&(url.len() as u16).to_le_bytes());
                    toc_bytes.extend_from_slice(url.as_bytes());
                } else {
                    toc_bytes.extend_from_slice(&0u16.to_le_bytes());
                }
            }

            data_bytes.extend_from_slice(&compressed);
        }

        toc_bytes.extend_from_slice(&(final_builder.pmap.len() as u32).to_le_bytes());
        for entry in &final_builder.pmap {
            toc_bytes.extend_from_slice(&entry.print_page.to_le_bytes());
            toc_bytes.extend_from_slice(&entry.chunk_id.to_le_bytes());
            toc_bytes.extend_from_slice(&entry.byte_offset.to_le_bytes());
        }

        let toc_size = toc_bytes.len() as u64;
        let data_size = data_bytes.len() as u64;
        let extra_size = final_builder.extra.len() as u64;
        let meta_size = final_builder.meta.len() as u64;

        let flags = (final_builder.flags & !0x0C) | ((final_builder.layout as u32) << 2);

        let mut out = Vec::new();
        out.extend_from_slice(MAGIC);
        out.push(1);
        out.push(0);
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&flags.to_le_bytes());
        out.extend_from_slice(&chunk_count.to_le_bytes());
        out.extend_from_slice(&toc_size.to_le_bytes());
        out.extend_from_slice(&data_size.to_le_bytes());
        out.extend_from_slice(&extra_size.to_le_bytes());
        out.extend_from_slice(&meta_size.to_le_bytes());
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(&toc_bytes);
        out.extend_from_slice(&data_bytes);
        out.extend_from_slice(&final_builder.extra);
        out.extend_from_slice(&final_builder.meta);

        Ok(out)
    }
}

impl Default for HonzoBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn prepare_chunk(chunk: &ChunkSpec) -> Result<(Vec<u8>, u32, u32, u32), HonzoError> {
    let (compressed, size_compressed, size_raw) = match chunk.compression {
        Compression::None => {
            let raw = chunk.raw_data.clone();
            let len = raw.len() as u32;
            (raw, len, len)
        }
        Compression::Lz4 => {
            let compressed = lz4_flex::compress(&chunk.raw_data);
            let size_raw = chunk.raw_data.len() as u32;
            let size_compressed = compressed.len() as u32;
            (compressed, size_compressed, size_raw)
        }
    };

    let crc32 = if chunk.tag == *b"CHAP" {
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(&chunk.raw_data);
        hasher.finalize()
    } else {
        0
    };

    Ok((compressed, size_compressed, size_raw, crc32))
}
