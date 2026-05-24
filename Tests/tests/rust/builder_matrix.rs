use crate::common::fixture;
use honzo_core::{Compression, CoverType, HonzoParser, LayoutMode, MarkupType, PmapEntry};
use honzo_io::{build_sidx, HonzoBuilder, HonzoMeta};
use std::collections::BTreeMap;

#[test]
fn builds_minimal_file_with_metadata_only() {
    let meta = HonzoMeta::default();
    let meta_bytes = rmp_serde::to_vec(&meta).unwrap();
    let file = HonzoBuilder::new()
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();
    let parser = HonzoParser::new(&file, 1).unwrap();

    assert_eq!(parser.head().chunk_count, 0);
    assert_eq!(parser.head().meta_size, meta_bytes.len() as u64);
}

#[test]
fn builds_compressed_chapters_that_stream_cleanly() {
    let meta = HonzoMeta::default();
    let meta_bytes = rmp_serde::to_vec(&meta).unwrap();
    let chapter = b"chapter text used for compression testing";

    let file = HonzoBuilder::new()
        .add_chunk(
            *b"CHAP",
            chapter,
            Compression::Lz4,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();

    let parser = HonzoParser::new(&file, 1).unwrap();
    let entry = parser.find_chunk(b"CHAP").unwrap();
    let decompressed = honzo_io::HonzoStream::open(std::io::Cursor::new(&file), 1)
        .unwrap()
        .read_chunk(&entry)
        .unwrap();

    assert_eq!(entry.compression, Compression::Lz4);
    assert_eq!(decompressed, chapter);
}

#[test]
fn builds_font_and_pmap_entries() {
    let meta_bytes = rmp_serde::to_vec(&HonzoMeta::default()).unwrap();
    let font_data = fixture("with_fonts.hzo");
    let font_parser = HonzoParser::new(&font_data, 1).unwrap();
    let font_entry = font_parser.find_chunk(b"FONT").unwrap();

    let file = HonzoBuilder::new()
        .set_layout(LayoutMode::Fixed)
        .add_chunk(
            *b"FONT",
            font_parser.chunk_bytes(&font_entry).unwrap(),
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            Some("sample font"),
            font_entry.font_embedding,
            font_entry.font_license_url,
        )
        .add_pmap_entry(PmapEntry {
            print_page: 1,
            chunk_id: 0,
            byte_offset: 0,
        })
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();

    let parser = HonzoParser::new(&file, 1).unwrap();
    assert_eq!(parser.head().layout_mode(), LayoutMode::Fixed);
    assert_eq!(parser.pmap_entries().count(), 1);
    let entry = parser.find_chunk(b"FONT").unwrap();
    assert_eq!(entry.alt_text, Some("sample font"));
}

#[test]
fn builds_sidx_chunk_from_text() {
    let meta_bytes = rmp_serde::to_vec(&HonzoMeta::default()).unwrap();
    let chapters = [(0u32, "hello world"), (1u32, "hello rust")];
    let sidx = build_sidx(&chapters, "en").unwrap();

    let file = HonzoBuilder::new()
        .add_chunk(
            *b"CHAP",
            chapters[0].1.as_bytes(),
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"CHAP",
            chapters[1].1.as_bytes(),
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"SIDX",
            &sidx,
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();

    let parser = HonzoParser::new(&file, 1).unwrap();
    assert!(parser.find_chunk(b"SIDX").is_some());
}

#[test]
fn auto_sidx_generates_index_when_chaps_exist() {
    let meta_bytes = rmp_serde::to_vec(&HonzoMeta::default()).unwrap();
    let file = HonzoBuilder::new()
        .set_language("en")
        .add_chunk(
            *b"CHAP",
            b"hello world",
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();
    let parser = HonzoParser::new(&file, 1).unwrap();
    assert!(parser.find_chunk(b"SIDX").is_some());
    assert!(parser.head().has_sidx());
}

#[test]
fn auto_sidx_skipped_when_disabled() {
    let meta_bytes = rmp_serde::to_vec(&HonzoMeta::default()).unwrap();
    let file = HonzoBuilder::new()
        .set_auto_sidx(false)
        .add_chunk(
            *b"CHAP",
            b"hello world",
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();
    let parser = HonzoParser::new(&file, 1).unwrap();
    assert!(parser.find_chunk(b"SIDX").is_none());
    assert!(!parser.head().has_sidx());
}

#[test]
fn auto_sidx_not_generated_without_chaps() {
    let meta_bytes = rmp_serde::to_vec(&HonzoMeta::default()).unwrap();
    let file = HonzoBuilder::new()
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();
    let parser = HonzoParser::new(&file, 1).unwrap();
    assert!(parser.find_chunk(b"SIDX").is_none());
}

#[test]
fn auto_sidx_uses_lz4_compression() {
    let meta_bytes = rmp_serde::to_vec(&HonzoMeta::default()).unwrap();
    let file = HonzoBuilder::new()
        .set_language("en")
        .add_chunk(
            *b"CHAP",
            b"hello world",
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();
    let parser = HonzoParser::new(&file, 1).unwrap();
    let sidx_entry = parser.find_chunk(b"SIDX").unwrap();
    assert_eq!(sidx_entry.compression, Compression::Lz4);
}

#[test]
fn auto_sidx_uses_provided_language() {
    let meta_bytes = rmp_serde::to_vec(&HonzoMeta::default()).unwrap();
    let file = HonzoBuilder::new()
        .set_language("de")
        .add_chunk(
            *b"CHAP",
            b"laufen laufen!",
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .set_meta(&meta_bytes)
        .finalize()
        .unwrap();
    let parser = HonzoParser::new(&file, 1).unwrap();
    let sidx_entry = parser.find_chunk(b"SIDX").unwrap();
    let raw = parser.chunk_bytes(&sidx_entry).unwrap();
    let decompressed =
        honzo_io::decompress(raw, sidx_entry.compression, sidx_entry.size_raw).unwrap();
    let index: BTreeMap<String, Vec<(u32, u32)>> = rmp_serde::from_slice(&decompressed).unwrap();
    assert!(index.contains_key("lauf"));
    assert!(!index.contains_key("laufen"));
}
