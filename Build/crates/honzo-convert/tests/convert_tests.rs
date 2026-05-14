use std::path::Path;

fn fixture(name: &str) -> Vec<u8> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .parent().unwrap()
        .parent().unwrap();
    std::fs::read(root.join("Tests/fixtures").join(name)).unwrap()
}

#[test]
fn test_epub_roundtrip() {
    let epub_bytes = fixture("test-book.epub");
    let hzo = honzo_convert::from_epub(&epub_bytes).unwrap();
    let p = honzo_core::HonzoParser::new(&hzo, 1).unwrap();
    assert!(p.head().chunk_count > 0, "should produce chunks");
    let chaps: Vec<_> = p.toc_entries().filter(|e| &e.chunk_type == b"CHAP").collect();
    assert!(chaps.len() >= 1, "should have at least one chapter");
}

#[test]
fn test_epub_metadata() {
    let epub_bytes = fixture("test-book.epub");
    let hzo = honzo_convert::from_epub(&epub_bytes).unwrap();
    let p = honzo_core::HonzoParser::new(&hzo, 1).unwrap();
    let meta: honzo_std::HonzoMeta = rmp_serde::from_slice(p.meta_bytes().unwrap()).unwrap();
    assert_eq!(meta.language, "en");
    assert!(!meta.authors.is_empty(), "should extract authors");
    assert!(meta.source_format.as_deref() == Some("epub"));
}

#[test]
fn test_epub_word_count() {
    let epub_bytes = fixture("test-book.epub");
    let hzo = honzo_convert::from_epub(&epub_bytes).unwrap();
    let p = honzo_core::HonzoParser::new(&hzo, 1).unwrap();
    let meta: honzo_std::HonzoMeta = rmp_serde::from_slice(p.meta_bytes().unwrap()).unwrap();
    assert!(meta.word_count.unwrap_or(0) > 0, "should have word count");
    assert!(meta.reading_time_mins.unwrap_or(0) >= 1, "should have reading time");
}

#[test]
fn test_invalid_data() {
    match honzo_convert::from_epub(b"not a valid epub") {
        Err(_) => {} // expected
        Ok(_) => panic!("should fail on invalid data"),
    }
}

#[test]
fn test_empty_data() {
    match honzo_convert::from_epub(b"") {
        Err(_) => {}
        Ok(_) => panic!("should fail on empty data"),
    }
}

#[test]
fn test_mobi_stub() {
    assert!(matches!(
        honzo_convert::from_mobi(b"data"),
        Err(honzo_convert::ConvertError::UnsupportedFormat)
    ));
}

#[test]
fn test_pdf_stub() {
    assert!(matches!(
        honzo_convert::from_pdf(b"data"),
        Err(honzo_convert::ConvertError::UnsupportedFormat)
    ));
}
