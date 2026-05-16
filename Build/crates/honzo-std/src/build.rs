use honzo_core::{
    Compression, CoverType, FontEmbedding, HonzoError, LayoutMode, MarkupType, PmapEntry,
};
use std::io::Write;
use std::vec::Vec;

const MAGIC: &[u8; 4] = b"HONO";

#[derive(Clone)]
struct ChunkSpec {
    tag: [u8; 4],
    raw_data: Vec<u8>,
    compression: Compression,
    markup_type: MarkupType,
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
            markup_type,
            cover_type,
            alt_text: alt_text.map(String::from),
            font_embedding,
            font_license_url: font_license_url.map(String::from),
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
        let chunk_count = self.chunks.len() as u32;
        let mut toc_bytes = Vec::new();
        let mut data_bytes = Vec::new();

        toc_bytes.extend_from_slice(&chunk_count.to_le_bytes());

        for (chunk_id, chunk) in self.chunks.iter().enumerate() {
            let (compressed, size_compressed, size_raw, crc32) = prepare_chunk(chunk)?;

            toc_bytes.extend_from_slice(&chunk.tag);
            toc_bytes.extend_from_slice(&(chunk_id as u32).to_le_bytes());
            let offset = data_bytes.len() as u64;
            toc_bytes.extend_from_slice(&offset.to_le_bytes());
            toc_bytes.extend_from_slice(&size_compressed.to_le_bytes());
            toc_bytes.extend_from_slice(&size_raw.to_le_bytes());
            toc_bytes.push(chunk.compression as u8);
            toc_bytes.push(chunk.markup_type as u8);
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

        toc_bytes.extend_from_slice(&(self.pmap.len() as u32).to_le_bytes());
        for entry in &self.pmap {
            toc_bytes.extend_from_slice(&entry.print_page.to_le_bytes());
            toc_bytes.extend_from_slice(&entry.chunk_id.to_le_bytes());
            toc_bytes.extend_from_slice(&entry.byte_offset.to_le_bytes());
        }

        let toc_size = toc_bytes.len() as u64;
        let data_size = data_bytes.len() as u64;
        let extra_size = self.extra.len() as u64;
        let meta_size = self.meta.len() as u64;

        let flags = (self.flags & !0x0C) | ((self.layout as u32) << 2);

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
        out.extend_from_slice(&self.extra);
        out.extend_from_slice(&self.meta);

        Ok(out)
    }
}

fn prepare_chunk(chunk: &ChunkSpec) -> Result<(Vec<u8>, u32, u32, u32), HonzoError> {
    let (compressed, size_compressed, size_raw) = match chunk.compression {
        Compression::None => {
            let raw = chunk.raw_data.clone();
            let len = raw.len() as u32;
            (raw, len, len)
        }
        Compression::Zlib => {
            let mut encoder =
                flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
            encoder
                .write_all(&chunk.raw_data)
                .map_err(|_| HonzoError::Truncated)?;
            let compressed = encoder.finish().map_err(|_| HonzoError::Truncated)?;
            let size_raw = chunk.raw_data.len() as u32;
            let size_compressed = compressed.len() as u32;
            (compressed, size_compressed, size_raw)
        }
        Compression::Zstd => {
            let compressed = zstd::encode_all(std::io::Cursor::new(&chunk.raw_data), 3)
                .map_err(|_| HonzoError::Truncated)?;
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
