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
    assert_eq!(handle.chunk_count(), 7);
    assert_eq!(handle.layout_mode(), 0);
    assert!(!handle.has_drm());
    assert!(handle.has_sidx());
}

#[test]
fn ffi_parse_manga() {
    let handle = honzo_c::ffi::HonzoHandle::parse(&fixture("manga.hzo"), 1).unwrap();
    assert_eq!(handle.chunk_count(), 7);
    assert_eq!(handle.layout_mode(), 2);
}

#[test]
fn ffi_parse_with_sidx() {
    let handle = honzo_c::ffi::HonzoHandle::parse(&fixture("with_sidx.hzo"), 1).unwrap();
    assert!(handle.has_sidx());
}

#[test]
fn ffi_get_chunk() {
    let mut handle = honzo_c::ffi::HonzoHandle::parse(&fixture("novel.hzo"), 1).unwrap();
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
    let mut handle = honzo_c::ffi::HonzoHandle::parse(&corpus("encrypted_chunk.hzo"), 1).unwrap();
    assert!(handle.get_chunk(0).is_none());
}

#[test]
fn ffi_builder_roundtrip() {
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    builder.add_chunk(b"CHAP", b"hello world", 0, 1, 0, 0, "", -1, "");
    assert!(builder.finalize());
    let output = builder.get_result();
    eprintln!(
        "ffi_builder_with_font_embedding: output len = {}",
        output.len()
    );
    if output.is_empty() {
        panic!("output is empty");
    }
    let handle = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    assert_eq!(handle.chunk_count(), 2);
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
    builder.add_chunk(b"CHAP", b"content", 0, 1, 0, 0, "", -1, "");
    builder.set_meta(&meta);
    assert!(builder.finalize());
    let output = builder.get_result();
    let handle = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    assert_eq!(handle.chunk_count(), 2);
    assert!(handle.has_sidx());
}

#[test]
fn ffi_builder_invalid_compression() {
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    assert!(!builder.add_chunk(b"CHAP", b"data", 99, 1, 0, 0, "", -1, ""));
}

#[test]
fn ffi_builder_invalid_markup() {
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    assert!(!builder.add_chunk(b"CHAP", b"data", 0, 1, 99, 0, "", -1, ""));
}

#[test]
fn ffi_builder_empty() {
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    assert!(builder.finalize());
    let output = builder.get_result();
    let handle = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    assert_eq!(handle.chunk_count(), 0);
}

#[test]
fn ffi_builder_with_alt_text() {
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    builder.add_chunk(
        b"CHAP",
        b"chapter content",
        0,
        1,
        0,
        0,
        "A chapter image",
        -1,
        "",
    );
    assert!(builder.finalize());
    let output = builder.get_result();
    let handle = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    let mut buffer = diplomat_runtime::rust_interop::RustWriteVec::with_capacity(1024);
    assert!(handle.get_toc(unsafe { buffer.borrow_mut() }).is_ok());
    let toc = std::str::from_utf8(buffer.borrow().as_bytes()).unwrap();
    assert!(toc.contains("\"alt_text\":\"A chapter image\""));
}

#[test]
fn ffi_builder_with_font_embedding() {
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    builder.add_chunk(
        b"FONT",
        b"font data",
        0,
        1,
        0,
        0,
        "",
        0,
        "https://example.com/license",
    );
    assert!(builder.finalize());
    let output = builder.get_result();
    let handle = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    let mut buffer = diplomat_runtime::rust_interop::RustWriteVec::with_capacity(1024);
    assert!(handle.get_toc(unsafe { buffer.borrow_mut() }).is_ok());
    let toc = std::str::from_utf8(buffer.borrow().as_bytes()).unwrap();
    assert!(toc.contains("\"font_embedding\":0"));
    assert!(toc.contains("\"font_license_url\":\"https://example.com/license\""));
}

#[test]
fn ffi_builder_with_cover_type() {
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    builder.add_chunk(b"COVR", b"cover image", 0, 1, 0, 1, "", -1, "");
    assert!(builder.finalize());
    let output = builder.get_result();
    let handle = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    let mut buffer = diplomat_runtime::rust_interop::RustWriteVec::with_capacity(1024);
    assert!(handle.get_toc(unsafe { buffer.borrow_mut() }).is_ok());
    let toc = std::str::from_utf8(buffer.borrow().as_bytes()).unwrap();
    assert!(toc.contains("\"cover_type\":1"));
}

#[test]
fn ffi_builder_set_extra() {
    // Build a valid extra entry for annotations
    let anno_body = honzo_chunks::extra::anno::build_anno(&[]).unwrap();
    let mut extra = Vec::new();
    extra.extend_from_slice(b"DATA");
    let ns = honzo_chunks::extra::ANNO_NAMESPACE;
    let ns_len = ns.len() as u16;
    extra.extend_from_slice(&ns_len.to_le_bytes());
    extra.extend_from_slice(ns.as_bytes());
    let body_len = anno_body.len() as u32;
    extra.extend_from_slice(&body_len.to_le_bytes());
    extra.extend_from_slice(&anno_body);

    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    builder.add_chunk(b"CHAP", b"content", 0, 1, 0, 0, "", -1, "");
    assert!(builder.set_extra(&extra));
    assert!(builder.finalize());
    let output = builder.get_result();
    let handle = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    let mut buffer = diplomat_runtime::rust_interop::RustWriteVec::with_capacity(256);
    let result = handle.get_annotations(unsafe { buffer.borrow_mut() });
    assert!(result.is_ok());
}

#[test]
fn ffi_builder_auto_covt_disabled() {
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    assert!(builder.set_auto_covt(false));
    builder.add_chunk(b"CHAP", b"content", 0, 1, 0, 0, "", -1, "");
    assert!(builder.finalize());
    let output = builder.get_result();
    // should not auto-generate COVT since there's no COVR
    assert!(honzo_c::ffi::HonzoHandle::parse(output, 1).is_some());
}

#[test]
fn ffi_builder_set_layout() {
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    assert!(builder.set_layout(1)); // Fixed
    builder.add_chunk(b"CHAP", b"content", 0, 1, 0, 0, "", -1, "");
    assert!(builder.finalize());
    let output = builder.get_result();
    let handle = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    assert_eq!(handle.layout_mode(), 1);
}

#[test]
fn ffi_builder_set_flags() {
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    assert!(builder.set_flags(0x20)); // has_sidx flag
    builder.add_chunk(b"CHAP", b"content", 0, 1, 0, 0, "", -1, "");
    assert!(builder.finalize());
    let output = builder.get_result();
    let handle = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    assert!(handle.has_sidx());
}

#[test]
fn ffi_builder_min_reader_version() {
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    assert!(builder.set_min_reader_version(2));
    builder.add_chunk(b"CHAP", b"content", 0, 1, 0, 0, "", -1, "");
    assert!(builder.finalize());
    let output = builder.get_result();
    // Should fail to parse with reader_version 1
    assert!(honzo_c::ffi::HonzoHandle::parse(output, 1).is_none());
    // Should parse with reader_version 2
    assert!(honzo_c::ffi::HonzoHandle::parse(output, 2).is_some());
}

#[test]
fn ffi_builder_invalid_layout() {
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    assert!(!builder.set_layout(99));
}

#[test]
fn ffi_builder_add_pmap_entry() {
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    builder.add_chunk(b"CHAP", b"content", 0, 1, 0, 0, "", -1, "");
    assert!(builder.add_pmap_entry(1, 0, 0));
    assert!(builder.finalize());
    let output = builder.get_result();
    assert!(honzo_c::ffi::HonzoHandle::parse(output, 1).is_some());
}

#[test]
fn ffi_header_accessors() {
    let handle = honzo_c::ffi::HonzoHandle::parse(&fixture("novel.hzo"), 1).unwrap();
    assert_eq!(handle.version_major(), 1);
    assert_eq!(handle.version_minor(), 0);
    assert_eq!(handle.min_reader_version(), 1);
    assert_ne!(handle.flags(), 0);
    assert_ne!(handle.toc_size(), 0);
    assert_ne!(handle.data_size(), 0);
    assert_eq!(handle.layout_mode(), 0);
    assert!(!handle.has_drm());
    assert!(handle.has_sidx());
    assert_eq!(handle.chunk_count(), 7);
}

#[test]
fn ffi_get_extra() {
    let handle = honzo_c::ffi::HonzoHandle::parse(&fixture("novel.hzo"), 1).unwrap();
    let extra = handle.get_extra();
    // novel.hzo has no extra data
    assert!(extra.is_empty());
}

#[test]
fn ffi_chunk_roundtrip() {
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    builder.add_chunk(b"CHAP", b"hello world", 0, 1, 0, 0, "", -1, "");
    assert!(builder.finalize());
    let output = builder.get_result();
    let mut handle = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    let chunk = handle.get_chunk(0).unwrap(); // CHAP is index 0, SIDX gets appended last
    assert_eq!(std::str::from_utf8(chunk).unwrap(), "hello world");
}

#[test]
fn ffi_get_extra_with_content() {
    let meta = rmp_serde::to_vec(&honzo_io::HonzoMeta::default()).unwrap();
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    builder.add_chunk(b"CHAP", b"extra test", 0, 1, 0, 0, "", -1, "");
    builder.add_extra_entry(b"XTRA", "com.example.test", b"hello");
    builder.set_meta(&meta);
    assert!(builder.finalize());
    let output = builder.get_result();
    let handle = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    let extra = handle.get_extra();
    assert!(!extra.is_empty(), "extra should contain our entry");
    let entries = honzo_io::parse_extra(extra).unwrap();
    assert!(honzo_io::find_extra(&entries, "com.example.test").is_some());
}

#[test]
fn ffi_get_annotations() {
    let meta = rmp_serde::to_vec(&honzo_io::HonzoMeta::default()).unwrap();
    let annos = vec![honzo_chunks::extra::anno::Annotation {
        chunk_id: 0,
        offset: 5,
        length: 10,
        r#type: "highlight".to_string(),
        note: Some("test".to_string()),
        color: None,
    }];
    let anno_body = honzo_chunks::extra::anno::build_anno(&annos).unwrap();

    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    builder.add_chunk(b"CHAP", b"anno test", 0, 1, 0, 0, "", -1, "");
    builder.add_annotation(&anno_body);
    builder.set_meta(&meta);
    assert!(builder.finalize());
    let output = builder.get_result();

    let handle = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    let mut buffer = diplomat_runtime::rust_interop::RustWriteVec::with_capacity(256);
    let result = handle.get_annotations(unsafe { buffer.borrow_mut() });
    assert!(result.is_ok(), "get_annotations should succeed");
    let json = std::str::from_utf8(buffer.borrow().as_bytes()).unwrap();
    assert!(json.contains("\"chunk_id\":0"));
    assert!(json.contains("\"type\":\"highlight\""));
    assert!(json.contains("\"note\":\"test\""));
}

#[test]
fn ffi_get_sync_cues() {
    let meta = rmp_serde::to_vec(&honzo_io::HonzoMeta::default()).unwrap();
    let cues = vec![honzo_chunks::extra::sync::SyncCue {
        sync_type: honzo_chunks::extra::sync::SyncType::Audio,
        chunk_id: 0,
        offset: 100,
        timestamp_ms: 5000,
        media_id: None,
        duration_ms: None,
        metadata: None,
    }];
    let sync_body = honzo_chunks::extra::sync::build_sync(&cues).unwrap();

    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    builder.add_chunk(b"CHAP", b"sync test", 0, 1, 0, 0, "", -1, "");
    builder.add_sync_cue(&sync_body);
    builder.set_meta(&meta);
    assert!(builder.finalize());
    let output = builder.get_result();

    let handle = honzo_c::ffi::HonzoHandle::parse(output, 1).unwrap();
    let mut buffer = diplomat_runtime::rust_interop::RustWriteVec::with_capacity(256);
    let result = handle.get_sync_cues(unsafe { buffer.borrow_mut() });
    assert!(result.is_ok(), "get_sync_cues should succeed");
    let json = std::str::from_utf8(buffer.borrow().as_bytes()).unwrap();
    assert!(json.contains("\"chunk_id\":0"));
    assert!(json.contains("\"offset\":100"));
    assert!(json.contains("\"timestamp_ms\":5000"));
}

// HonzoFileReader tests (file-backed streaming)
fn temp_fixture_path(test_name: &str, fixture_name: &str) -> std::path::PathBuf {
    let data = fixture(fixture_name);
    let path = std::env::temp_dir().join(format!("honzo_c_{}_{}", test_name, fixture_name));
    std::fs::write(&path, &data).unwrap();
    path
}

#[test]
fn ffi_filereader_open_novel() {
    let path = temp_fixture_path("open_novel", "novel.hzo");
    let reader = honzo_c::ffi::HonzoFileReader::open(path.to_str().unwrap(), 1).unwrap();
    assert_eq!(reader.chunk_count(), 7);
    std::fs::remove_file(&path).ok();
}

#[test]
fn ffi_filereader_chunks_match_handle() {
    let path = temp_fixture_path("chunks_match_handle", "novel.hzo");
    let mut reader = honzo_c::ffi::HonzoFileReader::open(path.to_str().unwrap(), 1).unwrap();
    let mut handle = honzo_c::ffi::HonzoHandle::parse(&fixture("novel.hzo"), 1).unwrap();

    assert_eq!(reader.chunk_count(), handle.chunk_count());

    for i in 0..reader.chunk_count() {
        let file_chunk = reader.get_chunk(i).unwrap();
        let handle_chunk = handle.get_chunk(i).unwrap();
        assert_eq!(file_chunk, handle_chunk, "chunk {} mismatch", i);
    }

    std::fs::remove_file(&path).ok();
}

#[test]
fn ffi_filereader_get_meta() {
    let path = temp_fixture_path("get_meta", "novel.hzo");
    let mut reader = honzo_c::ffi::HonzoFileReader::open(path.to_str().unwrap(), 1).unwrap();
    let mut buffer = diplomat_runtime::rust_interop::RustWriteVec::with_capacity(512);
    let result = reader.get_meta(unsafe { buffer.borrow_mut() });
    assert!(result.is_ok());
    let meta_json = std::str::from_utf8(buffer.borrow().as_bytes()).unwrap();
    assert!(meta_json.contains("\"language\":\"en\""));
    assert!(meta_json.contains("\"title\""));
    std::fs::remove_file(&path).ok();
}

#[test]
fn ffi_filereader_not_found() {
    let result = honzo_c::ffi::HonzoFileReader::open("/nonexistent/path.hzo", 1);
    assert!(result.is_err());
}

#[test]
fn ffi_filereader_bad_magic() {
    let bad = b"NOPE";
    let path = std::env::temp_dir().join("honzo_c_bad_magic.hzo");
    std::fs::write(&path, bad).unwrap();
    let result = honzo_c::ffi::HonzoFileReader::open(path.to_str().unwrap(), 1);
    assert!(result.is_err());
    std::fs::remove_file(&path).ok();
}

#[test]
fn ffi_filereader_open_lz4() {
    let path = temp_fixture_path("open_lz4", "compressed_lz4.hzo");
    let mut reader = honzo_c::ffi::HonzoFileReader::open(path.to_str().unwrap(), 1).unwrap();
    assert!(reader.chunk_count() > 0);
    let chunk = reader.get_chunk(1).unwrap();
    assert!(!chunk.is_empty());
    std::fs::remove_file(&path).ok();
}

#[test]
fn ffi_filereader_minimal() {
    let path = temp_fixture_path("minimal", "minimal.hzo");
    let reader = honzo_c::ffi::HonzoFileReader::open(path.to_str().unwrap(), 1).unwrap();
    assert_eq!(reader.chunk_count(), 0);
    std::fs::remove_file(&path).ok();
}
