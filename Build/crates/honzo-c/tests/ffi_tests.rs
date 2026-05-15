use std::path::Path;

fn fix(name: &str) -> Vec<u8> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::read(root.join("../../../Tests/fixtures").join(name)).unwrap()
}

fn cor(name: &str) -> Vec<u8> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::read(root.join("../../../Tests/corpus").join(name)).unwrap()
}

#[test]
fn ffi_parse_minimal() {
    let h = honzo_c::ffi::HonzoHandle::parse(&fix("minimal.hzo"), 1).unwrap();
    assert_eq!(h.chunk_count(), 0);
    assert!(!h.get_meta().is_empty());
}

#[test]
fn ffi_parse_novel() {
    let h = honzo_c::ffi::HonzoHandle::parse(&fix("novel.hzo"), 1).unwrap();
    assert_eq!(h.chunk_count(), 5);
    assert_eq!(h.layout_mode(), 0);
    assert!(!h.has_drm());
    assert!(!h.has_sidx());
}

#[test]
fn ffi_parse_manga() {
    let h = honzo_c::ffi::HonzoHandle::parse(&fix("manga.hzo"), 1).unwrap();
    assert_eq!(h.chunk_count(), 6);
    assert_eq!(h.layout_mode(), 2);
}

#[test]
fn ffi_parse_with_sidx() {
    let h = honzo_c::ffi::HonzoHandle::parse(&fix("with_sidx.hzo"), 1).unwrap();
    assert!(h.has_sidx());
}

#[test]
fn ffi_get_chunk() {
    let h = honzo_c::ffi::HonzoHandle::parse(&fix("novel.hzo"), 1).unwrap();
    let chunk = h.get_chunk(0).unwrap();
    assert!(!chunk.is_empty());
}

#[test]
fn ffi_get_meta() {
    let h = honzo_c::ffi::HonzoHandle::parse(&fix("novel.hzo"), 1).unwrap();
    assert!(!h.get_meta().is_empty());
}

#[test]
fn ffi_bad_magic() {
    assert!(honzo_c::ffi::HonzoHandle::parse(&cor("bad_magic.hzo"), 1).is_none());
}

#[test]
fn ffi_truncated_head() {
    assert!(honzo_c::ffi::HonzoHandle::parse(&cor("truncated_head.hzo"), 1).is_none());
}

#[test]
fn ffi_version_too_new() {
    assert!(honzo_c::ffi::HonzoHandle::parse(&cor("version_too_new.hzo"), 1).is_none());
}

#[test]
fn ffi_encrypted_chunk_skipped() {
    let h = honzo_c::ffi::HonzoHandle::parse(&cor("encrypted_chunk.hzo"), 1).unwrap();
    // Encrypted chunks get empty vec
    let chunk = h.get_chunk(0).unwrap();
    assert!(chunk.is_empty());
}

#[test]
fn ffi_builder_roundtrip() {
    let mut b = honzo_c::ffi::HonzoBuilderHandle::new();
    b.add_chunk(b"CHAP", b"hello world", 0, 0);
    assert!(b.finalize());
    let output = b.get_result();
    assert!(!output.is_empty());
    let h = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    assert_eq!(h.chunk_count(), 1);
}

#[test]
fn ffi_builder_set_meta() {
    let meta = rmp_serde::to_vec(&honzo_std::HonzoMeta::default()).unwrap();
    let mut b = honzo_c::ffi::HonzoBuilderHandle::new();
    b.add_chunk(b"CHAP", b"content", 0, 0);
    b.set_meta(&meta);
    assert!(b.finalize());
    let output = b.get_result();
    let h = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    assert_eq!(h.chunk_count(), 1);
}

#[test]
fn ffi_builder_invalid_compression() {
    let mut b = honzo_c::ffi::HonzoBuilderHandle::new();
    assert!(!b.add_chunk(b"CHAP", b"data", 99, 0));
}

#[test]
fn ffi_builder_invalid_markup() {
    let mut b = honzo_c::ffi::HonzoBuilderHandle::new();
    assert!(!b.add_chunk(b"CHAP", b"data", 0, 99));
}

#[test]
fn ffi_builder_empty() {
    let mut b = honzo_c::ffi::HonzoBuilderHandle::new();
    assert!(b.finalize());
    let output = b.get_result();
    let h = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    assert_eq!(h.chunk_count(), 0);
}
