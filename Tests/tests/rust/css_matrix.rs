use honzo_chunks::data::css::{chunk_name, is_css_tag, validate_css, CSS_TAG};

#[test]
fn recognizes_css_tag() {
    assert!(is_css_tag(&CSS_TAG));
    assert!(!is_css_tag(b"FONT"));
    assert!(!is_css_tag(b"CHAP"));
    assert_eq!(chunk_name(), "stylesheet");
}

#[test]
fn validates_valid_css() {
    let css = b"body { color: red; }";
    let result = validate_css(css).unwrap();
    assert_eq!(result, "body { color: red; }");
}

#[test]
fn validates_css_with_at_rules() {
    let css = b"@import url('fonts.css'); .cls { margin: 0; }";
    assert!(validate_css(css).is_ok());
}

#[test]
fn validates_css_with_media_queries() {
    let css = b"@media (max-width: 600px) { .hide-mobile { display: none; } }";
    assert!(validate_css(css).is_ok());
}

#[test]
fn rejects_invalid_css() {
    // Null byte is illegal in CSS
    let css = b"body { color: red; }\0";
    assert!(validate_css(css).is_err());
}

#[test]
fn rejects_truncated_utf8() {
    let bytes = &[0xff, 0xfe, 0x00];
    assert!(validate_css(bytes).is_err());
}
