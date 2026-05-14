use crate::compression::{decompress, verify_entry_crc32};
use honzo_core::{HonzoError, HonzoParser, TocEntry};

pub struct Reader<'a> {
    parser: HonzoParser<'a>,
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8], reader_version: u16) -> Result<Self, HonzoError> {
        Ok(Self {
            parser: HonzoParser::new(buf, reader_version)?,
        })
    }

    pub fn head(&self) -> &honzo_core::HonzoHead {
        self.parser.head()
    }

    pub fn toc(&self) -> Vec<TocEntry<'a>> {
        self.parser.toc_entries().collect()
    }

    pub fn pmap(&self) -> Vec<honzo_core::PmapEntry> {
        self.parser.pmap_entries().collect()
    }

    pub fn chunk_bytes(&self, entry: &TocEntry) -> Result<Vec<u8>, HonzoError> {
        if entry.is_encrypted() {
            return Err(HonzoError::EncryptedChunk {
                chunk_id: entry.chunk_id,
            });
        }
        let raw = self.parser.chunk_bytes(entry)?;
        let data = decompress(raw, entry.compression, entry.size_raw)?;
        verify_entry_crc32(entry, &data)?;
        Ok(data)
    }

    pub fn meta_bytes(&self) -> Result<&'a [u8], HonzoError> {
        self.parser.meta_bytes()
    }

    pub fn extra_bytes(&self) -> Result<&'a [u8], HonzoError> {
        self.parser.extra_bytes()
    }
}
