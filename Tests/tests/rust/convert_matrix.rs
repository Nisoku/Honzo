use crate::common::{fixture, workspace_root};
use honzo_convert::{from_epub, from_mobi, from_pdf};
use honzo_std::HonzoMeta;

#[test]
fn converts_epub_fixture_into_parseable_honzo() {
    let epub = fixture("test-book.epub");
    let hzo = from_epub(&epub).unwrap();
    let parser = honzo_core::HonzoParser::new(&hzo, 1).unwrap();
    let meta: HonzoMeta = rmp_serde::from_slice(parser.meta_bytes().unwrap()).unwrap();

    assert!(parser.head().chunk_count > 0);
    assert!(!meta.authors.is_empty());
    assert_eq!(meta.language, "en");
    assert_eq!(meta.source_format.as_deref(), Some("epub"));
}

#[test]
fn epub_conversion_preserves_chapter_content_shape() {
    let epub = fixture("test-book.epub");
    let hzo = from_epub(&epub).unwrap();
    let parser = honzo_core::HonzoParser::new(&hzo, 1).unwrap();

    assert!(parser
        .toc_entries()
        .any(|entry| entry.chunk_type == *b"CHAP"));
}

#[test]
fn unsupported_inputs_are_rejected() {
    assert!(matches!(
        from_mobi(b"data"),
        Err(honzo_convert::ConvertError::UnsupportedFormat)
    ));
    assert!(matches!(
        from_pdf(b"data"),
        Err(honzo_convert::ConvertError::UnsupportedFormat)
    ));
}

#[test]
fn invalid_epub_data_fails() {
    let invalid = from_epub(b"not a valid epub");
    assert!(invalid.is_err());
}

#[test]
fn fixture_paths_are_workspace_relative() {
    let root = workspace_root();
    assert!(root.join("Tests/fixtures/test-book.epub").exists());
}
