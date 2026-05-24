extern crate honzo_chunks;
extern crate honzo_core;
extern crate honzo_io;

pub use honzo_chunks::data::math::{
    latex_to_mathml_bytes, render_math_bytes, validate_mathml_bytes,
};
pub use honzo_chunks::data::sidx::normalize_search_term;
pub use honzo_core::HonzoParser;
pub use honzo_core::MathType;
pub use honzo_io::{decompress, Compression, CoverType, HonzoBuilder, HonzoMeta, MarkupType};

#[diplomat::bridge]
pub mod ffi {
    use crate::{
        decompress, latex_to_mathml_bytes, normalize_search_term as normalize_search_term_impl,
        render_math_bytes, validate_mathml_bytes, Compression, CoverType, HonzoBuilder, HonzoMeta,
        HonzoParser, MarkupType, MathType,
    };
    use core::fmt::Write as _;

    #[repr(C)]
    pub enum HonzoErrorCode {
        Ok = 0,
        InvalidMagic = 1,
        ReaderVersionTooOld = 2,
        BufferTooShort = 3,
        CrcMismatch = 4,
        EncryptedChunk = 5,
        InvalidMathML = 6,
        Truncated = 7,
        Unknown = 255,
    }

    #[diplomat::opaque]
    pub struct HonzoHandle {
        buf: Vec<u8>,
        meta: Vec<u8>,
        chunks: Vec<Vec<u8>>,
        reader_version: u16,
    }

    impl HonzoHandle {
        pub fn parse(data: &[u8], reader_version: u16) -> Option<Box<HonzoHandle>> {
            let p = HonzoParser::new(data, reader_version).ok()?;
            let meta = p.meta_bytes().ok()?.to_vec();

            let entries: Vec<_> = p.toc_entries().collect();
            let mut chunks = Vec::with_capacity(entries.len());

            for entry in &entries {
                if entry.is_encrypted() {
                    chunks.push(Vec::new());
                    continue;
                }
                let raw = p.chunk_bytes(entry).ok()?;
                let decompressed = decompress(raw, entry.compression, entry.size_raw).ok()?;
                chunks.push(decompressed);
            }

            Some(Box::new(HonzoHandle {
                buf: data.to_vec(),
                meta,
                chunks,
                reader_version,
            }))
        }

        pub fn chunk_count(&self) -> u32 {
            self.chunks.len() as u32
        }

        pub fn layout_mode(&self) -> u8 {
            HonzoParser::new(&self.buf, self.reader_version)
                .map(|p| p.head().layout_mode() as u8)
                .unwrap_or(0)
        }

        pub fn has_drm(&self) -> bool {
            HonzoParser::new(&self.buf, self.reader_version)
                .map(|p| p.head().has_drm())
                .unwrap_or(false)
        }

        pub fn has_sidx(&self) -> bool {
            HonzoParser::new(&self.buf, self.reader_version)
                .map(|p| p.head().has_sidx())
                .unwrap_or(false)
        }

        #[allow(clippy::needless_lifetimes)]
        pub fn get_chunk<'a>(&'a self, index: u32) -> Option<&'a [u8]> {
            self.chunks.get(index as usize).map(|c| c.as_slice())
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

        pub fn add_chunk(
            &mut self,
            tag: &[u8],
            data: &[u8],
            compression: u8,
            content_type_kind: u8,
            content_type_value: u8,
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
            let markup = match content_type_value {
                0 => MarkupType::Markdown,
                1 => MarkupType::Html,
                _ => return false,
            };
            self.builder = Some(builder.add_chunk(
                tag_arr,
                data,
                compression,
                markup,
                CoverType::Front,
                None,
                None,
                None,
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

        pub fn add_math_chunk(&mut self, data: &[u8], math_type: u8, compression: u8) -> bool {
            self.add_chunk(b"MATH", data, compression, 2, math_type)
        }

        pub fn set_meta(&mut self, msgpack: &[u8]) -> bool {
            let b = match self.builder.take() {
                Some(b) => b,
                None => return false,
            };
            self.builder = Some(b.set_meta(msgpack));
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
}
