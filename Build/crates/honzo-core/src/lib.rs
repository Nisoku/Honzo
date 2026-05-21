#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod error;
mod parse;
mod types;

#[cfg(feature = "alloc")]
mod write;

pub use error::HonzoError;
pub use parse::{HonzoParser, PmapEntryIter, TocEntryIter};
pub use types::MathType;
pub use types::{
    Compression, CoverType, FontEmbedding, HonzoHead, LayoutMode, MarkupType, PmapEntry, TocEntry,
};

#[cfg(feature = "alloc")]
pub use write::HonzoBuilder;
