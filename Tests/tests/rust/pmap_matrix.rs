use crate::common::fixture;
use honzo_convert::from_epub;
use honzo_core::{HonzoParser, PmapEntry};
use honzo_io::{Compression, CoverType, HonzoBuilder, HonzoMeta, MarkupType};

// pagebreak detection tests (testing the public API from honzo-convert)

// These test the public detect_pagebreaks function.
// The helper functions (is_pagebreak_tag, extract_page_number, etc.) are
// crate-internal; we test them indirectly through detect_pagebreaks.

#[test]
fn pb_detect_epub_type_pagebreak() {
    let html =
        r#"<p>Some text.</p><span epub:type="pagebreak" id="pg42" title="42"/><p>More text.</p>"#;
    let pages = honzo_convert::detect_pagebreaks(html);
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].0, 42);
}

#[test]
fn pb_detect_class_pagebreak() {
    let html = r#"<p>Before</p><span class="pagebreak" id="page-7"/><p>After</p>"#;
    let pages = honzo_convert::detect_pagebreaks(html);
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].0, 7);
}

#[test]
fn pb_detect_multiple_pagebreaks() {
    let html = r#"<span epub:type="pagebreak" title="1"/><p>Chapter 1</p><span epub:type="pagebreak" title="2"/><p>More</p>"#;
    let pages = honzo_convert::detect_pagebreaks(html);
    assert_eq!(pages.len(), 2);
    assert_eq!(pages[0].0, 1);
    assert_eq!(pages[1].0, 2);
}

#[test]
fn pb_detect_pagebreak_tag() {
    let html = r#"<pagebreak title="42"/>"#;
    let pages = honzo_convert::detect_pagebreaks(html);
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].0, 42);
}

#[test]
fn pb_detect_role_doc_pagebreak() {
    let html = r#"<span role="doc-pagebreak" title="99"/>text"#;
    let pages = honzo_convert::detect_pagebreaks(html);
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].0, 99);
}

#[test]
fn pb_detect_data_page_attribute() {
    let html = r#"<span class="pagebreak" data-page="33"/>text"#;
    let pages = honzo_convert::detect_pagebreaks(html);
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].0, 33);
}

#[test]
fn pb_no_false_positives() {
    let html = r#"<p>pagebreak is just text here</p><span class="nope">not a break</span>"#;
    let pages = honzo_convert::detect_pagebreaks(html);
    assert_eq!(pages.len(), 0);
}

#[test]
fn pb_estimate_simple() {
    let texts = vec!["a".repeat(5000), "b".repeat(3000)];
    let chunk_ids = vec![0u32, 1u32];
    let entries = honzo_convert::estimate_pagebreaks(&texts, &chunk_ids, 2000);
    assert_eq!(entries.len(), 5);
    assert_eq!(entries[0], (1, 0, 0));
    assert_eq!(entries[1], (2, 0, 2000));
    assert_eq!(entries[2], (3, 0, 4000));
    assert_eq!(entries[3], (4, 1, 0));
    assert_eq!(entries[4], (5, 1, 2000));
}

#[test]
fn pb_estimate_empty_chunks() {
    let texts = vec![String::new(), "content".to_string()];
    let chunk_ids = vec![0u32, 1u32];
    let entries = honzo_convert::estimate_pagebreaks(&texts, &chunk_ids, 2000);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0], (1, 1, 0));
}

// PMAP parser integration tests

#[test]
fn pmap_entries_in_textbook() {
    let data = fixture("textbook.hzo");
    let parser = HonzoParser::new(&data, 1).unwrap();
    let entries: Vec<_> = parser.pmap_entries().collect();

    assert_eq!(entries.len(), 10);
    assert_eq!(entries[0].print_page, 1);
    assert_eq!(entries[9].print_page, 10);
    assert!(entries
        .windows(2)
        .all(|pair| pair[0].print_page < pair[1].print_page));
}

#[test]
fn pmap_empty_when_no_pmap_section() {
    let data = fixture("minimal.hzo");
    let parser = HonzoParser::new(&data, 1).unwrap();
    let entries: Vec<_> = parser.pmap_entries().collect();
    assert_eq!(entries.len(), 0);
}

#[test]
fn pmap_empty_when_no_chunks() {
    let meta = rmp_serde::to_vec(&HonzoMeta::default()).unwrap();
    let hzo = HonzoBuilder::new().set_meta(&meta).finalize().unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();
    assert_eq!(parser.pmap_entries().count(), 0);
}

#[test]
fn pmap_builder_single_entry() {
    let meta = rmp_serde::to_vec(&HonzoMeta::default()).unwrap();
    let hzo = HonzoBuilder::new()
        .add_chunk(
            *b"CHAP",
            b"content",
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_pmap_entry(PmapEntry {
            print_page: 1,
            chunk_id: 0,
            byte_offset: 0,
        })
        .set_meta(&meta)
        .finalize()
        .unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();
    assert_eq!(parser.pmap_entries().count(), 1);
    let e = parser.pmap_entries().next().unwrap();
    assert_eq!(e.print_page, 1);
    assert_eq!(e.chunk_id, 0);
    assert_eq!(e.byte_offset, 0);
}

#[test]
fn pmap_builder_multiple_chunks() {
    let meta = rmp_serde::to_vec(&HonzoMeta::default()).unwrap();
    let hzo = HonzoBuilder::new()
        .add_chunk(
            *b"CHAP",
            b"chapter one",
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"CHAP",
            b"chapter two",
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_pmap_entry(PmapEntry {
            print_page: 1,
            chunk_id: 0,
            byte_offset: 0,
        })
        .add_pmap_entry(PmapEntry {
            print_page: 2,
            chunk_id: 1,
            byte_offset: 0,
        })
        .set_meta(&meta)
        .finalize()
        .unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();
    let entries: Vec<_> = parser.pmap_entries().collect();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].print_page, 1);
    assert_eq!(entries[0].chunk_id, 0);
    assert_eq!(entries[1].print_page, 2);
    assert_eq!(entries[1].chunk_id, 1);
}

#[test]
fn pmap_builder_with_lz4_compressed_chunks() {
    use honzo_io::decompress;
    let meta = rmp_serde::to_vec(&HonzoMeta::default()).unwrap();
    let large = "content ".repeat(1000);
    let hzo = HonzoBuilder::new()
        .add_chunk(
            *b"CHAP",
            large.as_bytes(),
            Compression::Lz4,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_pmap_entry(PmapEntry {
            print_page: 1,
            chunk_id: 0,
            byte_offset: 0,
        })
        .set_meta(&meta)
        .finalize()
        .unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();
    let entry = parser.find_chunk(b"CHAP").unwrap();
    assert_eq!(entry.compression, Compression::Lz4);
    let raw = parser.chunk_bytes(&entry).unwrap();
    let decompressed = decompress(raw, entry.compression, entry.size_raw).unwrap();
    assert_eq!(decompressed.len(), large.len());
    let pmap_entries: Vec<_> = parser.pmap_entries().collect();
    assert_eq!(pmap_entries.len(), 1);
}

#[test]
fn pmap_roundtrip_complex() {
    let meta = rmp_serde::to_vec(&HonzoMeta::default()).unwrap();
    let hzo = HonzoBuilder::new()
        .add_chunk(
            *b"CHAP",
            b"content",
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_pmap_entry(PmapEntry {
            print_page: 1,
            chunk_id: 0,
            byte_offset: 0,
        })
        .add_pmap_entry(PmapEntry {
            print_page: 5,
            chunk_id: 0,
            byte_offset: 100,
        })
        .add_pmap_entry(PmapEntry {
            print_page: 10,
            chunk_id: 0,
            byte_offset: 500,
        })
        .set_meta(&meta)
        .finalize()
        .unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();
    let entries: Vec<_> = parser.pmap_entries().collect();
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].print_page, 1);
    assert_eq!(entries[0].byte_offset, 0);
    assert_eq!(entries[1].print_page, 5);
    assert_eq!(entries[1].byte_offset, 100);
    assert_eq!(entries[2].print_page, 10);
    assert_eq!(entries[2].byte_offset, 500);
}

#[test]
fn pmap_preserves_insertion_order() {
    // PMAP entries are stored in insertion order (caller should pre-sort)
    let meta = rmp_serde::to_vec(&HonzoMeta::default()).unwrap();
    let hzo = HonzoBuilder::new()
        .add_chunk(
            *b"CHAP",
            b"content",
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_pmap_entry(PmapEntry {
            print_page: 10,
            chunk_id: 0,
            byte_offset: 0,
        })
        .add_pmap_entry(PmapEntry {
            print_page: 1,
            chunk_id: 0,
            byte_offset: 0,
        })
        .add_pmap_entry(PmapEntry {
            print_page: 5,
            chunk_id: 0,
            byte_offset: 0,
        })
        .set_meta(&meta)
        .finalize()
        .unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();
    let entries: Vec<_> = parser.pmap_entries().collect();
    assert_eq!(entries.len(), 3);
    // Entries are in insertion order
    assert_eq!(entries[0].print_page, 10);
    assert_eq!(entries[1].print_page, 1);
    assert_eq!(entries[2].print_page, 5);
}

// PMAP from EPUB conversion

#[test]
fn pmap_from_converted_epub_has_entries() {
    let epub = fixture("test-book.epub");
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();
    let pmap_entries: Vec<_> = parser.pmap_entries().collect();
    assert!(
        !pmap_entries.is_empty(),
        "converted EPUB should have PMAP entries"
    );
}

#[test]
fn pmap_converted_entries_reference_valid_chunk_ids() {
    let epub = fixture("test-book.epub");
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();
    let chunk_ids: Vec<u32> = parser.toc_entries().map(|e| e.chunk_id).collect();

    for entry in parser.pmap_entries() {
        assert!(
            chunk_ids.contains(&entry.chunk_id),
            "PMAP entry references chunk_id {} which doesn't exist in TOC",
            entry.chunk_id
        );
    }
}

#[test]
fn pmap_converted_entries_are_ordered() {
    let epub = fixture("test-book.epub");
    let hzo = from_epub(&epub).unwrap();
    let parser = HonzoParser::new(&hzo, 1).unwrap();
    let entries: Vec<_> = parser.pmap_entries().collect();

    assert!(
        entries
            .windows(2)
            .all(|w| w[0].print_page <= w[1].print_page),
        "PMAP entries should be ordered by print_page"
    );
}

// PMAP from dedicated fixture

#[test]
fn pmap_fixture_matches_expected() {
    let data = fixture("with_pmap.hzo");
    let parser = HonzoParser::new(&data, 1).unwrap();
    let entries: Vec<_> = parser.pmap_entries().collect();

    assert_eq!(entries.len(), 6);
    assert_eq!(
        entries[0],
        PmapEntry {
            print_page: 1,
            chunk_id: 0,
            byte_offset: 0
        }
    );
    assert_eq!(
        entries[1],
        PmapEntry {
            print_page: 3,
            chunk_id: 1,
            byte_offset: 0
        }
    );
    assert_eq!(
        entries[2],
        PmapEntry {
            print_page: 5,
            chunk_id: 2,
            byte_offset: 0
        }
    );
    assert_eq!(
        entries[3],
        PmapEntry {
            print_page: 10,
            chunk_id: 2,
            byte_offset: 7
        }
    );
    assert_eq!(
        entries[4],
        PmapEntry {
            print_page: 15,
            chunk_id: 3,
            byte_offset: 0
        }
    );
    assert_eq!(
        entries[5],
        PmapEntry {
            print_page: 20,
            chunk_id: 3,
            byte_offset: 12
        }
    );
}

// C API PMAP tests

#[test]
fn ffi_pmap_build_and_read() {
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    builder.add_chunk(b"CHAP", b"page one", 0, 1, 0, 0, "", -1, "");
    builder.add_chunk(b"CHAP", b"page two", 0, 1, 0, 0, "", -1, "");
    assert!(builder.add_pmap_entry(1, 0, 0));
    assert!(builder.add_pmap_entry(2, 1, 0));
    assert!(builder.finalize());
    let output = builder.get_result();

    let handle = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    let mut buffer = diplomat_runtime::rust_interop::RustWriteVec::with_capacity(1024);
    let result = handle.get_pmap(unsafe { buffer.borrow_mut() });
    assert!(result.is_ok());
    let pmap_json = std::str::from_utf8(buffer.borrow().as_bytes()).unwrap();
    assert!(pmap_json.contains(r#"{"print_page":1,"chunk_id":0,"byte_offset":0}"#));
    assert!(pmap_json.contains(r#"{"print_page":2,"chunk_id":1,"byte_offset":0}"#));
}

#[test]
fn ffi_pmap_empty_when_none_added() {
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    builder.add_chunk(b"CHAP", b"content", 0, 1, 0, 0, "", -1, "");
    assert!(builder.finalize());
    let output = builder.get_result();

    let handle = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    let mut buffer = diplomat_runtime::rust_interop::RustWriteVec::with_capacity(128);
    let result = handle.get_pmap(unsafe { buffer.borrow_mut() });
    assert!(result.is_ok());
    let pmap_json = std::str::from_utf8(buffer.borrow().as_bytes()).unwrap();
    assert_eq!(pmap_json, "[]");
}

#[test]
fn ffi_pmap_on_textbook_fixture() {
    let handle = honzo_c::ffi::HonzoHandle::parse(&fixture("textbook.hzo"), 1).unwrap();
    let mut buffer = diplomat_runtime::rust_interop::RustWriteVec::with_capacity(2048);
    let result = handle.get_pmap(unsafe { buffer.borrow_mut() });
    assert!(result.is_ok());
    let pmap_json = std::str::from_utf8(buffer.borrow().as_bytes()).unwrap();
    assert!(pmap_json.contains(r#""print_page":1"#));
    assert!(pmap_json.contains(r#""print_page":10"#));
}

#[test]
fn ffi_pmap_on_minimal_fixture_is_empty() {
    let handle = honzo_c::ffi::HonzoHandle::parse(&fixture("minimal.hzo"), 1).unwrap();
    let mut buffer = diplomat_runtime::rust_interop::RustWriteVec::with_capacity(128);
    let result = handle.get_pmap(unsafe { buffer.borrow_mut() });
    assert!(result.is_ok());
    let pmap_json = std::str::from_utf8(buffer.borrow().as_bytes()).unwrap();
    assert_eq!(pmap_json, "[]");
}
