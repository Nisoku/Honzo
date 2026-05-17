use crate::common::fixture;
use honzo_convert::{from_epub, from_mobi, from_pdf};
use honzo_std::HonzoMeta;
use lopdf::{Dictionary, Document, Object, Stream};

#[test]
fn from_mobi_delegates_epub_bytes() {
    let epub = fixture("test-book.epub");
    let hzo_from_epub = from_epub(&epub).unwrap();
    let hzo_from_mobi = from_mobi(&epub).unwrap();

    let parser_epub = honzo_core::HonzoParser::new(&hzo_from_epub, 1).unwrap();
    let parser_mobi = honzo_core::HonzoParser::new(&hzo_from_mobi, 1).unwrap();

    let meta_epub: HonzoMeta = rmp_serde::from_slice(parser_epub.meta_bytes().unwrap()).unwrap();
    let meta_mobi: HonzoMeta = rmp_serde::from_slice(parser_mobi.meta_bytes().unwrap()).unwrap();

    assert_eq!(meta_epub.source_format, meta_mobi.source_format);
}

#[test]
fn from_mobi_rejects_plain_text() {
    assert!(matches!(
        from_mobi(b"plain text"),
        Err(honzo_convert::ConvertError::UnsupportedFormat)
    ));
}

#[test]
fn from_pdf_rejects_plain_text() {
    assert!(matches!(
        from_pdf(b"plain text"),
        Err(honzo_convert::ConvertError::IoError(_))
            | Err(honzo_convert::ConvertError::UnsupportedFormat)
    ));
}

#[test]
fn from_pdf_handles_generated_pdf() {
    let pdf = build_pdf(&["Hello PDF"]);
    let hzo = from_pdf(&pdf).unwrap();
    let parser = honzo_core::HonzoParser::new(&hzo, 1).unwrap();
    let meta: HonzoMeta = rmp_serde::from_slice(parser.meta_bytes().unwrap()).unwrap();

    assert_eq!(meta.source_format.as_deref(), Some("pdf"));
    assert!(parser.toc_entries().any(|e| e.chunk_type == *b"CHAP"));
}

#[test]
fn from_pdf_preserves_multiple_pages() {
    let pdf = build_pdf(&["Alpha", "Beta"]);
    let hzo = from_pdf(&pdf).unwrap();
    let parser = honzo_core::HonzoParser::new(&hzo, 1).unwrap();

    let chapter_count = parser
        .toc_entries()
        .filter(|entry| entry.chunk_type == *b"CHAP")
        .count();

    assert_eq!(chapter_count, 2);
}

#[test]
fn from_mobi_accepts_zip_inputs_via_epub_path() {
    let epub = fixture("test-book.epub");
    let hzo = from_mobi(&epub).unwrap();
    let parser = honzo_core::HonzoParser::new(&hzo, 1).unwrap();

    assert!(parser.head().chunk_count > 0);
}

fn build_pdf(page_texts: &[&str]) -> Vec<u8> {
    let mut doc = Document::with_version("1.4");
    let font_id = doc.add_object(Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Font".to_vec())),
        ("Subtype", Object::Name(b"Type1".to_vec())),
        ("BaseFont", Object::Name(b"Helvetica".to_vec())),
    ]));

    let pages_id = doc.new_object_id();
    let mut kids = Vec::new();

    for page_text in page_texts {
        let content = format!(
            "BT /F1 24 Tf 100 700 Td ({}) Tj ET",
            escape_pdf_text(page_text)
        );
        let content_id = doc.add_object(Stream::new(Dictionary::new(), content.into_bytes()));

        let resources = Dictionary::from_iter(vec![(
            "Font",
            Object::Dictionary(Dictionary::from_iter(vec![(
                "F1",
                Object::Reference(font_id),
            )])),
        )]);

        let page_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Page".to_vec())),
            ("Parent", Object::Reference(pages_id)),
            (
                "MediaBox",
                Object::Array(vec![
                    Object::Integer(0),
                    Object::Integer(0),
                    Object::Integer(612),
                    Object::Integer(792),
                ]),
            ),
            ("Contents", Object::Reference(content_id)),
            ("Resources", Object::Dictionary(resources)),
        ]));
        kids.push(Object::Reference(page_id));
    }

    let pages = Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Pages".to_vec())),
        ("Kids", Object::Array(kids)),
        ("Count", Object::Integer(page_texts.len() as i64)),
    ]);
    doc.objects.insert(pages_id, Object::Dictionary(pages));

    let catalog_id = doc.add_object(Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Catalog".to_vec())),
        ("Pages", Object::Reference(pages_id)),
    ]));
    doc.trailer.set("Root", Object::Reference(catalog_id));

    let mut buf = Vec::new();
    doc.save_to(&mut buf).unwrap();
    buf
}

fn escape_pdf_text(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
}
