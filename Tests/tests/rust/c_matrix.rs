use crate::common::{corpus, fixture};

#[test]
fn ffi_parse_minimal() {
    let handle = honzo_c::ffi::HonzoHandle::parse(&fixture("minimal.hzo"), 1).unwrap();
    assert_eq!(handle.chunk_count(), 0);
    assert!(!handle.get_meta().is_empty());
}

#[test]
fn ffi_parse_novel() {
    let handle = honzo_c::ffi::HonzoHandle::parse(&fixture("novel.hzo"), 1).unwrap();
    assert_eq!(handle.chunk_count(), 5);
    assert_eq!(handle.layout_mode(), 0);
    assert!(!handle.has_drm());
    assert!(!handle.has_sidx());
}

#[test]
fn ffi_parse_manga() {
    let handle = honzo_c::ffi::HonzoHandle::parse(&fixture("manga.hzo"), 1).unwrap();
    assert_eq!(handle.chunk_count(), 6);
    assert_eq!(handle.layout_mode(), 2);
}

#[test]
fn ffi_parse_with_sidx() {
    let handle = honzo_c::ffi::HonzoHandle::parse(&fixture("with_sidx.hzo"), 1).unwrap();
    assert!(handle.has_sidx());
}

#[test]
fn ffi_get_chunk() {
    let handle = honzo_c::ffi::HonzoHandle::parse(&fixture("novel.hzo"), 1).unwrap();
    let chunk = handle.get_chunk(0).unwrap();
    assert!(!chunk.is_empty());
}

#[test]
fn ffi_get_meta() {
    let handle = honzo_c::ffi::HonzoHandle::parse(&fixture("novel.hzo"), 1).unwrap();
    assert!(!handle.get_meta().is_empty());
}

#[test]
fn ffi_get_meta_parsed() {
    let handle = honzo_c::ffi::HonzoHandle::parse(&fixture("novel.hzo"), 1).unwrap();
    let mut buffer = diplomat_runtime::rust_interop::RustWriteVec::with_capacity(256);
    let result = handle.get_meta_parsed(unsafe { buffer.borrow_mut() });
    assert!(result.is_ok());
    let meta_json = std::str::from_utf8(buffer.borrow().as_bytes()).unwrap();
    assert!(meta_json.contains("\"language\":\"en\""));
    assert!(meta_json.contains("\"title\""));
}

#[test]
fn ffi_get_toc() {
    let handle = honzo_c::ffi::HonzoHandle::parse(&fixture("novel.hzo"), 1).unwrap();
    let mut buffer = diplomat_runtime::rust_interop::RustWriteVec::with_capacity(512);
    let result = handle.get_toc(unsafe { buffer.borrow_mut() });
    assert!(result.is_ok());
    let toc_json = std::str::from_utf8(buffer.borrow().as_bytes()).unwrap();
    assert!(toc_json.contains("\"chunk_type\":\"COVR\""));
    assert!(toc_json.contains("\"chunk_type\":\"CHAP\""));
}

#[test]
fn ffi_bad_magic() {
    assert!(honzo_c::ffi::HonzoHandle::parse(&corpus("bad_magic.hzo"), 1).is_none());
}

#[test]
fn ffi_truncated_head() {
    assert!(honzo_c::ffi::HonzoHandle::parse(&corpus("truncated_head.hzo"), 1).is_none());
}

#[test]
fn ffi_version_too_new() {
    assert!(honzo_c::ffi::HonzoHandle::parse(&corpus("version_too_new.hzo"), 1).is_none());
}

#[test]
fn ffi_encrypted_chunk_skipped() {
    let handle = honzo_c::ffi::HonzoHandle::parse(&corpus("encrypted_chunk.hzo"), 1).unwrap();
    let chunk = handle.get_chunk(0).unwrap();
    assert!(chunk.is_empty());
}

#[test]
fn ffi_builder_roundtrip() {
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    builder.add_chunk(b"CHAP", b"hello world", 0, 1, 0);
    assert!(builder.finalize());
    let output = builder.get_result();
    assert!(!output.is_empty());
    let handle = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    assert_eq!(handle.chunk_count(), 1);
}

#[test]
fn ffi_builder_math_chunk_roundtrip() {
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    assert!(builder.add_math_chunk(
        b"<math xmlns=\"http://www.w3.org/1998/Math/MathML\"><mi>x</mi></math>",
        0,
        0,
    ));
    assert!(builder.finalize());
    let output = builder.get_result();
    let handle = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    assert_eq!(handle.chunk_count(), 1);
}

#[test]
fn ffi_builder_set_meta() {
    let meta = rmp_serde::to_vec(&honzo_io::HonzoMeta::default()).unwrap();
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    builder.add_chunk(b"CHAP", b"content", 0, 1, 0);
    builder.set_meta(&meta);
    assert!(builder.finalize());
    let output = builder.get_result();
    let handle = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    assert_eq!(handle.chunk_count(), 1);
}

#[test]
fn ffi_builder_invalid_compression() {
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    assert!(!builder.add_chunk(b"CHAP", b"data", 99, 1, 0));
}

#[test]
fn ffi_builder_invalid_markup() {
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    assert!(!builder.add_chunk(b"CHAP", b"data", 0, 1, 99));
}

#[test]
fn ffi_builder_empty() {
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    assert!(builder.finalize());
    let output = builder.get_result();
    let handle = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    assert_eq!(handle.chunk_count(), 0);
}
