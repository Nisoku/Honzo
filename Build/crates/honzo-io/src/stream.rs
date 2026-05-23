use crate::compression::{decompress, verify_entry_crc32};
use honzo_core::{HonzoError, HonzoHead, PmapEntry, TocEntry};
use honzo_core::{MarkupType, MathType};
use std::io::{Read, Seek, SeekFrom};

pub struct HonzoStream<R: Read + Seek> {
    reader: R,
    head: HonzoHead,
    toc_buf: Vec<u8>,
    pmap: Vec<PmapEntry>,
    data_start: u64,
}

impl<R: Read + Seek> HonzoStream<R> {
    pub fn open(mut reader: R, reader_version: u16) -> Result<Self, HonzoError> {
        let mut magic = [0u8; 4];
        reader
            .read_exact(&mut magic)
            .map_err(|_| HonzoError::BufferTooShort)?;
        if &magic != b"HONO" {
            return Err(HonzoError::InvalidMagic);
        }

        let version_major = read_u8(&mut reader)?;
        let version_minor = read_u8(&mut reader)?;
        let min_reader_version = read_u16(&mut reader)?;
        if reader_version < min_reader_version {
            return Err(HonzoError::ReaderVersionTooOld {
                required: min_reader_version,
                have: reader_version,
            });
        }
        let flags = read_u32(&mut reader)?;
        let chunk_count = read_u32(&mut reader)?;
        let toc_size = read_u64(&mut reader)?;
        let data_size = read_u64(&mut reader)?;
        let extra_size = read_u64(&mut reader)?;
        let meta_size = read_u64(&mut reader)?;
        let _reserved = read_u32(&mut reader)?;

        let head = HonzoHead {
            version_major,
            version_minor,
            min_reader_version,
            flags,
            chunk_count,
            toc_size,
            data_size,
            extra_size,
            meta_size,
        };

        let mut toc_buf = vec![0u8; toc_size as usize];
        reader
            .read_exact(&mut toc_buf)
            .map_err(|_| HonzoError::BufferTooShort)?;
        let (_, pmap) = parse_toc(&toc_buf, chunk_count)?;

        let data_start = 4 + 48 + toc_size;

        Ok(Self {
            reader,
            head,
            toc_buf,
            pmap,
            data_start,
        })
    }

    pub fn head(&self) -> &HonzoHead {
        &self.head
    }

    pub fn toc(&self) -> Vec<TocEntry<'_>> {
        parse_toc(&self.toc_buf, self.head.chunk_count)
            .map(|(toc, _)| toc)
            .unwrap_or_default()
    }

    pub fn toc_owned(&self) -> Vec<TocEntry<'static>> {
        self.toc()
            .into_iter()
            .map(|entry| {
                let alt_text = entry
                    .alt_text
                    .map(|s| &*Box::leak(s.to_string().into_boxed_str()));
                let font_license_url = entry
                    .font_license_url
                    .map(|s| &*Box::leak(s.to_string().into_boxed_str()));
                TocEntry {
                    chunk_type: entry.chunk_type,
                    chunk_id: entry.chunk_id,
                    offset: entry.offset,
                    size_compressed: entry.size_compressed,
                    size_raw: entry.size_raw,
                    compression: entry.compression,
                    content_type_kind: entry.content_type_kind,
                    content_type_value: entry.content_type_value,
                    cover_type: entry.cover_type,
                    flags: entry.flags,
                    crc32: entry.crc32,
                    alt_text,
                    font_embedding: entry.font_embedding,
                    font_license_url,
                }
            })
            .collect()
    }

    pub fn pmap(&self) -> &[PmapEntry] {
        &self.pmap
    }

    pub fn read_chunk(&mut self, entry: &TocEntry) -> Result<Vec<u8>, HonzoError> {
        if entry.is_encrypted() {
            return Err(HonzoError::EncryptedChunk {
                chunk_id: entry.chunk_id,
            });
        }
        let start = self.data_start + entry.offset;
        self.reader
            .seek(SeekFrom::Start(start))
            .map_err(|_| HonzoError::Truncated)?;
        let mut buf = vec![0u8; entry.size_compressed as usize];
        self.reader
            .read_exact(&mut buf)
            .map_err(|_| HonzoError::Truncated)?;
        let decompressed = decompress(&buf, entry.compression, entry.size_raw)?;
        verify_entry_crc32(entry, &decompressed)?;
        Ok(decompressed)
    }

    pub fn chapters(&mut self) -> ChapterIter<'_, R> {
        let toc = self
            .toc()
            .into_iter()
            .filter(|entry| entry.chunk_type == *b"CHAP" || entry.chunk_type == *b"NOTE")
            .map(|entry| {
                let alt_text = entry
                    .alt_text
                    .map(|s| &*Box::leak(s.to_string().into_boxed_str()));
                let font_license_url = entry
                    .font_license_url
                    .map(|s| &*Box::leak(s.to_string().into_boxed_str()));
                TocEntry {
                    chunk_type: entry.chunk_type,
                    chunk_id: entry.chunk_id,
                    offset: entry.offset,
                    size_compressed: entry.size_compressed,
                    size_raw: entry.size_raw,
                    compression: entry.compression,
                    content_type_kind: entry.content_type_kind,
                    content_type_value: entry.content_type_value,
                    cover_type: entry.cover_type,
                    flags: entry.flags,
                    crc32: entry.crc32,
                    alt_text,
                    font_embedding: entry.font_embedding,
                    font_license_url,
                }
            })
            .collect();
        ChapterIter {
            stream: self,
            toc,
            index: 0,
        }
    }

    pub fn meta_bytes(&mut self) -> Result<Vec<u8>, HonzoError> {
        let start = self.data_start + self.head.data_size + self.head.extra_size;
        self.reader
            .seek(SeekFrom::Start(start))
            .map_err(|_| HonzoError::Truncated)?;
        let mut buf = vec![0u8; self.head.meta_size as usize];
        self.reader
            .read_exact(&mut buf)
            .map_err(|_| HonzoError::Truncated)?;
        Ok(buf)
    }

    pub fn extra_bytes(&mut self) -> Result<Vec<u8>, HonzoError> {
        let start = self.data_start + self.head.data_size;
        self.reader
            .seek(SeekFrom::Start(start))
            .map_err(|_| HonzoError::Truncated)?;
        let mut buf = vec![0u8; self.head.extra_size as usize];
        self.reader
            .read_exact(&mut buf)
            .map_err(|_| HonzoError::Truncated)?;
        Ok(buf)
    }
}

pub struct ChapterIter<'a, R: Read + Seek> {
    stream: &'a mut HonzoStream<R>,
    toc: Vec<TocEntry<'static>>,
    index: usize,
}

impl<'a, R: Read + Seek> Iterator for ChapterIter<'a, R> {
    type Item = Result<Vec<u8>, HonzoError>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.toc.len() {
            let entry = self.toc[self.index];
            self.index += 1;
            if entry.chunk_type == *b"CHAP" || entry.chunk_type == *b"NOTE" {
                if entry.is_encrypted() {
                    return Some(Err(HonzoError::EncryptedChunk {
                        chunk_id: entry.chunk_id,
                    }));
                }
                return Some(self.stream.read_chunk(&entry));
            }
        }
        None
    }
}

fn parse_toc<'a>(
    buf: &'a [u8],
    chunk_count: u32,
) -> Result<(Vec<TocEntry<'a>>, Vec<PmapEntry>), HonzoError> {
    let mut cursor = 0usize;
    let entries = read_u32_bytes(buf, &mut cursor)?;
    if entries != chunk_count {
        return Err(HonzoError::Truncated);
    }

    let mut toc = Vec::with_capacity(entries as usize);
    for _ in 0..entries {
        let chunk_type = read_tag_bytes(buf, &mut cursor)?;
        if !is_known_chunk(&chunk_type) {
            return Err(HonzoError::InvalidChunkType);
        }
        let chunk_id = read_u32_bytes(buf, &mut cursor)?;
        let offset = read_u64_bytes(buf, &mut cursor)?;
        let size_compressed = read_u32_bytes(buf, &mut cursor)?;
        let size_raw = read_u32_bytes(buf, &mut cursor)?;
        let compression = read_u8_bytes(buf, &mut cursor)?;
        let content_type_kind = read_u8_bytes(buf, &mut cursor)?;
        let content_type_value = read_u8_bytes(buf, &mut cursor)?;
        let cover_type = read_u8_bytes(buf, &mut cursor)?;
        let flags = read_u8_bytes(buf, &mut cursor)?;
        let crc32 = read_u32_bytes(buf, &mut cursor)?;
        let alt_len = read_u16_bytes(buf, &mut cursor)? as usize;
        let alt_text = if alt_len > 0 {
            let slice = read_bytes(buf, &mut cursor, alt_len)?;
            Some(core::str::from_utf8(slice).map_err(|_| HonzoError::Truncated)?)
        } else {
            None
        };

        let mut font_embedding = None;
        let mut font_license_url = None;
        if &chunk_type == b"FONT" {
            let embed = read_u8_bytes(buf, &mut cursor)?;
            font_embedding = Some(match embed {
                0 => honzo_core::FontEmbedding::Allowed,
                1 => honzo_core::FontEmbedding::PrintOnly,
                2 => honzo_core::FontEmbedding::NoModify,
                3 => honzo_core::FontEmbedding::NoEmbed,
                other => return Err(HonzoError::UnknownFontEmbedding(other)),
            });
            let url_len = read_u16_bytes(buf, &mut cursor)? as usize;
            if url_len > 0 {
                let slice = read_bytes(buf, &mut cursor, url_len)?;
                font_license_url =
                    Some(core::str::from_utf8(slice).map_err(|_| HonzoError::Truncated)?);
            }
        }

        // validate content_type depending on chunk
        if &chunk_type == b"CHAP" || &chunk_type == b"NOTE" {
            if content_type_kind != 1 {
                return Err(HonzoError::UnknownMarkupType(content_type_kind));
            }
            MarkupType::from_u8(content_type_value)?;
        } else if &chunk_type == b"MATH" {
            if content_type_kind != 2 {
                return Err(HonzoError::UnknownMathType(content_type_kind));
            }
            MathType::from_u8(content_type_value)?;
        } else {
            // for other chunk types we expect kind==1 and value==0
            if content_type_kind != 1 || content_type_value != 0 {
                return Err(HonzoError::Truncated);
            }
        }

        toc.push(TocEntry {
            chunk_type,
            chunk_id,
            offset,
            size_compressed,
            size_raw,
            compression: honzo_core::Compression::from_u8(compression)?,
            content_type_kind,
            content_type_value,
            cover_type: honzo_core::CoverType::from_u8(cover_type)?,
            flags,
            crc32,
            alt_text,
            font_embedding,
            font_license_url,
        });
    }

    let pmap_count = read_u32_bytes(buf, &mut cursor)?;
    let mut pmap = Vec::with_capacity(pmap_count as usize);
    for _ in 0..pmap_count {
        let print_page = read_u32_bytes(buf, &mut cursor)?;
        let chunk_id = read_u32_bytes(buf, &mut cursor)?;
        let byte_offset = read_u32_bytes(buf, &mut cursor)?;
        pmap.push(PmapEntry {
            print_page,
            chunk_id,
            byte_offset,
        });
    }

    Ok((toc, pmap))
}

fn is_known_chunk(tag: &[u8; 4]) -> bool {
    matches!(
        tag,
        b"CHAP" | b"IMG_" | b"CSS_" | b"FONT" | b"COVR" | b"COVT" | b"NOTE" | b"SIDX" | b"MATH"
    )
}

fn read_u8(reader: &mut impl Read) -> Result<u8, HonzoError> {
    let mut buf = [0u8; 1];
    reader
        .read_exact(&mut buf)
        .map_err(|_| HonzoError::BufferTooShort)?;
    Ok(buf[0])
}

fn read_u16(reader: &mut impl Read) -> Result<u16, HonzoError> {
    let mut buf = [0u8; 2];
    reader
        .read_exact(&mut buf)
        .map_err(|_| HonzoError::BufferTooShort)?;
    Ok(u16::from_le_bytes(buf))
}

fn read_u32(reader: &mut impl Read) -> Result<u32, HonzoError> {
    let mut buf = [0u8; 4];
    reader
        .read_exact(&mut buf)
        .map_err(|_| HonzoError::BufferTooShort)?;
    Ok(u32::from_le_bytes(buf))
}

fn read_u64(reader: &mut impl Read) -> Result<u64, HonzoError> {
    let mut buf = [0u8; 8];
    reader
        .read_exact(&mut buf)
        .map_err(|_| HonzoError::BufferTooShort)?;
    Ok(u64::from_le_bytes(buf))
}

fn read_bytes<'a>(buf: &'a [u8], cursor: &mut usize, len: usize) -> Result<&'a [u8], HonzoError> {
    let end = *cursor + len;
    if end > buf.len() {
        return Err(HonzoError::Truncated);
    }
    let out = &buf[*cursor..end];
    *cursor = end;
    Ok(out)
}

fn read_tag_bytes(buf: &[u8], cursor: &mut usize) -> Result<[u8; 4], HonzoError> {
    let bytes = read_bytes(buf, cursor, 4)?;
    let mut tag = [0u8; 4];
    tag.copy_from_slice(bytes);
    Ok(tag)
}

fn read_u8_bytes(buf: &[u8], cursor: &mut usize) -> Result<u8, HonzoError> {
    let bytes = read_bytes(buf, cursor, 1)?;
    Ok(bytes[0])
}

fn read_u16_bytes(buf: &[u8], cursor: &mut usize) -> Result<u16, HonzoError> {
    let bytes = read_bytes(buf, cursor, 2)?;
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u32_bytes(buf: &[u8], cursor: &mut usize) -> Result<u32, HonzoError> {
    let bytes = read_bytes(buf, cursor, 4)?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn read_u64_bytes(buf: &[u8], cursor: &mut usize) -> Result<u64, HonzoError> {
    let bytes = read_bytes(buf, cursor, 8)?;
    Ok(u64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ]))
}
