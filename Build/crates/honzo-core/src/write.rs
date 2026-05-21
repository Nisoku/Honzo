use crate::types::{Compression, CoverType, FontEmbedding, LayoutMode, MarkupType, PmapEntry};
use crate::HonzoError;

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

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
            let size_compressed = chunk.data.len() as u32;
            let size_raw = chunk.data.len() as u32;
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
            compressed_chunks.push(chunk.data.clone());
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
    write_u32(&mut out, entries.len() as u32);
    for entry in entries {
        out.extend_from_slice(&entry.chunk_type);
        write_u32(&mut out, entry.chunk_id);
        write_u64(&mut out, entry.offset);
        write_u32(&mut out, entry.size_compressed);
        write_u32(&mut out, entry.size_raw);
        out.push(entry.compression as u8);
        out.push(entry.content_type_kind);
        out.push(entry.content_type_value);
        out.push(entry.cover_type as u8);
        out.push(entry.flags);
        write_u32(&mut out, entry.crc32);
        if let Some(text) = entry.alt_text {
            write_u16(&mut out, text.len() as u16);
            out.extend_from_slice(text.as_bytes());
        } else {
            write_u16(&mut out, 0);
        }

        if entry.chunk_type == *b"FONT" {
            out.push(entry.font_embedding.unwrap_or(FontEmbedding::Allowed) as u8);
            if let Some(url) = entry.font_license_url {
                write_u16(&mut out, url.len() as u16);
                out.extend_from_slice(url.as_bytes());
            } else {
                write_u16(&mut out, 0);
            }
        }
    }

    write_u32(&mut out, pmap.len() as u32);
    for entry in pmap {
        write_u32(&mut out, entry.print_page);
        write_u32(&mut out, entry.chunk_id);
        write_u32(&mut out, entry.byte_offset);
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
    write_u16(out, head.min_reader_version);
    write_u32(out, head.flags);
    write_u32(out, head.chunk_count);
    write_u64(out, head.toc_size);
    write_u64(out, head.data_size);
    write_u64(out, head.extra_size);
    write_u64(out, head.meta_size);
    write_u32(out, 0);
}

fn write_u16(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn write_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn write_u64(out: &mut Vec<u8>, value: u64) {
    out.extend_from_slice(&value.to_le_bytes());
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for byte in data {
        crc ^= *byte as u32;
        for _ in 0..8 {
            let mask = 0u32.wrapping_sub(crc & 1);
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}
