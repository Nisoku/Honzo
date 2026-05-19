use honzo_core::HonzoError;
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::GenericImageView;

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

/// Cover-related helpers.
///
/// - `COVR`: the full-size cover image bytes (typically the best available quality)
/// - `COVT`: an optional cover thumbnail derived from `COVR` for quick previews
///
/// `COVT` is not for in-chapter images; those are `IMG_` chunks.
pub fn generate_covr<'a>(bytes: &'a [u8]) -> Result<&'a [u8], HonzoError> {
    let img = image::load_from_memory(bytes).map_err(|_| HonzoError::Truncated)?;
    let (w, h) = img.dimensions();
    check_dims(w, h)?;
    Ok(bytes)
}

pub fn generate_covt(covr_bytes: &[u8]) -> Result<Vec<u8>, HonzoError> {
    let img = image::load_from_memory(covr_bytes).map_err(|_| HonzoError::Truncated)?;
    let (width, height) = img.dimensions();
    check_dims(width, height)?;
    let longest = width.max(height);
    if longest <= 300 {
        return Ok(covr_bytes.to_vec());
    }

    let scale = 300.0 / longest as f32;
    let new_width = (width as f32 * scale).round() as u32;
    let new_height = (height as f32 * scale).round() as u32;
    let resized = img.resize_exact(new_width, new_height, FilterType::Lanczos3);
    let mut out = Vec::new();
    let mut encoder = JpegEncoder::new_with_quality(&mut out, 75);
    let rgb = resized.to_rgb8();
    encoder
        .encode(&rgb, new_width, new_height, image::ExtendedColorType::Rgb8)
        .map_err(|_| HonzoError::Truncated)?;
    Ok(out)
}
