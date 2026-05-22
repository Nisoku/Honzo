extern crate honzo_chunks;
extern crate honzo_core;
extern crate honzo_io;

pub use honzo_chunks::data::math::{
    latex_to_mathml_bytes, render_math_bytes, validate_mathml_bytes,
};
pub use honzo_chunks::data::sidx::normalize_search_term;
pub use honzo_core::HonzoParser;
pub use honzo_core::MathType;
pub use honzo_io::{decompress, Compression, CoverType, HonzoBuilder, MarkupType};

#[diplomat::bridge]
pub mod ffi {
    use crate::{
        decompress, latex_to_mathml_bytes, normalize_search_term as normalize_search_term_impl,
        render_math_bytes, validate_mathml_bytes, Compression, CoverType, HonzoBuilder,
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
    }

    impl HonzoHandle {
        pub fn parse(data: &[u8], _reader_version: u16) -> Option<Box<HonzoHandle>> {
            let p = HonzoParser::new(data, 1).ok()?;
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
            }))
        }

        pub fn chunk_count(&self) -> u32 {
            self.chunks.len() as u32
        }

        pub fn layout_mode(&self) -> u8 {
            HonzoParser::new(&self.buf, 1)
                .map(|p| p.head().layout_mode() as u8)
                .unwrap_or(0)
        }

        pub fn has_drm(&self) -> bool {
            HonzoParser::new(&self.buf, 1)
                .map(|p| p.head().has_drm())
                .unwrap_or(false)
        }

        pub fn has_sidx(&self) -> bool {
            HonzoParser::new(&self.buf, 1)
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
            let b = match self.builder.take() {
                Some(b) => b,
                None => return false,
            };
            // dispatch based on tag and content type kind
            if &tag_arr == b"MATH" {
                if content_type_kind != 2 {
                    return false;
                }
                let math = match content_type_value {
                    0 => MathType::MathML,
                    1 => MathType::LaTeX,
                    _ => return false,
                };
                self.builder = Some(b.add_math_chunk(data, math, compression));
                return true;
            }

            if content_type_kind != 1 {
                return false;
            }
            let markup = match content_type_value {
                0 => MarkupType::Hmd,
                1 => MarkupType::Html,
                _ => return false,
            };
            self.builder = Some(b.add_chunk(
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
        write: &mut diplomat_runtime::DiplomatWrite,
    ) -> Result<(), HonzoErrorCode> {
        let normalized = normalize_search_term_impl(term);
        write
            .write_str(&normalized)
            .map_err(|_| HonzoErrorCode::Unknown)?;
        Ok(())
    }
}
