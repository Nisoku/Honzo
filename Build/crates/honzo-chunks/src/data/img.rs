//! `IMG_` chunk helpers.
//!
//! `IMG_` is for in-content images (figures/illustrations inside chapters)

use honzo_core::HonzoError;
use image::{codecs::jpeg::JpegEncoder, DynamicImage, GenericImageView};
use lexepub::core::chapter::{AstNode, ParsedChapter};
use std::collections::{HashMap, HashSet};

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

// Collect image alts and resolve raw hrefs to canonical manifest/OPF paths
pub fn collect_and_resolve_img_alts_async<F>(
    parsed: &[ParsedChapter],
    valid_paths: &HashSet<String>,
    on_resolved: &F,
) -> HashMap<String, String>
where
    F: Fn(),
{
    let raw_map = collect_img_alts_from_parsed(parsed);
    let mut resolved: HashMap<String, String> = HashMap::with_capacity(raw_map.len());

    let base_href = parsed.first().map(|p| p.chapter_info.href.as_str());

    for (raw_href, alt) in raw_map {
        let key = resolve_alt_key(&raw_href, base_href, valid_paths);
        // Insert the resolved key first; the raw href is a distinct fallback key only when it
        // differs, so `alt` is cloned at most once.
        resolved.entry(key.clone()).or_insert_with(|| alt.clone());
        if raw_href != key {
            resolved.entry(raw_href).or_insert(alt);
        }
        on_resolved();
    }

    resolved
}

fn is_external_href(href: &str) -> bool {
    href.starts_with("http://")
        || href.starts_with("https://")
        || href.starts_with("mailto:")
        || href.starts_with("data:")
        || href.starts_with("blob:")
        || href.starts_with('#')
}

/// Resolve a raw `img` src to a alt key using the manifest path set
fn resolve_alt_key(
    raw_href: &str,
    base_href: Option<&str>,
    valid_paths: &HashSet<String>,
) -> String {
    let trimmed = raw_href.trim();
    if trimmed.is_empty() || is_external_href(trimmed) {
        return raw_href.to_string();
    }

    let path_only = trimmed.split('#').next().unwrap_or(trimmed);
    if !path_only.is_empty() {
        let direct = normalize_internal_path(path_only);
        if !direct.is_empty() && valid_paths.contains(&direct) {
            return direct;
        }
        if let Some(base) = base_href {
            let relative = resolve_href_against(base, trimmed);
            if valid_paths.contains(&relative) {
                return relative;
            }
        }
    }

    raw_href.to_string()
}

fn resolve_href_against(base_path: &str, href: &str) -> String {
    if href.trim().is_empty() {
        return base_path.to_string();
    }
    if is_external_href(href) {
        return href.to_string();
    }

    let (path_part, fragment) = match href.split_once('#') {
        Some((p, f)) => (p, Some(f)),
        None => (href, None),
    };

    let joined = if path_part.starts_with('/') {
        std::path::PathBuf::from(path_part.trim_start_matches('/'))
    } else if path_part.is_empty() {
        std::path::PathBuf::from(base_path)
    } else {
        let base_dir = std::path::Path::new(base_path)
            .parent()
            .unwrap_or_else(|| std::path::Path::new(""));
        base_dir.join(path_part)
    };

    let mut normalized = normalize_internal_path(&joined.to_string_lossy());
    if let Some(frag) = fragment {
        normalized.push('#');
        normalized.push_str(frag);
    }
    normalized
}

fn normalize_internal_path(path: &str) -> String {
    let mut parts: Vec<&str> = Vec::new();
    let normalized = path.replace('\\', "/");
    for segment in normalized.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            other => parts.push(other),
        }
    }
    parts.join("/")
}
