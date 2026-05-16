use crate::common::{corpus, fixture};
use honzo_core::{HonzoError, HonzoParser, LayoutMode};

#[test]
fn parses_minimal_header_and_empty_toc() {
    let data = fixture("minimal.hzo");
    let parser = HonzoParser::new(&data, 1).unwrap();

    assert_eq!(parser.head().version_major, 1);
    assert_eq!(parser.head().version_minor, 0);
    assert_eq!(parser.head().chunk_count, 0);
    assert_eq!(parser.head().toc_size, 8);
    assert_eq!(parser.head().layout_mode(), LayoutMode::Reflowable);
    assert_eq!(parser.toc_entries().count(), 0);
}

#[test]
fn finds_chunks_by_tag_and_id() {
    let data = fixture("novel.hzo");
    let parser = HonzoParser::new(&data, 1).unwrap();

    let chap_by_tag = parser.find_chunk(b"CHAP").unwrap();
    let chap_by_id = parser.find_chunk_by_id(chap_by_tag.chunk_id).unwrap();

    assert_eq!(chap_by_tag.chunk_type, *b"CHAP");
    assert_eq!(chap_by_id.chunk_id, chap_by_tag.chunk_id);
    assert_eq!(chap_by_id.size_raw, chap_by_tag.size_raw);
    assert_eq!(
        parser
            .toc_entries()
            .filter(|entry| entry.chunk_type == *b"CHAP")
            .count(),
        3
    );
}

#[test]
fn parses_fixed_and_scroll_layouts() {
    let textbook_data = fixture("textbook.hzo");
    let manga_data = fixture("manga.hzo");
    let textbook = HonzoParser::new(&textbook_data, 1).unwrap();
    let manga = HonzoParser::new(&manga_data, 1).unwrap();

    assert_eq!(textbook.head().layout_mode(), LayoutMode::Fixed);
    assert_eq!(manga.head().layout_mode(), LayoutMode::Scroll);
}

#[test]
fn exposes_pmap_entries_in_order() {
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
fn zero_copy_chunk_bytes_have_expected_length() {
    let data = fixture("novel.hzo");
    let parser = HonzoParser::new(&data, 1).unwrap();
    let entry = parser.find_chunk(b"CHAP").unwrap();
    let bytes = parser.chunk_bytes(&entry).unwrap();

    assert_eq!(bytes.len(), entry.size_compressed as usize);
    assert!(bytes.as_ptr() >= data.as_ptr());
}

#[test]
fn rejects_invalid_magic_and_versions() {
    let bad_magic = HonzoParser::new(&corpus("bad_magic.hzo"), 1).unwrap_err();
    let too_new = HonzoParser::new(&corpus("version_too_new.hzo"), 1).unwrap_err();

    assert!(matches!(bad_magic, HonzoError::InvalidMagic));
    assert!(matches!(too_new, HonzoError::ReaderVersionTooOld { .. }));
}
