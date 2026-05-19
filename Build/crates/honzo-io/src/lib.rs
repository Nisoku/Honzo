mod build;
mod compression;
mod extra;
mod meta;
mod reader;
mod stream;
mod utils;

pub use build::HonzoBuilder;
pub use compression::{decompress, verify_crc32, verify_entry_crc32};
pub use extra::{find_extra, parse_extra, ExtraEntry};
pub use meta::{
    Accessibility, Contributor, HonzoMeta, Identifier, RenderHints, Revision, SeriesMeta,
};
pub use reader::HonzoReader;
pub use stream::{ChapterIter, HonzoStream};
pub use utils::{compute_reading_time, new_uuid};

pub use honzo_core::{
    Compression, CoverType, FontEmbedding, HonzoError, HonzoHead, LayoutMode, MarkupType,
    PmapEntry, TocEntry,
};

pub use honzo_chunks::{data, extra as extra_types};
pub use honzo_chunks::data::covr::{generate_covt, generate_covr};
pub use honzo_chunks::data::sidx::build_sidx;
