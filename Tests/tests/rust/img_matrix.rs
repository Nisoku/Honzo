use honzo_chunks::data::img as img_utils;
use honzo_core::guess_image_mime;
use image::GenericImageView;

#[test]
fn validate_jpeg_and_guess_mime() {
    let img = image::ImageBuffer::from_fn(10, 10, |x, y| {
        image::Rgb([(x % 256) as u8, (y % 256) as u8, 128u8])
    });
    let dyn_img = image::DynamicImage::ImageRgb8(img);
    let jpg = img_utils::encode_jpeg(&dyn_img, 80).expect("encode jpeg");

    assert_eq!(guess_image_mime(&jpg), Some("image/jpeg"));
    assert!(img_utils::validate_img(&jpg).is_ok());
    let loaded = img_utils::load_image(&jpg).expect("load image");
    assert_eq!(loaded.dimensions(), (10, 10));
}

#[test]
fn validate_png_and_guess_mime() {
    let img = image::ImageBuffer::from_fn(8, 6, |x, y| {
        image::Rgb([(x * 31 % 256) as u8, (y * 43 % 256) as u8, 64u8])
    });
    let dyn_img = image::DynamicImage::ImageRgb8(img);
    let mut buf = Vec::new();
    dyn_img
        .write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png)
        .expect("write png");

    assert_eq!(guess_image_mime(&buf), Some("image/png"));
    assert!(img_utils::validate_img(&buf).is_ok());
}
