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
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let data = fixture(fixture_name);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("honzo_c_{}_{}_{}", test_name, fixture_name, id));
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
    assert!(matches!(
        result,
        Err(honzo_c::ffi::HonzoErrorCode::FileNotFound)
    ));
}

#[test]
fn ffi_filereader_bad_magic() {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let bad = b"NOPE";
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("honzo_c_bad_magic_{}.hzo", id));
    std::fs::write(&path, bad).unwrap();
    let result = honzo_c::ffi::HonzoFileReader::open(path.to_str().unwrap(), 1);
    assert!(matches!(
        result,
        Err(honzo_c::ffi::HonzoErrorCode::InvalidMagic)
    ));
    std::fs::remove_file(&path).ok();
}

#[test]
fn ffi_filereader_open_lz4() {
    let path = temp_fixture_path("open_lz4", "compressed_lz4.hzo");
    let mut reader = honzo_c::ffi::HonzoFileReader::open(path.to_str().unwrap(), 1).unwrap();
    assert!(reader.chunk_count() > 1);
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

#[test]
fn ffi_filereader_chunk_types_novel() {
    let path = temp_fixture_path("chunk_types_novel", "novel.hzo");
    let reader = honzo_c::ffi::HonzoFileReader::open(path.to_str().unwrap(), 1).unwrap();
    assert_eq!(reader.chunk_count(), 7);

    // novel.hzo: COVR + 3 CHAP + CSS + auto-SIDX + auto-COVT
    let covr = reader.get_chunk_type(0);
    assert_eq!(covr, u32::from_le_bytes(*b"COVR"), "chunk 0 tag");

    let chap = reader.get_chunk_type(1);
    assert_eq!(chap, u32::from_le_bytes(*b"CHAP"), "chunk 1 tag");

    let chap2 = reader.get_chunk_type(2);
    assert_eq!(chap2, u32::from_le_bytes(*b"CHAP"), "chunk 2 tag");

    let sidx = reader.get_chunk_type(5);
    assert_eq!(sidx, u32::from_le_bytes(*b"SIDX"), "chunk 5 tag");

    std::fs::remove_file(&path).ok();
}

#[test]
fn ffi_filereader_content_types_novel() {
    let path = temp_fixture_path("content_types_novel", "novel.hzo");
    let reader = honzo_c::ffi::HonzoFileReader::open(path.to_str().unwrap(), 1).unwrap();

    // CHAP chunks are Markdown in novel.hzo (kind=1, value=0)
    for i in 1..=3 {
        assert_eq!(
            reader.get_chunk_content_type_kind(i),
            1,
            "chunk {} content_type_kind",
            i
        );
        assert_eq!(
            reader.get_chunk_content_type_value(i),
            0,
            "chunk {} content_type_value",
            i
        );
    }

    // COVR, CSS, SIDX, COVT have kind=1, value=0
    for i in [0u32, 4, 5, 6] {
        assert_eq!(
            reader.get_chunk_content_type_kind(i),
            1,
            "chunk {} content_type_kind",
            i
        );
        assert_eq!(
            reader.get_chunk_content_type_value(i),
            0,
            "chunk {} content_type_value",
            i
        );
    }

    std::fs::remove_file(&path).ok();
}

#[test]
fn ffi_filereader_content_types_textbook() {
    // textbook.hzo has CHAP chunks with MarkupType::Html (value=1)
    let path = temp_fixture_path("content_types_textbook", "textbook.hzo");
    let reader = honzo_c::ffi::HonzoFileReader::open(path.to_str().unwrap(), 1).unwrap();

    // MATH chunk: kind=2, value=0 (MathML)
    // auto-SIDX is last chunk
    let chap1_index = 0;
    let chap2_index = 1;
    let math_index = 2;

    assert_eq!(reader.get_chunk_content_type_kind(math_index), 2);
    assert_eq!(reader.get_chunk_content_type_value(math_index), 0);

    // CHAP chunks are HTML (kind=1, value=1)
    assert_eq!(reader.get_chunk_content_type_kind(chap1_index), 1);
    assert_eq!(reader.get_chunk_content_type_value(chap1_index), 1);
    assert_eq!(reader.get_chunk_content_type_kind(chap2_index), 1);
    assert_eq!(reader.get_chunk_content_type_value(chap2_index), 1);

    // Verify chunk tags
    assert_eq!(
        reader.get_chunk_type(chap1_index),
        u32::from_le_bytes(*b"CHAP")
    );
    assert_eq!(
        reader.get_chunk_type(chap2_index),
        u32::from_le_bytes(*b"CHAP")
    );
    assert_eq!(
        reader.get_chunk_type(math_index),
        u32::from_le_bytes(*b"MATH")
    );

    std::fs::remove_file(&path).ok();
}

#[test]
fn ffi_filereader_chunk_type_out_of_range() {
    let path = temp_fixture_path("type_oob", "novel.hzo");
    let reader = honzo_c::ffi::HonzoFileReader::open(path.to_str().unwrap(), 1).unwrap();
    // Out-of-range indices return 0 / 0 / 0
    assert_eq!(reader.get_chunk_type(99), 0);
    assert_eq!(reader.get_chunk_content_type_kind(99), 0);
    assert_eq!(reader.get_chunk_content_type_value(99), 0);
    std::fs::remove_file(&path).ok();
}

#[test]
fn ffi_filereader_chunk_alt_text_queen_victoria() {
    let path = temp_fixture_path("chunk_alt_text_qv", "lytton-strachey_queen-victoria.hzo");
    let reader = honzo_c::ffi::HonzoFileReader::open(path.to_str().unwrap(), 1).unwrap();

    let chap0 = reader.get_chunk_alt_text(7);
    assert_eq!(chap0, Some("Titlepage"));

    let chap1 = reader.get_chunk_alt_text(8);
    assert_eq!(chap1, Some("Imprint"));

    let chap2 = reader.get_chunk_alt_text(9);
    assert_eq!(chap2, Some("Foreword"));

    let chap3 = reader.get_chunk_alt_text(10);
    assert_eq!(chap3, Some("Queen Victoria"));

    let chap4 = reader.get_chunk_alt_text(11);
    assert_eq!(chap4, Some("I: Antecedents"));

    // SIDX has no alt_text (auto-generated chunk)
    let sidx = reader.get_chunk_alt_text(24);
    assert!(sidx.is_none() || sidx.unwrap().is_empty());

    // Out-of-range returns None
    let oob = reader.get_chunk_alt_text(999);
    assert!(oob.is_none());

    std::fs::remove_file(&path).ok();
}

#[test]
fn ffi_filereader_chunk_alt_text_novel() {
    let path = temp_fixture_path("chunk_alt_text_novel", "novel.hzo");
    let reader = honzo_c::ffi::HonzoFileReader::open(path.to_str().unwrap(), 1).unwrap();

    // novel.hzo was built without alt_text on any chunk
    for i in 0..reader.chunk_count() {
        let alt = reader.get_chunk_alt_text(i);
        assert!(alt.is_none() || alt.unwrap().is_empty());
    }

    std::fs::remove_file(&path).ok();
}

#[test]
fn ffi_filereader_chunk_alt_text_roundtrip() {
    use std::sync::atomic::{AtomicU64, Ordering};
    static ROUNDTRIP_COUNTER: AtomicU64 = AtomicU64::new(0);

    // Build a file with known alt_text via the builder
    let mut builder = honzo_c::ffi::HonzoBuilderHandle::new();
    builder.add_chunk(b"CHAP", b"chapter one", 0, 1, 0, 0, "Chapter 1", -1, "");
    builder.add_chunk(b"CHAP", b"chapter two", 0, 1, 0, 0, "Chapter 2", -1, "");
    builder.add_chunk(b"IMG_", b"\xff\xd8\xff", 0, 1, 0, 0, "A test image", -1, "");
    assert!(builder.finalize());

    // Write to temp file
    let id = ROUNDTRIP_COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("honzo_c_alt_text_roundtrip_{}.hzo", id));
    std::fs::write(&path, builder.get_result()).unwrap();
    // Open with HonzoFileReader and verify alt_text
    let reader = honzo_c::ffi::HonzoFileReader::open(path.to_str().unwrap(), 1).unwrap();

    // The builder may add auto-generated chunks (SIDX, COVT); search for
    // our explicitly-added chunks by checking their alt_text exists.
    let mut found_chap1 = false;
    let mut found_chap2 = false;
    let mut found_img = false;
    for i in 0..reader.chunk_count() {
        match reader.get_chunk_alt_text(i) {
            Some("Chapter 1") => found_chap1 = true,
            Some("Chapter 2") => found_chap2 = true,
            Some("A test image") => found_img = true,
            _ => {}
        }
    }
    assert!(found_chap1, "Chapter 1 alt_text not found");
    assert!(found_chap2, "Chapter 2 alt_text not found");
    assert!(found_img, "A test image alt_text not found");

    std::fs::remove_file(&path).ok();
}
