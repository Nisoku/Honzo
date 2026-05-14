use honzo_core::{Compression, HonzoError, HonzoParser, LayoutMode};

fn fix(name: &str) -> Vec<u8> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::read(root.join("fixtures").join(name)).unwrap()
}

fn cor(name: &str) -> Vec<u8> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::read(root.join("corpus").join(name)).unwrap()
}

#[test]
fn parse_minimal() {
    let d = fix("minimal.hzo");
    let p = HonzoParser::new(&d, 1).unwrap();
    assert_eq!(p.head().version_major, 1);
    assert_eq!(p.head().chunk_count, 0);
    assert_eq!(p.head().toc_size, 8);
    assert_eq!(p.head().data_size, 0);
    assert!(p.head().meta_size > 0);
    assert_eq!(p.toc_entries().count(), 0);
    assert!(p.meta_bytes().is_ok());
}

#[test]
fn parse_novel() {
    let d = fix("novel.hzo");
    let p = HonzoParser::new(&d, 1).unwrap();
    assert_eq!(p.head().chunk_count, 5);
    assert_eq!(p.toc_entries().count(), 5);
    assert_eq!(p.head().layout_mode(), LayoutMode::Reflowable);
}

#[test]
fn parse_manga() {
    let d = fix("manga.hzo");
    let p = HonzoParser::new(&d, 1).unwrap();
    assert_eq!(p.head().chunk_count, 6);
    assert_eq!(p.head().layout_mode(), LayoutMode::Scroll);
}

#[test]
fn parse_textbook() {
    let d = fix("textbook.hzo");
    let p = HonzoParser::new(&d, 1).unwrap();
    assert_eq!(p.head().chunk_count, 3);
    assert_eq!(p.head().layout_mode(), LayoutMode::Fixed);
    assert_eq!(p.pmap_entries().count(), 10);
}

#[test]
fn parse_max_chunks() {
    let d = fix("max_chunks.hzo");
    let p = HonzoParser::new(&d, 1).unwrap();
    assert_eq!(p.head().chunk_count, 1000);
    assert_eq!(p.toc_entries().count(), 1000);
}

#[test]
fn parse_compression_flags() {
    let d = fix("compressed_zlib.hzo");
    let p = HonzoParser::new(&d, 1).unwrap();
    let chap = p.find_chunk(b"CHAP").unwrap();
    assert_eq!(chap.compression, Compression::Zlib);
    assert!(chap.size_compressed < chap.size_raw);
}

#[test]
fn bad_magic() {
    let d = cor("bad_magic.hzo");
    let err = HonzoParser::new(&d, 1).unwrap_err();
    assert!(matches!(err, HonzoError::InvalidMagic));
}

#[test]
fn version_too_new() {
    let d = cor("version_too_new.hzo");
    let err = HonzoParser::new(&d, 1).unwrap_err();
    assert!(matches!(err, HonzoError::ReaderVersionTooOld { .. }));
}

#[test]
fn truncated_head() {
    let d = cor("truncated_head.hzo");
    let err = HonzoParser::new(&d, 1).unwrap_err();
    assert!(matches!(err, HonzoError::BufferTooShort));
}

#[test]
fn truncated_toc() {
    let d = cor("truncated_toc.hzo");
    let err = HonzoParser::new(&d, 1).unwrap_err();
    assert!(matches!(err, HonzoError::BufferTooShort | HonzoError::Truncated));
}

#[test]
fn unknown_chunk_type() {
    let d = cor("unknown_chunk_type.hzo");
    let err = HonzoParser::new(&d, 1).unwrap_err();
    assert!(matches!(err, HonzoError::InvalidChunkType));
}

#[test]
fn zero_chunks() {
    let d = cor("zero_chunks.hzo");
    let p = HonzoParser::new(&d, 1).unwrap();
    assert_eq!(p.head().chunk_count, 0);
    assert_eq!(p.toc_entries().count(), 0);
}

#[test]
fn encrypted_chunk() {
    let d = cor("encrypted_chunk.hzo");
    let p = HonzoParser::new(&d, 1).unwrap();
    let chap = p.find_chunk(b"CHAP").unwrap();
    assert!(chap.is_encrypted());
    let err = p.chunk_bytes(&chap).unwrap_err();
    assert!(matches!(err, HonzoError::EncryptedChunk { .. }));
}

#[test]
fn empty_extra() {
    let d = cor("empty_extra.hzo");
    let p = HonzoParser::new(&d, 1).unwrap();
    assert!(p.extra_bytes().unwrap().is_empty());
}

#[test]
fn empty_meta() {
    let d = cor("empty_meta.hzo");
    let p = HonzoParser::new(&d, 1).unwrap();
    assert!(p.meta_bytes().unwrap().is_empty());
}
