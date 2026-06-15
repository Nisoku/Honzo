pub mod chap;
#[cfg(feature = "image")]
pub mod covr;
pub mod css;
pub mod font;
#[cfg(feature = "image")]
pub mod img;
pub mod math;
pub mod sidx;

/// Returns true if the tag is one of the known chunk types.
pub fn is_known_chunk(tag: &[u8; 4]) -> bool {
    matches!(
        tag,
        b"CHAP" | b"IMG_" | b"CSS_" | b"FONT" | b"COVR" | b"COVT" | b"NOTE" | b"SIDX" | b"MATH"
    )
}
