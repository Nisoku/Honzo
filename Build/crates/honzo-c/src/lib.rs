#[diplomat::bridge]
pub mod ffi {
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
        meta: Vec<u8>,
        chunks: Vec<Vec<u8>>,
    }

    impl HonzoHandle {
        pub fn parse(data: &[u8], _reader_version: u16) -> Option<Box<HonzoHandle>> {
            let p = honzo_core::HonzoParser::new(data, 1).ok()?;
            let meta = p.meta_bytes().ok()?.to_vec();

            let entries: Vec<_> = p.toc_entries().collect();
            let mut chunks = Vec::with_capacity(entries.len());

            for entry in &entries {
                if entry.is_encrypted() {
                    chunks.push(Vec::new());
                    continue;
                }
                let raw = p.chunk_bytes(entry).ok()?;
                let decompressed =
                    honzo_std::decompress(raw, entry.compression, entry.size_raw).ok()?;
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
        builder: Option<honzo_std::HonzoBuilder>,
        result: Vec<u8>,
    }

    impl HonzoBuilderHandle {
        pub fn new() -> Box<HonzoBuilderHandle> {
            Box::new(HonzoBuilderHandle {
                builder: Some(honzo_std::HonzoBuilder::new()),
                result: Vec::new(),
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
}
