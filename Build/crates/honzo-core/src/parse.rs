use crate::types::{
    Compression, CoverType, FontEmbedding, HonzoHead, MarkupType, PmapEntry, TocEntry,
};
use crate::HonzoError;

const MAGIC: &[u8; 4] = b"HONO";
const HEAD_SIZE: usize = 48;

pub struct HonzoParser<'buf> {
    buf: &'buf [u8],
    head: HonzoHead,
    toc_offset: usize,
    toc_entries: u32,
    data_offset: usize,
    extra_offset: usize,
    meta_offset: usize,
    pmap_offset: usize,
    pmap_entries: u32,
}

impl<'buf> HonzoParser<'buf> {
    pub fn new(buf: &'buf [u8], reader_version: u16) -> Result<Self, HonzoError> {
        if buf.len() < 4 + HEAD_SIZE {
            return Err(HonzoError::BufferTooShort);
        }
        if &buf[..4] != MAGIC {
            return Err(HonzoError::InvalidMagic);
        }

        let mut cursor = 4;
        let version_major = read_u8(buf, &mut cursor)?;
        let version_minor = read_u8(buf, &mut cursor)?;
        let min_reader_version = read_u16(buf, &mut cursor)?;
        let flags = read_u32(buf, &mut cursor)?;
        let chunk_count = read_u32(buf, &mut cursor)?;
        let toc_size = read_u64(buf, &mut cursor)?;
        let data_size = read_u64(buf, &mut cursor)?;
        let extra_size = read_u64(buf, &mut cursor)?;
        let meta_size = read_u64(buf, &mut cursor)?;
        let _reserved = read_u32(buf, &mut cursor)?;

        if reader_version < min_reader_version {
            return Err(HonzoError::ReaderVersionTooOld {
                required: min_reader_version,
                have: reader_version,
            });
        }

        let toc_offset = 4 + HEAD_SIZE;
        let data_offset = toc_offset + toc_size as usize;
        let extra_offset = data_offset + data_size as usize;
        let meta_offset = extra_offset + extra_size as usize;

        if buf.len() < meta_offset + meta_size as usize {
            return Err(HonzoError::BufferTooShort);
        }

        let toc_end = toc_offset + toc_size as usize;
        if toc_end > buf.len() {
            return Err(HonzoError::BufferTooShort);
        }

        let (toc_entries, pmap_offset, pmap_entries) =
            validate_toc(buf, toc_offset, toc_end)?;

        Ok(Self {
            buf,
            head: HonzoHead {
                version_major,
                version_minor,
                min_reader_version,
                flags,
                chunk_count,
                toc_size,
                data_size,
                extra_size,
                meta_size,
            },
            toc_offset,
            toc_entries,
            data_offset,
            extra_offset,
            meta_offset,
            pmap_offset,
            pmap_entries,
        })
    }

    pub fn head(&self) -> &HonzoHead {
        &self.head
    }

    pub fn toc_entries(&self) -> TocEntryIter<'buf> {
        TocEntryIter::new(self.buf, self.toc_offset, self.toc_entries)
    }

    pub fn pmap_entries(&self) -> PmapEntryIter<'buf> {
        PmapEntryIter::new(self.buf, self.pmap_offset, self.pmap_entries)
    }

    pub fn chunk_bytes(&self, entry: &TocEntry) -> Result<&'buf [u8], HonzoError> {
        let start = self.data_offset + entry.offset as usize;
        let end = start + entry.size_compressed as usize;
        if end > self.data_offset + self.head.data_size as usize {
            return Err(HonzoError::Truncated);
        }
        if end > self.buf.len() {
            return Err(HonzoError::BufferTooShort);
        }
        Ok(&self.buf[start..end])
    }

    pub fn meta_bytes(&self) -> Result<&'buf [u8], HonzoError> {
        let start = self.meta_offset;
        let end = start + self.head.meta_size as usize;
        if end > self.buf.len() {
            return Err(HonzoError::BufferTooShort);
        }
        Ok(&self.buf[start..end])
    }

    pub fn extra_bytes(&self) -> Result<&'buf [u8], HonzoError> {
        let start = self.extra_offset;
        let end = start + self.head.extra_size as usize;
        if end > self.buf.len() {
            return Err(HonzoError::BufferTooShort);
        }
        Ok(&self.buf[start..end])
    }

    pub fn find_chunk(&self, tag: &[u8; 4]) -> Option<TocEntry<'buf>> {
        self.toc_entries().find(|entry| &entry.chunk_type == tag)
    }

    pub fn find_chunk_by_id(&self, id: u32) -> Option<TocEntry<'buf>> {
        self.toc_entries().find(|entry| entry.chunk_id == id)
    }
}

pub struct TocEntryIter<'buf> {
    buf: &'buf [u8],
    cursor: usize,
    remaining: u32,
}

impl<'buf> TocEntryIter<'buf> {
    fn new(buf: &'buf [u8], toc_offset: usize, chunk_count: u32) -> Self {
        let mut cursor = toc_offset;
        let _ = read_u32(buf, &mut cursor).ok();
        Self {
            buf,
            cursor,
            remaining: chunk_count,
        }
    }
}

impl<'buf> Iterator for TocEntryIter<'buf> {
    type Item = TocEntry<'buf>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let start = self.cursor;
        let mut cursor = self.cursor;
        let chunk_type = read_tag(self.buf, &mut cursor).ok()?;
        let chunk_id = read_u32(self.buf, &mut cursor).ok()?;
        let offset = read_u64(self.buf, &mut cursor).ok()?;
        let size_compressed = read_u32(self.buf, &mut cursor).ok()?;
        let size_raw = read_u32(self.buf, &mut cursor).ok()?;
        let compression = read_u8(self.buf, &mut cursor).ok()?;
        let markup_type = read_u8(self.buf, &mut cursor).ok()?;
        let cover_type = read_u8(self.buf, &mut cursor).ok()?;
        let flags = read_u8(self.buf, &mut cursor).ok()?;
        let crc32 = read_u32(self.buf, &mut cursor).ok()?;
        let alt_text_len = read_u16(self.buf, &mut cursor).ok()? as usize;

        let alt_text = if alt_text_len > 0 {
            let bytes = read_bytes(self.buf, &mut cursor, alt_text_len).ok()?;
            core::str::from_utf8(bytes).ok()
        } else {
            None
        };

        let mut font_embedding = None;
        let mut font_license_url = None;

        if &chunk_type == b"FONT" {
            let embedding = read_u8(self.buf, &mut cursor).ok()?;
            font_embedding = FontEmbedding::from_u8(embedding).ok();
            let url_len = read_u16(self.buf, &mut cursor).ok()? as usize;
            if url_len > 0 {
                let bytes = read_bytes(self.buf, &mut cursor, url_len).ok()?;
                font_license_url = core::str::from_utf8(bytes).ok();
            }
        }

        self.cursor = cursor;
        self.remaining -= 1;

        let entry = TocEntry {
            chunk_type,
            chunk_id,
            offset,
            size_compressed,
            size_raw,
            compression: Compression::from_u8(compression).ok()?,
            markup_type: MarkupType::from_u8(markup_type).ok()?,
            cover_type: CoverType::from_u8(cover_type).ok()?,
            flags,
            crc32,
            alt_text,
            font_embedding,
            font_license_url,
        };

        let min_len = cursor - start;
        if min_len == 0 {
            return None;
        }
        Some(entry)
    }
}

pub struct PmapEntryIter<'buf> {
    buf: &'buf [u8],
    cursor: usize,
    remaining: u32,
}

impl<'buf> PmapEntryIter<'buf> {
    fn new(buf: &'buf [u8], pmap_offset: usize, pmap_count: u32) -> Self {
        Self {
            buf,
            cursor: pmap_offset,
            remaining: pmap_count,
        }
    }
}

impl<'buf> Iterator for PmapEntryIter<'buf> {
    type Item = PmapEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let mut cursor = self.cursor;
        let print_page = read_u32(self.buf, &mut cursor).ok()?;
        let chunk_id = read_u32(self.buf, &mut cursor).ok()?;
        let byte_offset = read_u32(self.buf, &mut cursor).ok()?;
        self.cursor = cursor;
        self.remaining -= 1;
        Some(PmapEntry {
            print_page,
            chunk_id,
            byte_offset,
        })
    }
}

fn is_known_chunk(tag: &[u8; 4]) -> bool {
    matches!(
        tag,
        b"CHAP" | b"IMG_" | b"CSS_" | b"FONT" | b"COVR" | b"COVT" | b"NOTE" | b"SIDX" | b"MATH"
    )
}

fn validate_toc(
    buf: &[u8],
    toc_offset: usize,
    toc_end: usize,
) -> Result<(u32, usize, u32), HonzoError> {
    let mut cursor = toc_offset;
    let num_entries = read_u32_limit(buf, &mut cursor, toc_end)?;
    for _ in 0..num_entries {
        let chunk_type = read_tag_limit(buf, &mut cursor, toc_end)?;
        if !is_known_chunk(&chunk_type) {
            return Err(HonzoError::InvalidChunkType);
        }
        let _ = read_u32_limit(buf, &mut cursor, toc_end)?;
        let _ = read_u64_limit(buf, &mut cursor, toc_end)?;
        let _ = read_u32_limit(buf, &mut cursor, toc_end)?;
        let _ = read_u32_limit(buf, &mut cursor, toc_end)?;
        let compression = read_u8_limit(buf, &mut cursor, toc_end)?;
        let markup_type = read_u8_limit(buf, &mut cursor, toc_end)?;
        let cover_type = read_u8_limit(buf, &mut cursor, toc_end)?;
        let _ = read_u8_limit(buf, &mut cursor, toc_end)?;
        let _ = read_u32_limit(buf, &mut cursor, toc_end)?;
        let alt_text_len = read_u16_limit(buf, &mut cursor, toc_end)? as usize;
        let _ = read_bytes_limit(buf, &mut cursor, alt_text_len, toc_end)?;

        Compression::from_u8(compression)?;
        MarkupType::from_u8(markup_type)?;
        CoverType::from_u8(cover_type)?;

        if &chunk_type == b"FONT" {
            let embedding = read_u8_limit(buf, &mut cursor, toc_end)?;
            FontEmbedding::from_u8(embedding)?;
            let url_len = read_u16_limit(buf, &mut cursor, toc_end)? as usize;
            let _ = read_bytes_limit(buf, &mut cursor, url_len, toc_end)?;
        }
    }

    let pmap_offset = cursor;
    let num_pmap_entries = read_u32_limit(buf, &mut cursor, toc_end)?;
    for _ in 0..num_pmap_entries {
        let _ = read_u32_limit(buf, &mut cursor, toc_end)?;
        let _ = read_u32_limit(buf, &mut cursor, toc_end)?;
        let _ = read_u32_limit(buf, &mut cursor, toc_end)?;
    }

    Ok((num_entries, pmap_offset + 4, num_pmap_entries))
}

fn read_bytes<'a>(buf: &'a [u8], cursor: &mut usize, len: usize) -> Result<&'a [u8], HonzoError> {
    let end = *cursor + len;
    if end > buf.len() {
        return Err(HonzoError::BufferTooShort);
    }
    let out = &buf[*cursor..end];
    *cursor = end;
    Ok(out)
}

fn read_bytes_limit<'a>(
    buf: &'a [u8],
    cursor: &mut usize,
    len: usize,
    limit: usize,
) -> Result<&'a [u8], HonzoError> {
    let end = *cursor + len;
    if end > limit {
        return Err(HonzoError::Truncated);
    }
    if end > buf.len() {
        return Err(HonzoError::BufferTooShort);
    }
    let out = &buf[*cursor..end];
    *cursor = end;
    Ok(out)
}

fn read_tag_limit(buf: &[u8], cursor: &mut usize, limit: usize) -> Result<[u8; 4], HonzoError> {
    let bytes = read_bytes_limit(buf, cursor, 4, limit)?;
    let mut tag = [0u8; 4];
    tag.copy_from_slice(bytes);
    Ok(tag)
}

fn read_tag(buf: &[u8], cursor: &mut usize) -> Result<[u8; 4], HonzoError> {
    let bytes = read_bytes(buf, cursor, 4)?;
    let mut tag = [0u8; 4];
    tag.copy_from_slice(bytes);
    Ok(tag)
}

fn read_u8(buf: &[u8], cursor: &mut usize) -> Result<u8, HonzoError> {
    let bytes = read_bytes(buf, cursor, 1)?;
    Ok(bytes[0])
}

fn read_u8_limit(buf: &[u8], cursor: &mut usize, limit: usize) -> Result<u8, HonzoError> {
    let bytes = read_bytes_limit(buf, cursor, 1, limit)?;
    Ok(bytes[0])
}

fn read_u16(buf: &[u8], cursor: &mut usize) -> Result<u16, HonzoError> {
    let bytes = read_bytes(buf, cursor, 2)?;
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u16_limit(buf: &[u8], cursor: &mut usize, limit: usize) -> Result<u16, HonzoError> {
    let bytes = read_bytes_limit(buf, cursor, 2, limit)?;
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u32(buf: &[u8], cursor: &mut usize) -> Result<u32, HonzoError> {
    let bytes = read_bytes(buf, cursor, 4)?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn read_u32_limit(buf: &[u8], cursor: &mut usize, limit: usize) -> Result<u32, HonzoError> {
    let bytes = read_bytes_limit(buf, cursor, 4, limit)?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn read_u64(buf: &[u8], cursor: &mut usize) -> Result<u64, HonzoError> {
    let bytes = read_bytes(buf, cursor, 8)?;
    Ok(u64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ]))
}

fn read_u64_limit(buf: &[u8], cursor: &mut usize, limit: usize) -> Result<u64, HonzoError> {
    let bytes = read_bytes_limit(buf, cursor, 8, limit)?;
    Ok(u64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ]))
}
