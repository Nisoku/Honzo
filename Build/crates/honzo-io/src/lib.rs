mod compression;
pub mod crypto;
mod extra;
mod meta;
mod reader;
mod stream;
mod utils;
mod writer;

pub use compression::{decompress, verify_crc32, verify_entry_crc32};
pub use extra::{find_extra, parse_extra, validate_extra, ExtraEntry};
pub use meta::{
    Accessibility, Contributor, HonzoMeta, Identifier, RenderHints, Revision, SeriesMeta,
};
pub use reader::HonzoReader;
pub use stream::{ChapterIter, HonzoStream};
pub use utils::{compute_reading_time, new_uuid};
pub use writer::DrmConfig;
pub use writer::HonzoBuilder;

pub use honzo_core::{
    Compression, CoverType, FontEmbedding, HonzoError, HonzoHead, LayoutMode, MarkupType, MathType,
    PmapEntry, TocEntry,
};

pub use honzo_chunks::data::covr::{generate_covr, generate_covt};
pub use honzo_chunks::data::sidx::{build_sidx, normalize_search_term};
pub use honzo_chunks::{data, extra as extra_types};
