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
