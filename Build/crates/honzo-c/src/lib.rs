#[diplomat::bridge]
mod ffi {
    use core::fmt::Write as _;
    use honzo_std::{Compression, CoverType, MarkupType};

    #[repr(C)]
    pub enum HonzoErrorCode {
        Ok = 0,
        InvalidMagic = 1,
        ReaderVersionTooOld = 2,
        BufferTooShort = 3,
        CrcMismatch = 4,
        EncryptedChunk = 5,
        Unknown = 255,
    }

    #[diplomat::opaque]
    pub struct HonzoHandle {
        buf: Vec<u8>,
    }

    impl HonzoHandle {
        pub fn parse(data: &[u8], _reader_version: u16) -> Option<Box<HonzoHandle>> {
            honzo_core::HonzoParser::new(data, 1).ok()?;
            Some(Box::new(HonzoHandle { buf: data.to_vec() }))
        }

        pub fn chunk_count(&self) -> u32 {
            honzo_core::HonzoParser::new(&self.buf, 1)
                .map(|p| p.head().chunk_count)
                .unwrap_or(0)
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

        pub fn get_chunk(&self, index: u32, to: &mut diplomat_runtime::DiplomatWrite) -> Result<(), ()> {
            let p = honzo_core::HonzoParser::new(&self.buf, 1).map_err(|_| ())?;
            let entries: Vec<_> = p.toc_entries().collect();
            let entry = entries.get(index as usize).ok_or(())?;
            if entry.is_encrypted() {
                return Err(());
            }
            let raw = p.chunk_bytes(entry).map_err(|_| ())?;
            let decompressed = honzo_std::decompress(raw, entry.compression, entry.size_raw).map_err(|_| ())?;
            let s = core::str::from_utf8(&decompressed).map_err(|_| ())?;
            to.write_str(s).map_err(|_| ())
        }

        pub fn get_meta(&self, to: &mut diplomat_runtime::DiplomatWrite) -> Result<(), ()> {
            let p = honzo_core::HonzoParser::new(&self.buf, 1).map_err(|_| ())?;
            let meta = p.meta_bytes().map_err(|_| ())?;
            let s = core::str::from_utf8(meta).map_err(|_| ())?;
            to.write_str(s).map_err(|_| ())
        }
    }

    #[diplomat::opaque]
    pub struct HonzoBuilderHandle {
        builder: Option<honzo_std::Builder>,
    }

    impl HonzoBuilderHandle {
        pub fn new() -> Box<HonzoBuilderHandle> {
            Box::new(HonzoBuilderHandle {
                builder: Some(honzo_std::Builder::new()),
            })
        }

        pub fn add_chunk(
            &mut self,
            tag: &[u8],
            data: &[u8],
            compression: u8,
            markup_type: u8,
        ) -> bool {
            if tag.len() != 4 {
                return false;
            }
            let mut tag_arr = [0u8; 4];
            tag_arr.copy_from_slice(tag);
            let compression = match compression {
                0 => Compression::None,
                1 => Compression::Zlib,
                2 => Compression::Zstd,
                _ => return false,
            };
            let markup = match markup_type {
                0 => MarkupType::Hmd,
                1 => MarkupType::Html,
                _ => return false,
            };
            let b = match self.builder.take() {
                Some(b) => b,
                None => return false,
            };
            self.builder = Some(b.add_chunk(tag_arr, data, compression, markup, CoverType::Front, None, None, None));
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

        pub fn finalize(&self, to: &mut diplomat_runtime::DiplomatWrite) -> bool {
            let b = match self.builder.as_ref() {
                Some(b) => b.clone().finalize(),
                None => return false,
            };
            match b {
                Ok(bytes) => {
                    let s = core::str::from_utf8(&bytes).unwrap_or("");
                    to.write_str(s).is_ok()
                }
                Err(_) => false,
            }
        }
    }
}
