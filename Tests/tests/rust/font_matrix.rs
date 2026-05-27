use honzo_chunks::data::font::{
    chunk_name, guess_font_format, is_font_tag, validate_font, FONT_TAG,
};

fn woff() -> Vec<u8> {
    b"wOFF".to_vec()
}

fn woff2() -> Vec<u8> {
    b"wOF2".to_vec()
}

fn otf() -> Vec<u8> {
    b"OTTO".to_vec()
}

fn ttf() -> Vec<u8> {
    let mut buf = vec![0x00, 0x01, 0x00, 0x00];
    buf.extend_from_slice(b"some ttf data");
    buf
}

fn ttf_alt() -> Vec<u8> {
    let mut buf = vec![0x00, 0x01, 0x00, 0x01];
    buf.extend_from_slice(b"some ttf data");
    buf
}

fn ttf_true() -> Vec<u8> {
    let mut buf = b"true".to_vec();
    buf.extend_from_slice(b"some ttf data");
    buf
}

fn ttf_typ1() -> Vec<u8> {
    let mut buf = b"typ1".to_vec();
    buf.extend_from_slice(b"some ttf data");
    buf
}

#[test]
fn recognizes_font_tag() {
    assert!(is_font_tag(&FONT_TAG));
    assert!(!is_font_tag(b"CHAP"));
    assert!(!is_font_tag(b"IMG_"));
    assert_eq!(chunk_name(), "font");
}

#[test]
fn guess_font_format_woff() {
    assert_eq!(guess_font_format(&woff()), Some("font/woff"));
}

#[test]
fn guess_font_format_woff2() {
    assert_eq!(guess_font_format(&woff2()), Some("font/woff2"));
}

#[test]
fn guess_font_format_otf() {
    assert_eq!(guess_font_format(&otf()), Some("font/otf"));
}

#[test]
fn guess_font_format_ttf() {
    assert_eq!(guess_font_format(&ttf()), Some("font/ttf"));
    assert_eq!(guess_font_format(&ttf_alt()), Some("font/ttf"));
    assert_eq!(guess_font_format(&ttf_true()), Some("font/ttf"));
    assert_eq!(guess_font_format(&ttf_typ1()), Some("font/ttf"));
}

#[test]
fn guess_font_format_unknown() {
    assert_eq!(guess_font_format(b"unknown"), None);
    assert_eq!(guess_font_format(b"ab"), None);
    assert_eq!(guess_font_format(b""), None);
}

#[test]
fn validates_font_woff() {
    assert!(validate_font(&woff()).is_ok());
}

#[test]
fn validates_font_ttf() {
    assert!(validate_font(&ttf()).is_ok());
}

#[test]
fn rejects_invalid_font() {
    assert!(validate_font(b"not a font").is_err());
    assert!(validate_font(b"").is_err());
    assert!(validate_font(b"abc").is_err());
}
