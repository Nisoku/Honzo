use getrandom::getrandom;
use honzo_core::HonzoError;
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::GenericImageView;

pub fn generate_covt(covr_bytes: &[u8]) -> Result<Vec<u8>, HonzoError> {
    let img = image::load_from_memory(covr_bytes).map_err(|_| HonzoError::Truncated)?;
    let (width, height) = img.dimensions();
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

pub fn new_uuid() -> String {
    let mut bytes = [0u8; 16];
    let _ = getrandom(&mut bytes);
    bytes[6] = (bytes[6] & 0x0F) | 0x40;
    bytes[8] = (bytes[8] & 0x3F) | 0x80;
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0],
        bytes[1],
        bytes[2],
        bytes[3],
        bytes[4],
        bytes[5],
        bytes[6],
        bytes[7],
        bytes[8],
        bytes[9],
        bytes[10],
        bytes[11],
        bytes[12],
        bytes[13],
        bytes[14],
        bytes[15]
    )
}

pub fn compute_reading_time(word_count: u32) -> u32 {
    let mins = (word_count + 237) / 238;
    if mins == 0 {
        1
    } else {
        mins
    }
}
