use crate::common::corpus;
use honzo_core::{HonzoError, HonzoParser};
use honzo_std::parse_extra;

#[test]
fn rejects_truncated_and_invalid_corpus_files() {
    type Check = fn(HonzoError) -> bool;
    let cases: [(&str, Check); 4] = [
        ("truncated_head.hzo", |err: HonzoError| {
            matches!(err, HonzoError::BufferTooShort)
        }),
        ("truncated_toc.hzo", |err: HonzoError| {
            matches!(err, HonzoError::BufferTooShort | HonzoError::Truncated)
        }),
        ("truncated_data.hzo", |err: HonzoError| {
            matches!(err, HonzoError::BufferTooShort | HonzoError::Truncated)
        }),
        ("unknown_chunk_type.hzo", |err: HonzoError| {
            matches!(err, HonzoError::InvalidChunkType)
        }),
    ];

    for (name, check) in cases {
        let data = corpus(name);
        let result = HonzoParser::new(&data, 1);
        assert!(
            result.err().is_some_and(check),
            "unexpected parse result for {name}"
        );
    }
}

#[test]
fn parses_empty_sections_and_large_alt_text() {
    let empty_extra_data = corpus("empty_extra.hzo");
    let empty_meta_data = corpus("empty_meta.hzo");
    let large_alt_data = corpus("large_alt_text.hzo");
    let empty_extra = HonzoParser::new(&empty_extra_data, 1).unwrap();
    let empty_meta = HonzoParser::new(&empty_meta_data, 1).unwrap();
    let large_alt = HonzoParser::new(&large_alt_data, 1).unwrap();

    assert!(empty_extra.extra_bytes().unwrap().is_empty());
    assert!(empty_meta.meta_bytes().unwrap().is_empty());
    let entry = large_alt.find_chunk(b"IMG_").unwrap();
    assert_eq!(entry.alt_text.unwrap().len(), 500);
}

#[test]
fn keeps_unknown_extra_namespaces_intact() {
    let data = corpus("unknown_extra_ns.hzo");
    let parser = HonzoParser::new(&data, 1).unwrap();
    let entries = parse_extra(parser.extra_bytes().unwrap()).unwrap();

    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].namespace, "com.unknown.thing");
}

#[test]
fn zero_chunks_file_is_still_parseable() {
    let data = corpus("zero_chunks.hzo");
    let parser = HonzoParser::new(&data, 1).unwrap();

    assert_eq!(parser.head().chunk_count, 0);
    assert_eq!(parser.toc_entries().count(), 0);
}
