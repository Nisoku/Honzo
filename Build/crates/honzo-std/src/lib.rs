mod build;
mod compression;
mod extra;
mod meta;
mod reader;
mod sidx;
mod stream;
mod utils;

pub use build::Builder;
pub use compression::{decompress, verify_crc32};
pub use extra::{find_extra, parse_extra, ExtraEntry};
pub use meta::{
    Accessibility, Contributor, HonzoMeta, Identifier, RenderHints, Revision, SeriesMeta,
};
pub use reader::Reader;
pub use sidx::build_sidx;
pub use stream::{ChapterIter, HonzoStream};
pub use utils::{compute_reading_time, generate_covt, new_uuid};

pub use honzo_core::{
    Compression, CoverType, FontEmbedding, HonzoError, HonzoHead, LayoutMode, MarkupType,
    PmapEntry, TocEntry,
};
