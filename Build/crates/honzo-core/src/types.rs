use crate::HonzoError;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Compression {
    None = 0,
    Zlib = 1,
    Zstd = 2,
}

impl Compression {
    pub fn from_u8(value: u8) -> Result<Self, HonzoError> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Zlib),
            2 => Ok(Self::Zstd),
            other => Err(HonzoError::UnknownCompression(other)),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LayoutMode {
    Reflowable = 0,
    Fixed = 1,
    Scroll = 2,
}

impl LayoutMode {
    pub fn from_u8(value: u8) -> Result<Self, HonzoError> {
        match value {
            0 => Ok(Self::Reflowable),
            1 => Ok(Self::Fixed),
            2 => Ok(Self::Scroll),
            other => Err(HonzoError::UnknownLayoutMode(other)),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MarkupType {
    Hmd = 0,
    Html = 1,
}

impl MarkupType {
    pub fn from_u8(value: u8) -> Result<Self, HonzoError> {
        match value {
            0 => Ok(Self::Hmd),
            1 => Ok(Self::Html),
            other => Err(HonzoError::UnknownMarkupType(other)),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoverType {
    Front = 0,
    Back = 1,
    FullSpread = 2,
}

impl CoverType {
    pub fn from_u8(value: u8) -> Result<Self, HonzoError> {
        match value {
            0 => Ok(Self::Front),
            1 => Ok(Self::Back),
            2 => Ok(Self::FullSpread),
            other => Err(HonzoError::UnknownCoverType(other)),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FontEmbedding {
    Allowed = 0,
    PrintOnly = 1,
    NoModify = 2,
    NoEmbed = 3,
}

impl FontEmbedding {
    pub fn from_u8(value: u8) -> Result<Self, HonzoError> {
        match value {
            0 => Ok(Self::Allowed),
            1 => Ok(Self::PrintOnly),
            2 => Ok(Self::NoModify),
            3 => Ok(Self::NoEmbed),
            other => Err(HonzoError::UnknownFontEmbedding(other)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HonzoHead {
    pub version_major: u8,
    pub version_minor: u8,
    pub min_reader_version: u16,
    pub flags: u32,
    pub chunk_count: u32,
    pub toc_size: u64,
    pub data_size: u64,
    pub extra_size: u64,
    pub meta_size: u64,
}

impl HonzoHead {
    pub fn compression_default(&self) -> Compression {
        let value = (self.flags & 0x03) as u8;
        Compression::from_u8(value).unwrap_or(Compression::None)
    }

    pub fn layout_mode(&self) -> LayoutMode {
        let value = ((self.flags >> 2) & 0x03) as u8;
        LayoutMode::from_u8(value).unwrap_or(LayoutMode::Reflowable)
    }

    pub fn has_drm(&self) -> bool {
        (self.flags & 0x10) != 0
    }

    pub fn has_sidx(&self) -> bool {
        (self.flags & 0x20) != 0
    }

    pub fn has_anno(&self) -> bool {
        (self.flags & 0x40) != 0
    }

    pub fn has_sync(&self) -> bool {
        (self.flags & 0x80) != 0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TocEntry<'a> {
    pub chunk_type: [u8; 4],
    pub chunk_id: u32,
    pub offset: u64,
    pub size_compressed: u32,
    pub size_raw: u32,
    pub compression: Compression,
    pub markup_type: MarkupType,
    pub cover_type: CoverType,
    pub flags: u8,
    pub crc32: u32,
    pub alt_text: Option<&'a str>,
    pub font_embedding: Option<FontEmbedding>,
    pub font_license_url: Option<&'a str>,
}

impl<'a> TocEntry<'a> {
    pub fn is_encrypted(&self) -> bool {
        (self.flags & 0x01) != 0
    }

    pub fn chunk_type_str(&self) -> &str {
        core::str::from_utf8(&self.chunk_type).unwrap_or("????")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PmapEntry {
    pub print_page: u32,
    pub chunk_id: u32,
    pub byte_offset: u32,
}
