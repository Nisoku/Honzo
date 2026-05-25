use super::img::{encode_jpeg, load_image};
use honzo_core::HonzoError;
use image::imageops::FilterType;
use image::GenericImageView;

/// Cover-related helpers.
///
/// - `COVR`: the full-size cover image bytes (typically the best available quality)
/// - `COVT`: an optional cover thumbnail derived from `COVR` for quick previews
///
/// `COVT` is not for in-chapter images; those are `IMG_` chunks.
pub fn generate_covr(bytes: &[u8]) -> Result<&[u8], HonzoError> {
    let _img = load_image(bytes)?;
    Ok(bytes)
}

pub fn generate_covt(covr_bytes: &[u8]) -> Result<Vec<u8>, HonzoError> {
    let img = load_image(covr_bytes)?;
    let (width, height) = img.dimensions();
    let longest = width.max(height);
    if longest <= 300 {
        return Ok(covr_bytes.to_vec());
    }

    let scale = 300.0 / longest as f32;
    let new_width = (width as f32 * scale).round() as u32;
    let new_height = (height as f32 * scale).round() as u32;
    let resized = img.resize_exact(new_width, new_height, FilterType::Lanczos3);
    encode_jpeg(&resized, 75)
}
