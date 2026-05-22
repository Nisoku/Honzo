use honzo_io::{generate_covr, generate_covt};
use image::codecs::jpeg::JpegEncoder;
use image::ExtendedColorType;

fn make_1x1_jpeg() -> Vec<u8> {
    let mut out = Vec::new();
    let pixel = [0u8, 0u8, 0u8];
    let mut encoder = JpegEncoder::new(&mut out);
    encoder
        .encode(&pixel, 1, 1, ExtendedColorType::Rgb8)
        .expect("encode jpeg");
    out
}

#[test]
fn covr_valid_jpeg_passes() {
    let jpg = make_1x1_jpeg();
    let out = generate_covr(&jpg).unwrap();
    assert_eq!(out, jpg.as_slice());
}

#[test]
fn covt_from_small_jpeg_returns_original() {
    let jpg = make_1x1_jpeg();
    let out = generate_covt(&jpg).unwrap();
    assert_eq!(out, jpg);
}

#[test]
fn invalid_jpeg_returns_error() {
    let bad = b"not a jpeg";
    assert!(generate_covr(bad).is_err());
    assert!(generate_covt(bad).is_err());
}

#[test]
fn covt_resizes_large_image() {
    // create a 800x600 image and ensure thumbnailing produces different bytes
    let mut out = Vec::new();
    let img = image::ImageBuffer::from_fn(800, 600, |x, y| {
        image::Rgb([(x % 256) as u8, (y % 256) as u8, 128u8])
    });
    let mut enc = JpegEncoder::new_with_quality(&mut out, 75);
    enc.encode(&img, 800, 600, ExtendedColorType::Rgb8).unwrap();

    let thumb = generate_covt(&out).unwrap();
    assert!(!thumb.is_empty());
    assert_ne!(thumb, out);
}
