//! `IMG_` chunk helpers.
//!
//! `IMG_` is for in-content images (figures/illustrations inside chapters)

use honzo_core::HonzoError;
use image::{codecs::jpeg::JpegEncoder, DynamicImage, GenericImageView};
use lexepub::core::chapter::{AstNode, ParsedChapter};
use lexepub::LexEpub;
use std::collections::HashMap;

pub const IMG_TAG: [u8; 4] = *b"IMG_";

const MAX_IMAGE_DIM_PX: u32 = 20_000;

fn check_dims(width: u32, height: u32) -> Result<(), HonzoError> {
    if width == 0 || height == 0 {
        return Err(HonzoError::Truncated);
    }
    if width > MAX_IMAGE_DIM_PX || height > MAX_IMAGE_DIM_PX {
        return Err(HonzoError::Truncated);
    }
    Ok(())
}

/// Load the image and return the decoded `DynamicImage` after basic validation.
pub fn load_image(bytes: &[u8]) -> Result<DynamicImage, HonzoError> {
    let img = image::load_from_memory(bytes).map_err(|_| HonzoError::Truncated)?;
    let (w, h) = img.dimensions();
    check_dims(w, h)?;
    Ok(img)
}

/// Validate raw image bytes for inclusion as an `IMG_` chunk.
/// Ensures the bytes decode as a supported image and that dimensions are sane.
pub fn validate_img(bytes: &[u8]) -> Result<&[u8], HonzoError> {
    load_image(bytes)?;
    Ok(bytes)
}

/// Helper to encode a `DynamicImage` to JPEG bytes with quality.
pub fn encode_jpeg(img: &DynamicImage, quality: u8) -> Result<Vec<u8>, HonzoError> {
    let rgb = img.to_rgb8();
    let (w, h) = rgb.dimensions();
    let mut out = Vec::new();
    let mut encoder = JpegEncoder::new_with_quality(&mut out, quality);
    encoder
        .encode(&rgb, w, h, image::ExtendedColorType::Rgb8)
        .map_err(|_| HonzoError::Truncated)?;
    Ok(out)
}

/// Walk provided parsed chapters' ASTs and collect a mapping of raw href/src -> alt text.
/// The returned map contains the raw attribute values as keys; callers should resolve
/// them against manifest/OPF paths as needed.
pub fn collect_img_alts_from_parsed(parsed: &[ParsedChapter]) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    fn walk(node: &AstNode, map: &mut HashMap<String, String>) {
        if let AstNode::Element {
            tag,
            attrs,
            children,
            ..
        } = node
        {
            if tag.eq_ignore_ascii_case("img") {
                if let Some(src) = attrs.get("src").or_else(|| attrs.get("href")) {
                    let alt = attrs.get("alt").cloned().unwrap_or_default();
                    map.entry(src.clone()).or_insert(alt);
                }
            }
            for c in children {
                walk(c, map);
            }
        }
    }

    for p in parsed.iter() {
        if let Some(ast) = &p.ast {
            walk(ast, &mut map);
        }
    }

    map
}

// Collect image alts and normalize keys by resolving per-chapter hrefs. The resolver
// behavior is: try to resolve raw chapter-relative hrefs to manifest/OPF paths; if
// resolution succeeds use the resolved path as a key, otherwise keep the raw href.
// (The async variant below performs resolution via `LexEpub`.)
pub async fn collect_and_resolve_img_alts_async(
    parsed: &[ParsedChapter],
    epub: &mut LexEpub,
) -> HashMap<String, String> {
    let raw_map = collect_img_alts_from_parsed(parsed);
    let mut resolved: HashMap<String, String> = HashMap::new();

    for (raw_href, alt) in raw_map.into_iter() {
        let mut final_key = raw_href.clone();
        for ci in 0..parsed.len() {
            match epub.resolve_chapter_resource_path(ci, &raw_href).await {
                Ok(p) => {
                    final_key = p;
                    break;
                }
                Err(_) => continue,
            }
        }
        // Insert both resolved key and the original raw href so callers can lookup
        // by either form.
        resolved.entry(final_key.clone()).or_insert(alt.clone());
        resolved.entry(raw_href).or_insert(alt);
    }

    resolved
}
