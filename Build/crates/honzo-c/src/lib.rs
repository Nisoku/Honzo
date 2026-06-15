extern crate honzo_chunks;
extern crate honzo_core;
extern crate honzo_io;

pub use honzo_chunks::data::css::validate_css_bytes;
pub use honzo_chunks::data::font::guess_font_format;
pub use honzo_chunks::data::math::{
    latex_to_mathml_bytes, render_math_bytes, validate_mathml_bytes,
};
pub use honzo_chunks::data::sidx::normalize_search_term;
pub use honzo_core::{FontEmbedding, HonzoError, HonzoParser, LayoutMode, MathType, PmapEntry};
pub use honzo_io::DrmConfig;
pub use honzo_io::{decompress, Compression, CoverType, HonzoBuilder, HonzoMeta, MarkupType};

struct CachedEntry {
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
    alt_text: Option<String>,
    font_embedding: Option<FontEmbedding>,
    font_license_url: Option<String>,
}

fn map_honzo_error(err: HonzoError) -> ffi::HonzoErrorCode {
    match err {
        HonzoError::InvalidMagic => ffi::HonzoErrorCode::InvalidMagic,
        HonzoError::ReaderVersionTooOld { .. } => ffi::HonzoErrorCode::ReaderVersionTooOld,
        HonzoError::BufferTooShort => ffi::HonzoErrorCode::BufferTooShort,
        HonzoError::CrcMismatch { .. } => ffi::HonzoErrorCode::CrcMismatch,
        HonzoError::EncryptedChunk { .. } => ffi::HonzoErrorCode::EncryptedChunk,
        HonzoError::InvalidMathML => ffi::HonzoErrorCode::InvalidMathML,
        HonzoError::Truncated => ffi::HonzoErrorCode::Truncated,
        HonzoError::InvalidCss => ffi::HonzoErrorCode::InvalidCss,
        HonzoError::InvalidSyncCue => ffi::HonzoErrorCode::InvalidSyncCue,
        _ => ffi::HonzoErrorCode::Unknown,
    }
}

#[diplomat::bridge]
pub mod ffi {
    use crate::{
        decompress, guess_font_format as guess_font_format_impl, latex_to_mathml_bytes,
        normalize_search_term as normalize_search_term_impl, render_math_bytes, validate_css_bytes,
        validate_mathml_bytes, CachedEntry, Compression, CoverType, DrmConfig, FontEmbedding,
        HonzoBuilder, HonzoMeta, HonzoParser, LayoutMode, MarkupType, MathType, PmapEntry,
    };
    use core::fmt::Write as _;
    use honzo_chunks::extra::{anno, sync};
    use honzo_io::HonzoStream;

    #[repr(C)]
    #[derive(Debug)]
    pub enum HonzoErrorCode {
        Ok = 0,
        InvalidMagic = 1,
        ReaderVersionTooOld = 2,
        BufferTooShort = 3,
        CrcMismatch = 4,
        EncryptedChunk = 5,
        InvalidMathML = 6,
        Truncated = 7,
        InvalidCss = 8,
        InvalidSyncCue = 9,
        FileNotFound = 10,
        Unknown = 255,
    }

    #[diplomat::opaque_mut]
    pub struct HonzoHandle {
        buf: Vec<u8>,
        meta: Vec<u8>,
        data_start: usize,
        toc_entries: Vec<TocEntryOwned>,
        chunk_cache: Vec<Option<Vec<u8>>>,
        reader_version: u16,
        cek: Option<[u8; 32]>,
    }

    struct TocEntryOwned {
        chunk_id: u32,
        offset: u64,
        size_compressed: u32,
        size_raw: u32,
        compression: u8,
        ctype_kind: u8,
        ctype_value: u8,
        cover_type: u8,
        flags: u8,
        crc32: u32,
    }

    impl HonzoHandle {
        pub fn parse(data: &[u8], reader_version: u16) -> Option<Box<HonzoHandle>> {
            Self::parse_inner(data, reader_version, &[])
        }

        /// Parse with an X25519 private key (32 bytes) for DRM decryption.
        pub fn parse_with_private_key(
            data: &[u8],
            reader_version: u16,
            private_key: &[u8],
        ) -> Option<Box<HonzoHandle>> {
            Self::parse_inner(data, reader_version, private_key)
        }

        fn parse_inner(
            data: &[u8],
            reader_version: u16,
            private_key: &[u8],
        ) -> Option<Box<HonzoHandle>> {
            let p = HonzoParser::new(data, reader_version).ok()?;
            let meta = p.meta_bytes().ok()?.to_vec();
            let head = p.head();

            let data_start = (52 + head.toc_size) as usize;

            let toc_entries: Vec<_> = p
                .toc_entries()
                .map(|e| TocEntryOwned {
                    chunk_id: e.chunk_id,
                    offset: e.offset,
                    size_compressed: e.size_compressed,
                    size_raw: e.size_raw,
                    compression: e.compression as u8,
                    ctype_kind: e.content_type_kind,
                    ctype_value: e.content_type_value,
                    cover_type: e.cover_type as u8,
                    flags: e.flags,
                    crc32: e.crc32,
                })
                .collect();

            let chunk_count = toc_entries.len();
            let chunk_cache = (0..chunk_count).map(|_| None).collect();

            // Parse DRM envelope if DRM flag is set and a private key was provided
            let cek = if head.has_drm() && !private_key.is_empty() {
                let extra = p.extra_bytes().ok()?;
                let entries = honzo_io::parse_extra(extra).ok()?;
                let entry = honzo_io::find_extra(&entries, honzo_chunks::extra::drm::NAMESPACE)?;
                let envelope = honzo_chunks::extra::drm::parse_drm(&entry.body).ok()?;
                honzo_io::crypto::unwrap_cek(&envelope.key_envelope, private_key).ok()
            } else {
                None
            };

            Some(Box::new(HonzoHandle {
                buf: data.to_vec(),
                meta,
                data_start,
                toc_entries,
                chunk_cache,
                reader_version,
                cek,
            }))
        }

        pub fn chunk_count(&self) -> u32 {
            self.toc_entries.len() as u32
        }

        pub fn version_major(&self) -> u8 {
            self.buf.get(4).copied().unwrap_or(0)
        }

        pub fn version_minor(&self) -> u8 {
            self.buf.get(5).copied().unwrap_or(0)
        }

        pub fn min_reader_version(&self) -> u16 {
            read_le_u16(&self.buf, 6).unwrap_or(0)
        }

        pub fn flags(&self) -> u32 {
            read_le_u32(&self.buf, 8).unwrap_or(0)
        }

        pub fn toc_size(&self) -> u64 {
            read_le_u64(&self.buf, 16).unwrap_or(0)
        }

        pub fn data_size(&self) -> u64 {
            read_le_u64(&self.buf, 24).unwrap_or(0)
        }

        pub fn extra_size(&self) -> u64 {
            read_le_u64(&self.buf, 32).unwrap_or(0)
        }

        pub fn meta_size(&self) -> u64 {
            read_le_u64(&self.buf, 40).unwrap_or(0)
        }

        pub fn layout_mode(&self) -> u8 {
            self.flags().wrapping_shr(2) as u8 & 3
        }

        pub fn has_drm(&self) -> bool {
            self.flags() & 0x10 != 0
        }

        pub fn has_sidx(&self) -> bool {
            self.flags() & 0x20 != 0
        }

        pub fn has_annotations(&self) -> bool {
            self.flags() & 0x40 != 0
        }

        pub fn has_sync(&self) -> bool {
            self.flags() & 0x80 != 0
        }

        #[allow(clippy::needless_lifetimes)]
        pub fn get_extra<'a>(&'a self) -> &'a [u8] {
            let data_size = self.data_size();
            let extra_size = self.extra_size();
            if extra_size == 0 {
                return &[];
            }
            let start = self.data_start + data_size as usize;
            let end = start + extra_size as usize;
            if end > self.buf.len() {
                return &[];
            }
            &self.buf[start..end]
        }

        #[allow(clippy::needless_lifetimes)]
        pub fn get_chunk<'a>(&'a mut self, index: u32) -> Option<&'a [u8]> {
            let entry = self.toc_entries.get(index as usize)?;

            // Check cache. take the slot temporarily to drop the borrow
            let cached = self.chunk_cache[index as usize].take();
            if let Some(data) = cached {
                self.chunk_cache[index as usize] = Some(data);
                return Some(self.chunk_cache[index as usize].as_ref().unwrap());
            }

            // Read raw bytes from buffer
            let start = self.data_start + entry.offset as usize;
            let end = start + entry.size_compressed as usize;
            if end > self.buf.len() {
                return None;
            }
            let raw = &self.buf[start..end];

            let comp = match entry.compression {
                0 => Compression::None,
                1 => Compression::Lz4,
                _ => return None,
            };

            let decompressed = if entry.flags & 0x01 != 0 {
                if let Some(ref cek) = self.cek {
                    let compressed = honzo_io::crypto::decrypt_chunk(raw, cek).ok()?;
                    decompress(&compressed, comp, entry.size_raw).ok()?
                } else {
                    return None;
                }
            } else {
                decompress(raw, comp, entry.size_raw).ok()?
            };

            self.chunk_cache[index as usize] = Some(decompressed);
            Some(self.chunk_cache[index as usize].as_ref().unwrap())
        }

        #[allow(clippy::needless_lifetimes)]
        pub fn get_meta<'a>(&'a self) -> &'a [u8] {
            &self.meta
        }

        pub fn get_meta_parsed(
            &self,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), HonzoErrorCode> {
            let meta: HonzoMeta =
                rmp_serde::from_slice(&self.meta).map_err(|_| HonzoErrorCode::Truncated)?;
            let json = serde_json::to_string(&meta).map_err(|_| HonzoErrorCode::Unknown)?;
            write
                .write_str(&json)
                .map_err(|_| HonzoErrorCode::Unknown)?;
            Ok(())
        }

        pub fn get_annotations(
            &self,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), HonzoErrorCode> {
            let parser = HonzoParser::new(&self.buf, self.reader_version)
                .map_err(|_| HonzoErrorCode::Unknown)?;
            let extra = parser
                .extra_bytes()
                .map_err(|_| HonzoErrorCode::Truncated)?;
            let entries = honzo_io::parse_extra(extra).map_err(|_| HonzoErrorCode::Truncated)?;
            let entry =
                honzo_io::find_extra(&entries, anno::NAMESPACE).ok_or(HonzoErrorCode::Truncated)?;
            let annotations =
                anno::parse_anno(&entry.body).map_err(|_| HonzoErrorCode::Truncated)?;
            let json = serde_json::to_string(&annotations).map_err(|_| HonzoErrorCode::Unknown)?;
            write
                .write_str(&json)
                .map_err(|_| HonzoErrorCode::Unknown)?;
            Ok(())
        }

        pub fn get_sync_cues(
            &self,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), HonzoErrorCode> {
            let parser = HonzoParser::new(&self.buf, self.reader_version)
                .map_err(|_| HonzoErrorCode::Unknown)?;
            let extra = parser
                .extra_bytes()
                .map_err(|_| HonzoErrorCode::Truncated)?;
            let entries = honzo_io::parse_extra(extra).map_err(|_| HonzoErrorCode::Truncated)?;
            let entry =
                honzo_io::find_extra(&entries, sync::NAMESPACE).ok_or(HonzoErrorCode::Truncated)?;
            let cues = sync::parse_sync(&entry.body).map_err(|_| HonzoErrorCode::Truncated)?;
            let json = serde_json::to_string(&cues).map_err(|_| HonzoErrorCode::Unknown)?;
            write
                .write_str(&json)
                .map_err(|_| HonzoErrorCode::Unknown)?;
            Ok(())
        }

        pub fn get_pmap(
            &self,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), HonzoErrorCode> {
            #[derive(serde::Serialize)]
            struct PmapOut {
                print_page: u32,
                chunk_id: u32,
                byte_offset: u32,
            }
            let parser = HonzoParser::new(&self.buf, self.reader_version)
                .map_err(|_| HonzoErrorCode::Unknown)?;
            let entries: Vec<PmapOut> = parser
                .pmap_entries()
                .map(|e| PmapOut {
                    print_page: e.print_page,
                    chunk_id: e.chunk_id,
                    byte_offset: e.byte_offset,
                })
                .collect();
            let json = serde_json::to_string(&entries).map_err(|_| HonzoErrorCode::Unknown)?;
            write
                .write_str(&json)
                .map_err(|_| HonzoErrorCode::Unknown)?;
            Ok(())
        }

        pub fn get_toc(
            &self,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), HonzoErrorCode> {
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

            let parser = HonzoParser::new(&self.buf, self.reader_version)
                .map_err(|_| HonzoErrorCode::Unknown)?;
            let entries: Vec<TocOut> = parser
                .toc_entries()
                .map(|entry| TocOut {
                    chunk_type: std::str::from_utf8(&entry.chunk_type)
                        .unwrap_or("????")
                        .to_string(),
                    chunk_id: entry.chunk_id,
                    offset: entry.offset,
                    size_compressed: entry.size_compressed,
                    size_raw: entry.size_raw,
                    compression: entry.compression as u8,
                    content_type_kind: entry.content_type_kind,
                    content_type_value: entry.content_type_value,
                    cover_type: entry.cover_type as u8,
                    flags: entry.flags,
                    crc32: entry.crc32,
                    alt_text: entry.alt_text.map(|s| s.to_string()),
                    font_embedding: entry.font_embedding.map(|e| e as u8),
                    font_license_url: entry.font_license_url.map(|s| s.to_string()),
                })
                .collect();
            let json = serde_json::to_string(&entries).map_err(|_| HonzoErrorCode::Unknown)?;
            write
                .write_str(&json)
                .map_err(|_| HonzoErrorCode::Unknown)?;
            Ok(())
        }
    }

    #[diplomat::opaque_mut]
    pub struct HonzoFileReader {
        stream: HonzoStream<std::fs::File>,
        chunk_buf: Vec<u8>,
        toc_cache: Vec<CachedEntry>,
    }

    impl HonzoFileReader {
        fn cache_toc(stream: &mut HonzoStream<std::fs::File>) -> Vec<CachedEntry> {
            let cached: Vec<CachedEntry> = stream
                .toc()
                .into_iter()
                .map(|e| CachedEntry {
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
            stream.drop_toc_buf();
            cached
        }

        pub fn open(path: &str, reader_version: u16) -> Result<Box<Self>, HonzoErrorCode> {
            let file = std::fs::File::open(path).map_err(|_| HonzoErrorCode::FileNotFound)?;
            let mut stream = HonzoStream::open(file, reader_version).map_err(crate::map_honzo_error)?;
            let toc_cache = Self::cache_toc(&mut stream);
            Ok(Box::new(HonzoFileReader {
                stream,
                chunk_buf: Vec::new(),
                toc_cache,
            }))
        }

        pub fn open_with_private_key(
            path: &str,
            reader_version: u16,
            private_key: &[u8],
        ) -> Result<Box<Self>, HonzoErrorCode> {
            let file = std::fs::File::open(path).map_err(|_| HonzoErrorCode::FileNotFound)?;
            let mut stream = HonzoStream::open_with_private_key(file, reader_version, private_key)
                .map_err(crate::map_honzo_error)?;
            let toc_cache = Self::cache_toc(&mut stream);
            Ok(Box::new(HonzoFileReader {
                stream,
                chunk_buf: Vec::new(),
                toc_cache,
            }))
        }

        pub fn chunk_count(&self) -> u32 {
            self.stream.head().chunk_count
        }

        pub fn get_chunk_type(&self, index: u32) -> u32 {
            self.toc_cache
                .get(index as usize)
                .map(|e| u32::from_le_bytes(e.chunk_type))
                .unwrap_or(0)
        }

        pub fn get_chunk_content_type_kind(&self, index: u32) -> u8 {
            self.toc_cache
                .get(index as usize)
                .map(|e| e.content_type_kind)
                .unwrap_or(0)
        }

        pub fn get_chunk_content_type_value(&self, index: u32) -> u8 {
            self.toc_cache
                .get(index as usize)
                .map(|e| e.content_type_value)
                .unwrap_or(0)
        }

        #[allow(clippy::needless_lifetimes)]
        /// Returns the decompressed data for chunk `index`.
        ///
        /// The returned slice is backed by an internal buffer and is valid only
        /// until the next call to `get_chunk` on the same reader. C/C++ callers
        /// must copy the data if they need it to outlive a subsequent call.
        pub fn get_chunk<'a>(&'a mut self, index: u32) -> Option<&'a [u8]> {
            let cached = self.toc_cache.get(index as usize)?;
            let entry = honzo_core::TocEntry {
                chunk_type: cached.chunk_type,
                chunk_id: cached.chunk_id,
                offset: cached.offset,
                size_compressed: cached.size_compressed,
                size_raw: cached.size_raw,
                compression: cached.compression,
                content_type_kind: cached.content_type_kind,
                content_type_value: cached.content_type_value,
                cover_type: cached.cover_type,
                flags: cached.flags,
                crc32: cached.crc32,
                alt_text: cached.alt_text.as_deref(),
                font_embedding: cached.font_embedding,
                font_license_url: cached.font_license_url.as_deref(),
            };

            let data = self.stream.read_chunk(&entry).ok()?;
            self.chunk_buf = data;
            Some(&self.chunk_buf)
        }

        pub fn get_meta(
            &mut self,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) -> Result<(), HonzoErrorCode> {
            let meta = self
                .stream
                .meta_bytes()
                .map_err(|_| HonzoErrorCode::Truncated)?;
            let meta: HonzoMeta =
                rmp_serde::from_slice(&meta).map_err(|_| HonzoErrorCode::Truncated)?;
            let json = serde_json::to_string(&meta).map_err(|_| HonzoErrorCode::Unknown)?;
            write
                .write_str(&json)
                .map_err(|_| HonzoErrorCode::Unknown)?;
            Ok(())
        }
    }

    #[diplomat::opaque_mut]
    pub struct HonzoBuilderHandle {
        builder: Option<HonzoBuilder>,
        result: Vec<u8>,
    }

    impl HonzoBuilderHandle {
        pub fn new() -> Box<HonzoBuilderHandle> {
            Box::new(HonzoBuilderHandle {
                builder: Some(HonzoBuilder::new()),
                result: Vec::new(),
            })
        }

        #[allow(clippy::too_many_arguments)]
        pub fn add_chunk(
            &mut self,
            tag: &[u8],
            data: &[u8],
            compression: u8,
            content_type_kind: u8,
            content_type_value: u8,
            cover_type: u8,
            alt_text: &str,
            font_embedding: i32,
            font_license_url: &str,
        ) -> bool {
            if tag.len() != 4 {
                return false;
            }
            let mut tag_arr = [0u8; 4];
            tag_arr.copy_from_slice(tag);
            let compression = match compression {
                0 => Compression::None,
                1 => Compression::Lz4,
                _ => return false,
            };
            let builder = match self.builder.as_mut() {
                Some(b) => std::mem::take(b),
                None => return false,
            };
            if &tag_arr == b"MATH" {
                if content_type_kind != 2 {
                    return false;
                }
                let math = match content_type_value {
                    0 => MathType::MathML,
                    1 => MathType::LaTeX,
                    _ => return false,
                };
                self.builder = Some(builder.add_math_chunk(data, math, compression));
                return true;
            }
            if content_type_kind != 1 {
                return false;
            }
            let markup = match &tag_arr {
                b"CHAP" | b"NOTE" => match content_type_value {
                    0 => MarkupType::Markdown,
                    1 => MarkupType::Html,
                    _ => return false,
                },
                _ => {
                    if content_type_value != 0 {
                        return false;
                    }
                    MarkupType::Markdown
                }
            };
            let cover = match cover_type {
                0 => CoverType::Front,
                1 => CoverType::Back,
                2 => CoverType::FullSpread,
                _ => CoverType::Front,
            };
            let alt = if alt_text.is_empty() {
                None
            } else {
                Some(alt_text)
            };
            let embedding = match font_embedding {
                0 => Some(FontEmbedding::Allowed),
                1 => Some(FontEmbedding::PrintOnly),
                2 => Some(FontEmbedding::NoModify),
                3 => Some(FontEmbedding::NoEmbed),
                _ => None,
            };
            let license_url = if font_license_url.is_empty() {
                None
            } else {
                Some(font_license_url)
            };
            self.builder = Some(builder.add_chunk(
                tag_arr,
                data,
                compression,
                markup,
                cover,
                alt,
                embedding,
                license_url,
            ));
            true
        }

        pub fn set_language(&mut self, lang: &str) -> bool {
            let b = match self.builder.as_mut() {
                Some(b) => std::mem::take(b),
                None => return false,
            };
            self.builder = Some(b.set_language(lang));
            true
        }

        pub fn set_auto_sidx(&mut self, enable: bool) -> bool {
            let b = match self.builder.as_mut() {
                Some(b) => std::mem::take(b),
                None => return false,
            };
            self.builder = Some(b.set_auto_sidx(enable));
            true
        }

        pub fn set_auto_covt(&mut self, enable: bool) -> bool {
            let b = match self.builder.as_mut() {
                Some(b) => std::mem::take(b),
                None => return false,
            };
            self.builder = Some(b.set_auto_covt(enable));
            true
        }

        pub fn set_layout(&mut self, layout: u8) -> bool {
            let l = match layout {
                0 => LayoutMode::Reflowable,
                1 => LayoutMode::Fixed,
                2 => LayoutMode::Scroll,
                _ => return false,
            };
            let b = match self.builder.as_mut() {
                Some(b) => std::mem::take(b),
                None => return false,
            };
            self.builder = Some(b.set_layout(l));
            true
        }

        pub fn set_flags(&mut self, flags: u32) -> bool {
            let b = match self.builder.as_mut() {
                Some(b) => std::mem::take(b),
                None => return false,
            };
            self.builder = Some(b.set_flags(flags));
            true
        }

        pub fn set_min_reader_version(&mut self, version: u16) -> bool {
            let b = match self.builder.as_mut() {
                Some(b) => std::mem::take(b),
                None => return false,
            };
            self.builder = Some(b.set_min_reader_version(version));
            true
        }

        pub fn add_pmap_entry(&mut self, print_page: u32, chunk_id: u32, byte_offset: u32) -> bool {
            let b = match self.builder.as_mut() {
                Some(b) => std::mem::take(b),
                None => return false,
            };
            self.builder = Some(b.add_pmap_entry(PmapEntry {
                print_page,
                chunk_id,
                byte_offset,
            }));
            true
        }

        pub fn add_math_chunk(&mut self, data: &[u8], math_type: u8, compression: u8) -> bool {
            self.add_chunk(b"MATH", data, compression, 2, math_type, 0, "", -1, "")
        }

        pub fn set_meta(&mut self, msgpack: &[u8]) -> bool {
            let b = match self.builder.take() {
                Some(b) => b,
                None => return false,
            };
            self.builder = Some(b.set_meta(msgpack));
            true
        }

        pub fn set_extra(&mut self, extra: &[u8]) -> bool {
            let b = match self.builder.take() {
                Some(b) => b,
                None => return false,
            };
            self.builder = Some(b.set_extra(extra));
            true
        }

        pub fn add_extra_entry(&mut self, tag: &[u8], namespace: &str, body: &[u8]) -> bool {
            if tag.len() != 4 {
                return false;
            }
            let mut tag_arr = [0u8; 4];
            tag_arr.copy_from_slice(tag);
            let b = match self.builder.take() {
                Some(b) => b,
                None => return false,
            };
            self.builder = Some(b.add_extra_entry(tag_arr, namespace, body));
            true
        }

        pub fn add_annotation(&mut self, body: &[u8]) -> bool {
            let annotations: Vec<anno::Annotation> = match rmp_serde::from_slice(body) {
                Ok(a) => a,
                Err(_) => return false,
            };
            let b = match self.builder.take() {
                Some(b) => b,
                None => return false,
            };
            self.builder = Some(b.add_annotation(&annotations));
            true
        }

        pub fn set_drm_config(
            &mut self,
            encrypt_chunk_ids: &[u32],
            recipient_public_key: &[u8],
            license_url: &str,
            expires_at: u64,
        ) -> bool {
            let b = match self.builder.take() {
                Some(b) => b,
                None => return false,
            };
            let config = DrmConfig {
                encrypt_chunk_ids: encrypt_chunk_ids.to_vec(),
                recipient_public_key: recipient_public_key.to_vec(),
                license_url: if license_url.is_empty() {
                    None
                } else {
                    Some(license_url.to_string())
                },
                expires_at: if expires_at == 0 {
                    None
                } else {
                    Some(expires_at)
                },
            };
            self.builder = Some(b.set_drm_config(config));
            true
        }

        pub fn add_sync_cue(&mut self, body: &[u8]) -> bool {
            let cues: Vec<sync::SyncCue> = match rmp_serde::from_slice(body) {
                Ok(c) => c,
                Err(_) => return false,
            };
            let b = match self.builder.take() {
                Some(b) => b,
                None => return false,
            };
            self.builder = Some(b.add_sync_cue(&cues));
            true
        }

        pub fn finalize(&mut self) -> bool {
            let b = match self.builder.take() {
                Some(b) => b,
                None => return false,
            };
            match b.finalize() {
                Ok(bytes) => {
                    self.result = bytes;
                    true
                }
                Err(_) => false,
            }
        }

        #[allow(clippy::needless_lifetimes)]
        pub fn get_result<'a>(&'a self) -> &'a [u8] {
            &self.result
        }
    }

    pub fn validate_mathml(bytes: &[u8]) -> bool {
        validate_mathml_bytes(bytes).is_ok()
    }

    pub fn latex_to_mathml(
        bytes: &[u8],
        write: &mut diplomat_runtime::DiplomatWrite,
    ) -> Result<(), HonzoErrorCode> {
        match latex_to_mathml_bytes(bytes) {
            Ok(v) => {
                write
                    .write_str(core::str::from_utf8(&v).unwrap())
                    .map_err(|_| HonzoErrorCode::Unknown)?;
                Ok(())
            }
            Err(code) => match code {
                6 => Err(HonzoErrorCode::InvalidMathML),
                7 => Err(HonzoErrorCode::Truncated),
                _ => Err(HonzoErrorCode::Unknown),
            },
        }
    }

    pub fn render_math(
        bytes: &[u8],
        math_type: u8,
        write: &mut diplomat_runtime::DiplomatWrite,
    ) -> Result<(), HonzoErrorCode> {
        match render_math_bytes(bytes, math_type) {
            Ok(v) => {
                write
                    .write_str(core::str::from_utf8(&v).unwrap())
                    .map_err(|_| HonzoErrorCode::Unknown)?;
                Ok(())
            }
            Err(code) => match code {
                6 => Err(HonzoErrorCode::InvalidMathML),
                7 => Err(HonzoErrorCode::Truncated),
                _ => Err(HonzoErrorCode::Unknown),
            },
        }
    }

    pub fn normalize_search_term(
        term: &str,
        lang: &str,
        write: &mut diplomat_runtime::DiplomatWrite,
    ) -> Result<(), HonzoErrorCode> {
        let normalized = normalize_search_term_impl(term, lang);
        write
            .write_str(&normalized)
            .map_err(|_| HonzoErrorCode::Unknown)?;
        Ok(())
    }

    pub fn validate_css(bytes: &[u8]) -> bool {
        validate_css_bytes(bytes).is_ok()
    }

    pub fn guess_font_format(
        bytes: &[u8],
        write: &mut diplomat_runtime::DiplomatWrite,
    ) -> Result<(), HonzoErrorCode> {
        match guess_font_format_impl(bytes) {
            Some(fmt) => {
                write.write_str(fmt).map_err(|_| HonzoErrorCode::Unknown)?;
                Ok(())
            }
            None => Err(HonzoErrorCode::Truncated),
        }
    }

    fn read_le_u16(buf: &[u8], offset: usize) -> Option<u16> {
        if offset + 2 > buf.len() {
            return None;
        }
        Some(u16::from_le_bytes([buf[offset], buf[offset + 1]]))
    }

    fn read_le_u32(buf: &[u8], offset: usize) -> Option<u32> {
        if offset + 4 > buf.len() {
            return None;
        }
        Some(u32::from_le_bytes([
            buf[offset],
            buf[offset + 1],
            buf[offset + 2],
            buf[offset + 3],
        ]))
    }

    fn read_le_u64(buf: &[u8], offset: usize) -> Option<u64> {
        if offset + 8 > buf.len() {
            return None;
        }
        Some(u64::from_le_bytes([
            buf[offset],
            buf[offset + 1],
            buf[offset + 2],
            buf[offset + 3],
            buf[offset + 4],
            buf[offset + 5],
            buf[offset + 6],
            buf[offset + 7],
        ]))
    }
}
