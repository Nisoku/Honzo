use crate::types::{
    Compression, CoverType, FontEmbedding, LayoutMode, MarkupType, MathType, PmapEntry,
};
use crate::HonzoError;

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

#[cfg(feature = "compression")]
use lz4_flex::compress_prepend_size;

const MAGIC: &[u8; 4] = b"HONO";

#[derive(Clone)]
struct ChunkSpec {
    tag: [u8; 4],
    data: Vec<u8>,
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
        }
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
            data: data.to_vec(),
            compression,
            content_type_kind: 1,
            content_type_value: markup_type as u8,
            cover_type,
            alt_text: alt_text.map(|value| value.to_string()),
            font_embedding,
            font_license_url: font_license_url.map(|value| value.to_string()),
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
            data: data.to_vec(),
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
        let mut compressed_chunks: Vec<Vec<u8>> = Vec::with_capacity(self.chunks.len());
        let mut toc_entries = Vec::with_capacity(self.chunks.len());
        let mut data_offset = 0u64;

        for chunk in &self.chunks {
            let (compressed_data, size_compressed, size_raw) = match chunk.compression {
                Compression::None => {
                    let len = chunk.data.len() as u32;
                    (chunk.data.clone(), len, len)
                }
                Compression::Lz4 => {
                    #[cfg(not(feature = "compression"))]
                    {
                        let _ = &chunk.compression;
                        return Err(HonzoError::UnknownCompression(1));
                    }
                    #[cfg(feature = "compression")]
                    {
                        let compressed = compress_prepend_size(&chunk.data);
                        let size_raw = chunk.data.len() as u32;
                        let size_compressed = compressed.len() as u32;
                        (compressed, size_compressed, size_raw)
                    }
                }
            };

            let crc32 = if &chunk.tag == b"CHAP" {
                crc32(&chunk.data)
            } else {
                0u32
            };
            let alt_text = chunk.alt_text.as_deref();
            let font_license_url = chunk.font_license_url.as_deref();

            toc_entries.push(TocEntryWrite {
                chunk_type: chunk.tag,
                chunk_id: toc_entries.len() as u32,
                offset: data_offset,
                size_compressed,
                size_raw,
                compression: chunk.compression,
                content_type_kind: chunk.content_type_kind,
                content_type_value: chunk.content_type_value,
                cover_type: chunk.cover_type,
                flags: 0,
                crc32,
                alt_text,
                font_embedding: chunk.font_embedding,
                font_license_url,
            });

            data_offset += size_compressed as u64;
            compressed_chunks.push(compressed_data);
        }

        let toc_bytes = build_toc(&toc_entries, &self.pmap)?;
        let data_bytes = concat_chunks(&compressed_chunks);
        let extra_bytes = self.extra;
        let meta_bytes = self.meta;

        let flags = (self.flags & !0x0C) | ((self.layout as u32) << 2);
        let head = HonzoHeadWrite {
            version_major: 1,
            version_minor: 0,
            min_reader_version: 1,
            flags,
            chunk_count: toc_entries.len() as u32,
            toc_size: toc_bytes.len() as u64,
            data_size: data_bytes.len() as u64,
            extra_size: extra_bytes.len() as u64,
            meta_size: meta_bytes.len() as u64,
        };

        let mut out = Vec::new();
        out.extend_from_slice(MAGIC);
        write_head(&mut out, head);
        out.extend_from_slice(&toc_bytes);
        out.extend_from_slice(&data_bytes);
        out.extend_from_slice(&extra_bytes);
        out.extend_from_slice(&meta_bytes);
        Ok(out)
    }
}

impl Default for HonzoBuilder {
    fn default() -> Self {
        Self::new()
    }
}

struct HonzoHeadWrite {
    version_major: u8,
    version_minor: u8,
    min_reader_version: u16,
    flags: u32,
    chunk_count: u32,
    toc_size: u64,
    data_size: u64,
    extra_size: u64,
    meta_size: u64,
}

struct TocEntryWrite<'a> {
    chunk_type: [u8; 4],
    chunk_id: u32,
    offset: u64,
    size_compressed: u32,
    size_raw: u32,
    compression: Compression,
    content_type_kind: u8,
    content_type_value: u8,
    cover_type: CoverType,
    flags: u8,
    crc32: u32,
    alt_text: Option<&'a str>,
    font_embedding: Option<FontEmbedding>,
    font_license_url: Option<&'a str>,
}

fn build_toc(entries: &[TocEntryWrite<'_>], pmap: &[PmapEntry]) -> Result<Vec<u8>, HonzoError> {
    let mut out = Vec::new();
    out.extend_from_slice(&(entries.len() as u32).to_le_bytes());
    for entry in entries {
        out.extend_from_slice(&entry.chunk_type);
        out.extend_from_slice(&entry.chunk_id.to_le_bytes());
        out.extend_from_slice(&entry.offset.to_le_bytes());
        out.extend_from_slice(&entry.size_compressed.to_le_bytes());
        out.extend_from_slice(&entry.size_raw.to_le_bytes());
        out.push(entry.compression as u8);
        out.push(entry.content_type_kind);
        out.push(entry.content_type_value);
        out.push(entry.cover_type as u8);
        out.push(entry.flags);
        out.extend_from_slice(&entry.crc32.to_le_bytes());
        if let Some(text) = entry.alt_text {
            out.extend_from_slice(&(text.len() as u16).to_le_bytes());
            out.extend_from_slice(text.as_bytes());
        } else {
            out.extend_from_slice(&0u16.to_le_bytes());
        }

        if entry.chunk_type == *b"FONT" {
            out.push(entry.font_embedding.unwrap_or(FontEmbedding::Allowed) as u8);
            if let Some(url) = entry.font_license_url {
                out.extend_from_slice(&(url.len() as u16).to_le_bytes());
                out.extend_from_slice(url.as_bytes());
            } else {
                out.extend_from_slice(&0u16.to_le_bytes());
            }
        }
    }

    out.extend_from_slice(&(pmap.len() as u32).to_le_bytes());
    for entry in pmap {
        out.extend_from_slice(&entry.print_page.to_le_bytes());
        out.extend_from_slice(&entry.chunk_id.to_le_bytes());
        out.extend_from_slice(&entry.byte_offset.to_le_bytes());
    }
    Ok(out)
}

fn concat_chunks(chunks: &[Vec<u8>]) -> Vec<u8> {
    let total: usize = chunks.iter().map(|chunk| chunk.len()).sum();
    let mut out = Vec::with_capacity(total);
    for chunk in chunks {
        out.extend_from_slice(chunk);
    }
    out
}

fn write_head(out: &mut Vec<u8>, head: HonzoHeadWrite) {
    out.push(head.version_major);
    out.push(head.version_minor);
    out.extend_from_slice(&head.min_reader_version.to_le_bytes());
    out.extend_from_slice(&head.flags.to_le_bytes());
    out.extend_from_slice(&head.chunk_count.to_le_bytes());
    out.extend_from_slice(&head.toc_size.to_le_bytes());
    out.extend_from_slice(&head.data_size.to_le_bytes());
    out.extend_from_slice(&head.extra_size.to_le_bytes());
    out.extend_from_slice(&head.meta_size.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
}

fn crc32(data: &[u8]) -> u32 {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(data);
    hasher.finalize()
}
