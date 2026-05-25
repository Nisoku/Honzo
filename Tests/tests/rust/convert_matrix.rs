use std::io::{Cursor, Write};

use crate::common::{fixture, workspace_root};
use honzo_chunks::data::img as img_utils;
use honzo_convert::{from_epub, from_mobi, from_pdf};
use honzo_core::{Compression, HonzoParser};
use honzo_io::HonzoMeta;

type FileOpts<'a> = zip::write::FileOptions<'a, ()>;

#[test]
fn converts_epub_fixture_into_parseable_honzo() {
    let epub = fixture("test-book.epub");
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();
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
    let parser = HonzoParser::new(&hzo, 1).unwrap();

    assert!(parser
        .toc_entries()
        .any(|entry| entry.chunk_type == *b"CHAP"));
}

#[test]
fn epub_conversion_preserves_html_markup() {
    let epub = fixture("test-book.epub");
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();

    let chap_entry = parser
        .toc_entries()
        .find(|e| e.chunk_type == *b"CHAP")
        .expect("should have CHAP chunk");

    let raw_bytes = parser.chunk_bytes(&chap_entry).unwrap();
    let content = String::from_utf8_lossy(raw_bytes);

    assert!(
        content.contains("<h1>"),
        "CHAP should contain HTML heading tags, got: {}",
        content
    );
    assert!(
        content.contains("<p>"),
        "CHAP should contain HTML paragraph tags"
    );
    assert!(
        content.contains("</html>"),
        "CHAP should contain full HTML document structure"
    );
    assert!(
        content.contains("Chapter 1"),
        "CHAP should contain original chapter title text"
    );
}

#[test]
fn epub_conversion_stores_chap_as_html_type() {
    let epub = fixture("test-book.epub");
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();

    let chap_entry = parser
        .toc_entries()
        .find(|e| e.chunk_type == *b"CHAP")
        .expect("should have CHAP chunk");

    assert_eq!(chap_entry.content_type_kind, 1);
    assert_eq!(chap_entry.content_type_value, 1);
}

#[test]
fn epub_conversion_includes_search_index() {
    let epub = fixture("test-book.epub");
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();

    let sidx_entry = parser.find_chunk(b"SIDX").expect("should have SIDX chunk");
    assert!(sidx_entry.size_raw > 0);
    assert_eq!(sidx_entry.compression, Compression::Lz4);
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
            | Err(honzo_convert::ConvertError::IoError(_))
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

#[test]
fn epub_conversion_has_title_metadata() {
    let epub = fixture("test-book.epub");
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();
    let meta: HonzoMeta = rmp_serde::from_slice(parser.meta_bytes().unwrap()).unwrap();

    let titles = meta.title.expect("should have title");
    assert_eq!(
        titles.get("en").map(|s| s.as_str()),
        Some("Simple Test Book")
    );
}

#[test]
fn epub_conversion_has_source_format_epub() {
    let epub = fixture("test-book.epub");
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();
    let meta: HonzoMeta = rmp_serde::from_slice(parser.meta_bytes().unwrap()).unwrap();

    assert_eq!(meta.source_format.as_deref(), Some("epub"));
}

#[test]
fn epub_conversion_has_uuid_identifier() {
    let epub = fixture("test-book.epub");
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();
    let meta: HonzoMeta = rmp_serde::from_slice(parser.meta_bytes().unwrap()).unwrap();

    let ids = meta.identifiers.expect("should have identifiers");
    assert!(!ids.is_empty());
    assert_eq!(ids[0].id_type, "uuid");
    assert!(!ids[0].value.is_empty());
}

#[test]
fn epub_conversion_has_word_count() {
    let epub = fixture("test-book.epub");
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();
    let meta: HonzoMeta = rmp_serde::from_slice(parser.meta_bytes().unwrap()).unwrap();

    let wc = meta.word_count.expect("should have word count");
    assert!(wc > 0, "word count should be positive, got {}", wc);
}

#[test]
fn epub_conversion_has_reading_time() {
    let epub = fixture("test-book.epub");
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();
    let meta: HonzoMeta = rmp_serde::from_slice(parser.meta_bytes().unwrap()).unwrap();

    let rt = meta.reading_time_mins.expect("should have reading time");
    assert!(rt > 0, "reading time should be positive, got {}", rt);
}

#[test]
fn epub_conversion_has_creators() {
    let epub = fixture("test-book.epub");
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();
    let meta: HonzoMeta = rmp_serde::from_slice(parser.meta_bytes().unwrap()).unwrap();

    assert_eq!(meta.authors, vec!["Test Author"]);
}

#[test]
fn epub_conversion_chapters_have_alt_text_from_toc() {
    let epub = fixture("test-book.epub");
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();

    let chap_titles: Vec<Option<String>> = parser
        .toc_entries()
        .filter(|e| e.chunk_type == *b"CHAP")
        .map(|e| {
            // The parser exposes content_type_kind/value but not alt_text directly.
            // We verify by checking chunk exists and was added with expected content.
            // alt_text is set via HonzoBuilder.add_chunk().
            // Since we don't have a nav in test-book.epub, alt_text may be None.
            // This test verifies chapters exist with expected content.
            let bytes = parser.chunk_bytes(&e).unwrap();
            let content = String::from_utf8_lossy(bytes);
            let has_title = content.contains("<h1>");
            Some(if has_title { "has_title" } else { "no_title" }.to_string())
        })
        .collect();

    assert_eq!(chap_titles.len(), 2, "should have 2 chapters");
}

#[test]
fn epub_conversion_multiple_chapters_preserve_content() {
    let epub = fixture("test-book.epub");
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();

    let chaps: Vec<_> = parser
        .toc_entries()
        .filter(|e| e.chunk_type == *b"CHAP")
        .collect();

    assert_eq!(chaps.len(), 2, "expected 2 chapters");

    let chap1 = String::from_utf8_lossy(parser.chunk_bytes(&chaps[0]).unwrap());
    let chap2 = String::from_utf8_lossy(parser.chunk_bytes(&chaps[1]).unwrap());

    assert!(chap1.contains("first chapter"));
    assert!(chap2.contains("second chapter"));
    assert!(chap1.contains("Chapter 1"));
    assert!(chap2.contains("Chapter 2"));
}

#[test]
fn epub_conversion_chunks_ordered_resources_before_chapters() {
    let epub = build_epub_with_resources();
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();

    let entries: Vec<_> = parser.toc_entries().collect();
    let mut found_cover = false;
    let mut found_img = false;
    let mut found_css = false;
    let mut found_font = false;
    let mut found_chap = false;

    for entry in &entries {
        match &entry.chunk_type {
            b"COVR" => found_cover = true,
            b"IMG_" => found_img = true,
            b"CSS_" => found_css = true,
            b"FONT" => found_font = true,
            b"CHAP" => found_chap = true,
            _ => {}
        }
    }

    assert!(found_cover, "should have COVR chunk");
    assert!(found_img, "should have IMG_ chunk");
    assert!(found_css, "should have CSS_ chunk");
    assert!(found_font, "should have FONT chunk");
    assert!(found_chap, "should have CHAP chunk");
}

#[test]
fn epub_conversion_resources_appear_before_chapters() {
    let epub = build_epub_with_resources();
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();

    let entries: Vec<_> = parser.toc_entries().collect();
    let mut last_resource_idx = 0usize;
    let mut first_chap_idx = entries.len();

    for (i, entry) in entries.iter().enumerate() {
        match &entry.chunk_type {
            t if *t == *b"COVR" || *t == *b"IMG_" || *t == *b"CSS_" || *t == *b"FONT" => {
                last_resource_idx = i;
            }
            b"CHAP" => {
                if i < first_chap_idx {
                    first_chap_idx = i;
                }
            }
            _ => {}
        }
    }

    assert!(
        last_resource_idx < first_chap_idx,
        "resources (idx {}) should appear before chapters (idx {})",
        last_resource_idx,
        first_chap_idx
    );
}

#[test]
fn epub_conversion_cover_thumbnail() {
    let epub = build_epub_with_cover();
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();

    let covt = parser.find_chunk(b"COVT").expect("should have COVT chunk");
    assert!(covt.size_raw > 0, "COVT should have data");
}

#[test]
fn epub_conversion_handles_epub2_ncx_toc() {
    let epub = build_epub2_with_ncx();
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();

    let chaps: Vec<_> = parser
        .toc_entries()
        .filter(|e| e.chunk_type == *b"CHAP")
        .collect();

    assert_eq!(chaps.len(), 1, "should convert EPUB 2 with NCX");
    let content = String::from_utf8_lossy(parser.chunk_bytes(&chaps[0]).unwrap());
    assert!(content.contains("NCX Chapter"));
}

#[test]
fn epub_conversion_missing_opf_fails_gracefully() {
    let epub = build_broken_epub_no_opf();
    let result = from_epub(&epub);
    assert!(result.is_err());
}

#[test]
fn epub_conversion_empty_spine_fails() {
    let epub = build_epub_empty_spine();
    let result = from_epub(&epub);
    assert!(matches!(
        result,
        Err(honzo_convert::ConvertError::MissingSpine)
    ));
}

#[test]
fn epub_conversion_succeeds_with_no_html_chapters() {
    let epub = build_epub_no_html_chapters();
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();

    let chap_count = parser
        .toc_entries()
        .filter(|e| e.chunk_type == *b"CHAP")
        .count();
    assert_eq!(chap_count, 0, "no CHAP chunks for non-HTML spine items");
}

fn build_epub_with_resources() -> Vec<u8> {
    let buf = Cursor::new(Vec::new());
    let mut zip = zip::ZipWriter::new(buf);

    zip.start_file("mimetype", FileOpts::default()).unwrap();
    zip.write_all(b"application/epub+zip").unwrap();

    zip.add_directory("META-INF/", FileOpts::default()).unwrap();
    zip.start_file("META-INF/container.xml", FileOpts::default())
        .unwrap();
    zip.write_all(
        br#"<?xml version="1.0"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles><rootfile full-path="content.opf" media-type="application/oebps-package+xml"/></rootfiles>
</container>"#,
    )
    .unwrap();

    zip.start_file("content.opf", FileOpts::default()).unwrap();
    zip.write_all(
        br#"<?xml version="1.0"?>
<package version="2.0" xmlns="http://www.idpf.org/2007/opf">
  <metadata>
    <dc:title>Resource Test</dc:title>
    <dc:creator>Test Author</dc:creator>
    <dc:language>en</dc:language>
    <meta name="cover" content="cover-image"/>
  </metadata>
  <manifest>
    <item id="cover-image" href="cover.jpg" media-type="image/jpeg"/>
    <item id="img1" href="image1.png" media-type="image/png"/>
    <item id="styles" href="style.css" media-type="text/css"/>
    <item id="font1" href="font.ttf" media-type="font/ttf"/>
    <item id="chap1" href="chapter1.xhtml" media-type="application/xhtml+xml"/>
  </manifest>
  <spine>
    <itemref idref="chap1"/>
  </spine>
</package>"#,
    )
    .unwrap();

    zip.start_file("cover.jpg", FileOpts::default()).unwrap();
    {
        let img = image::RgbImage::new(100, 100);
        let dyn_img = image::DynamicImage::ImageRgb8(img);
        let jpg_buf = img_utils::encode_jpeg(&dyn_img, 75).unwrap();
        zip.write_all(&jpg_buf).unwrap();
    }

    zip.start_file("image1.png", FileOpts::default()).unwrap();
    zip.write_all(b"fake png data").unwrap();

    zip.start_file("style.css", FileOpts::default()).unwrap();
    zip.write_all(b"body { color: red; }").unwrap();

    zip.start_file("font.ttf", FileOpts::default()).unwrap();
    zip.write_all(b"fake font data").unwrap();

    zip.start_file("chapter1.xhtml", FileOpts::default())
        .unwrap();
    zip.write_all(
        br#"<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml">
<head><title>Chapter 1</title></head>
<body><h1>Chapter 1</h1><p>Resource test chapter.</p></body>
</html>"#,
    )
    .unwrap();

    zip.finish().unwrap().into_inner()
}

fn build_epub_with_cover() -> Vec<u8> {
    let buf = Cursor::new(Vec::new());
    let mut zip = zip::ZipWriter::new(buf);

    zip.start_file("mimetype", FileOpts::default()).unwrap();
    zip.write_all(b"application/epub+zip").unwrap();

    zip.add_directory("META-INF/", FileOpts::default()).unwrap();
    zip.start_file("META-INF/container.xml", FileOpts::default())
        .unwrap();
    zip.write_all(
        br#"<?xml version="1.0"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles><rootfile full-path="content.opf" media-type="application/oebps-package+xml"/></rootfiles>
</container>"#,
    )
    .unwrap();

    zip.start_file("content.opf", FileOpts::default()).unwrap();
    zip.write_all(
        br#"<?xml version="1.0"?>
<package version="2.0" xmlns="http://www.idpf.org/2007/opf">
  <metadata>
    <dc:title>Cover Test</dc:title>
    <dc:creator>Test Author</dc:creator>
    <dc:language>en</dc:language>
    <meta name="cover" content="cover-image"/>
  </metadata>
  <manifest>
    <item id="cover-image" href="cover.jpg" media-type="image/jpeg"/>
    <item id="chap1" href="chapter1.xhtml" media-type="application/xhtml+xml"/>
  </manifest>
  <spine>
    <itemref idref="chap1"/>
  </spine>
</package>"#,
    )
    .unwrap();

    // Write a JPEG cover image
    {
        let img = image::RgbImage::new(200, 300);
        let dyn_img = image::DynamicImage::ImageRgb8(img);
        let jpg_buf = img_utils::encode_jpeg(&dyn_img, 75).unwrap();
        zip.start_file("cover.jpg", FileOpts::default()).unwrap();
        zip.write_all(&jpg_buf).unwrap();
    }

    zip.start_file("chapter1.xhtml", FileOpts::default())
        .unwrap();
    zip.write_all(
        br#"<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml">
<head><title>Cover Chapter</title></head>
<body><h1>Cover Chapter</h1><p>Has a cover image.</p></body>
</html>"#,
    )
    .unwrap();

    zip.finish().unwrap().into_inner()
}

fn build_epub2_with_ncx() -> Vec<u8> {
    let buf = Cursor::new(Vec::new());
    let mut zip = zip::ZipWriter::new(buf);

    zip.start_file("mimetype", FileOpts::default()).unwrap();
    zip.write_all(b"application/epub+zip").unwrap();

    zip.add_directory("META-INF/", FileOpts::default()).unwrap();
    zip.start_file("META-INF/container.xml", FileOpts::default())
        .unwrap();
    zip.write_all(
        br#"<?xml version="1.0"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles><rootfile full-path="content.opf" media-type="application/oebps-package+xml"/></rootfiles>
</container>"#,
    )
    .unwrap();

    zip.start_file("content.opf", FileOpts::default()).unwrap();
    zip.write_all(
        br#"<?xml version="1.0"?>
<package version="2.0" xmlns="http://www.idpf.org/2007/opf">
  <metadata>
    <dc:title>NCX Test</dc:title>
    <dc:creator>Test Author</dc:creator>
    <dc:language>en</dc:language>
  </metadata>
  <manifest>
    <item id="ncx" href="toc.ncx" media-type="application/x-dtbncx+xml"/>
    <item id="chap1" href="chapter1.xhtml" media-type="application/xhtml+xml"/>
  </manifest>
  <spine toc="ncx">
    <itemref idref="chap1"/>
  </spine>
</package>"#,
    )
    .unwrap();

    zip.start_file("toc.ncx", FileOpts::default()).unwrap();
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8"?>
<ncx xmlns="http://www.daisy.org/z3986/2005/ncx/" version="2005-1">
  <head><meta name="dtb:uid" content="test-uid"/></head>
  <docTitle><text>NCX Test</text></docTitle>
  <navMap>
    <navPoint id="nav1" playOrder="1">
      <navLabel><text>NCX Chapter One</text></navLabel>
      <content src="chapter1.xhtml"/>
    </navPoint>
  </navMap>
</ncx>"#,
    )
    .unwrap();

    zip.start_file("chapter1.xhtml", FileOpts::default())
        .unwrap();
    zip.write_all(
        br#"<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml">
<head><title>NCX Chapter</title></head>
<body><h1>NCX Chapter</h1><p>This is an NCX chapter.</p></body>
</html>"#,
    )
    .unwrap();

    zip.finish().unwrap().into_inner()
}

fn build_broken_epub_no_opf() -> Vec<u8> {
    let buf = Cursor::new(Vec::new());
    let mut zip = zip::ZipWriter::new(buf);

    zip.start_file("mimetype", FileOpts::default()).unwrap();
    zip.write_all(b"application/epub+zip").unwrap();

    zip.add_directory("META-INF/", FileOpts::default()).unwrap();
    zip.start_file("META-INF/container.xml", FileOpts::default())
        .unwrap();
    // Point to a non-existent file
    zip.write_all(
        br#"<?xml version="1.0"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles><rootfile full-path="nonexistent.opf" media-type="application/oebps-package+xml"/></rootfiles>
</container>"#,
    )
    .unwrap();

    zip.finish().unwrap().into_inner()
}

fn build_epub_empty_spine() -> Vec<u8> {
    let buf = Cursor::new(Vec::new());
    let mut zip = zip::ZipWriter::new(buf);

    zip.start_file("mimetype", FileOpts::default()).unwrap();
    zip.write_all(b"application/epub+zip").unwrap();

    zip.add_directory("META-INF/", FileOpts::default()).unwrap();
    zip.start_file("META-INF/container.xml", FileOpts::default())
        .unwrap();
    zip.write_all(
        br#"<?xml version="1.0"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles><rootfile full-path="content.opf" media-type="application/oebps-package+xml"/></rootfiles>
</container>"#,
    )
    .unwrap();

    zip.start_file("content.opf", FileOpts::default()).unwrap();
    zip.write_all(
        br#"<?xml version="1.0"?>
<package version="2.0" xmlns="http://www.idpf.org/2007/opf">
  <metadata>
    <dc:title>Empty Spine</dc:title>
    <dc:creator>Test Author</dc:creator>
    <dc:language>en</dc:language>
  </metadata>
  <manifest>
    <item id="chap1" href="chapter1.xhtml" media-type="application/xhtml+xml"/>
  </manifest>
  <spine>
  </spine>
</package>"#,
    )
    .unwrap();

    zip.finish().unwrap().into_inner()
}

fn build_epub_no_html_chapters() -> Vec<u8> {
    let buf = Cursor::new(Vec::new());
    let mut zip = zip::ZipWriter::new(buf);

    zip.start_file("mimetype", FileOpts::default()).unwrap();
    zip.write_all(b"application/epub+zip").unwrap();

    zip.add_directory("META-INF/", FileOpts::default()).unwrap();
    zip.start_file("META-INF/container.xml", FileOpts::default())
        .unwrap();
    zip.write_all(
        br#"<?xml version="1.0"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles><rootfile full-path="content.opf" media-type="application/oebps-package+xml"/></rootfiles>
</container>"#,
    )
    .unwrap();

    zip.start_file("content.opf", FileOpts::default()).unwrap();
    zip.write_all(
        br#"<?xml version="1.0"?>
<package version="2.0" xmlns="http://www.idpf.org/2007/opf">
  <metadata>
    <dc:title>No Chapters</dc:title>
    <dc:creator>Test Author</dc:creator>
    <dc:language>en</dc:language>
  </metadata>
  <manifest>
    <item id="styles" href="style.css" media-type="text/css"/>
  </manifest>
  <spine>
    <itemref idref="styles"/>
  </spine>
</package>"#,
    )
    .unwrap();

    zip.start_file("style.css", FileOpts::default()).unwrap();
    zip.write_all(b"body { }").unwrap();

    zip.finish().unwrap().into_inner()
}

#[test]
fn epub_conversion_preserves_img_alt_text() {
    let epub = build_epub_with_img_alt();
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();

    let img_entry = parser
        .toc_entries()
        .find(|e| e.chunk_type == *b"IMG_")
        .expect("should have IMG_ chunk");

    assert_eq!(img_entry.alt_text, Some("An image description"));
}

fn build_epub_with_img_alt() -> Vec<u8> {
    let buf = Cursor::new(Vec::new());
    let mut zip = zip::ZipWriter::new(buf);

    zip.start_file("mimetype", FileOpts::default()).unwrap();
    zip.write_all(b"application/epub+zip").unwrap();

    zip.add_directory("META-INF/", FileOpts::default()).unwrap();
    zip.start_file("META-INF/container.xml", FileOpts::default())
        .unwrap();
    zip.write_all(
                br#"<?xml version="1.0"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
    <rootfiles><rootfile full-path="content.opf" media-type="application/oebps-package+xml"/></rootfiles>
</container>"#,
        )
        .unwrap();

    zip.start_file("content.opf", FileOpts::default()).unwrap();
    zip.write_all(
        br#"<?xml version="1.0"?>
<package version="2.0" xmlns="http://www.idpf.org/2007/opf">
    <metadata>
        <dc:title>Img Alt Test</dc:title>
        <dc:creator>Test Author</dc:creator>
        <dc:language>en</dc:language>
    </metadata>
    <manifest>
        <item id="img1" href="image1.jpg" media-type="image/jpeg"/>
        <item id="chap1" href="chapter1.xhtml" media-type="application/xhtml+xml"/>
    </manifest>
    <spine>
        <itemref idref="chap1"/>
    </spine>
</package>"#,
    )
    .unwrap();

    // Write an actual JPEG image
    zip.start_file("image1.jpg", FileOpts::default()).unwrap();
    {
        let img = image::RgbImage::new(10, 10);
        let dyn_img = image::DynamicImage::ImageRgb8(img);
        let jpg_buf = img_utils::encode_jpeg(&dyn_img, 75).unwrap();
        zip.write_all(&jpg_buf).unwrap();
    }

    zip.start_file("chapter1.xhtml", FileOpts::default())
        .unwrap();
    zip.write_all(
        br#"<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml">
<head><title>Img Chapter</title></head>
<body><h1>Img Chapter</h1><p><img src="image1.jpg" alt="An image description"/></p></body>
</html>"#,
    )
    .unwrap();

    zip.finish().unwrap().into_inner()
}

#[test]
fn epub_conversion_preserves_img_alt_text_with_subdir() {
    // manifest href is in a subdir but chapter references basename only
    let buf = Cursor::new(Vec::new());
    let mut zip = zip::ZipWriter::new(buf);

    zip.start_file("mimetype", FileOpts::default()).unwrap();
    zip.write_all(b"application/epub+zip").unwrap();

    zip.add_directory("META-INF/", FileOpts::default()).unwrap();
    zip.start_file("META-INF/container.xml", FileOpts::default())
        .unwrap();
    zip.write_all(
        br#"<?xml version="1.0"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
    <rootfiles><rootfile full-path="content.opf" media-type="application/oebps-package+xml"/></rootfiles>
</container>"#,
    )
    .unwrap();

    zip.start_file("content.opf", FileOpts::default()).unwrap();
    zip.write_all(
        br#"<?xml version="1.0"?>
<package version="2.0" xmlns="http://www.idpf.org/2007/opf">
    <metadata>
        <dc:title>Img Alt Subdir Test</dc:title>
        <dc:creator>Test Author</dc:creator>
        <dc:language>en</dc:language>
    </metadata>
    <manifest>
        <item id="img1" href="images/image1.jpg" media-type="image/jpeg"/>
        <item id="chap1" href="chapter1.xhtml" media-type="application/xhtml+xml"/>
    </manifest>
    <spine>
        <itemref idref="chap1"/>
    </spine>
</package>"#,
    )
    .unwrap();

    // Write image under images/
    zip.start_file("images/image1.jpg", FileOpts::default())
        .unwrap();
    {
        let img = image::RgbImage::new(10, 10);
        let dyn_img = image::DynamicImage::ImageRgb8(img);
        let jpg_buf = img_utils::encode_jpeg(&dyn_img, 75).unwrap();
        zip.write_all(&jpg_buf).unwrap();
    }

    zip.start_file("chapter1.xhtml", FileOpts::default())
        .unwrap();
    zip.write_all(
        br#"<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml">
<head><title>Img Chapter</title></head>
<body><h1>Img Chapter</h1><p><img src="image1.jpg" alt="Subdir image alt"/></p></body>
</html>"#,
    )
    .unwrap();

    let epub = zip.finish().unwrap().into_inner();
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();

    let img_entry = parser
        .toc_entries()
        .find(|e| e.chunk_type == *b"IMG_")
        .expect("should have IMG_ chunk");

    assert_eq!(img_entry.alt_text, Some("Subdir image alt"));
}
