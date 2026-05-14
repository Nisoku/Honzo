use honzo_std::*;

fn fix(name: &str) -> Vec<u8> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::read(root.join("fixtures").join(name)).unwrap()
}

fn cor(name: &str) -> Vec<u8> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::read(root.join("corpus").join(name)).unwrap()
}

#[test]
fn stream_minimal() {
    let d = fix("minimal.hzo");
    let s = HonzoStream::open(std::io::Cursor::new(&d), 1).unwrap();
    assert_eq!(s.head().chunk_count, 0);
}

#[test]
fn stream_novel() {
    let d = fix("novel.hzo");
    let mut s = HonzoStream::open(std::io::Cursor::new(&d), 1).unwrap();
    assert_eq!(s.head().chunk_count, 5);
    assert_eq!(s.toc().len(), 5);
    let toc = s.toc_owned();
    for entry in &toc {
        if entry.chunk_type == *b"CHAP" {
            let bytes = s.read_chunk(entry).unwrap();
            assert!(!bytes.is_empty());
        }
    }
}

#[test]
fn reader_decompress_zlib() {
    let d = fix("compressed_zlib.hzo");
    let mut s = HonzoStream::open(std::io::Cursor::new(&d), 1).unwrap();
    let toc = s.toc_owned();
    let chap = toc[1].clone();
    assert_eq!(chap.compression, Compression::Zlib);
    let bytes = s.read_chunk(&chap).unwrap();
    assert!(!bytes.is_empty());
    assert!(bytes.len() > chap.size_compressed as usize);
}

#[test]
fn reader_decompress_zstd() {
    let d = fix("compressed_zstd.hzo");
    let mut s = HonzoStream::open(std::io::Cursor::new(&d), 1).unwrap();
    let toc = s.toc_owned();
    let chap = toc[1].clone();
    assert_eq!(chap.compression, Compression::Zstd);
    let bytes = s.read_chunk(&chap).unwrap();
    assert!(!bytes.is_empty());
    assert!(bytes.len() > chap.size_compressed as usize);
}

#[test]
fn reader_crc_mismatch() {
    let d = cor("crc_mismatch.hzo");
    let mut s = HonzoStream::open(std::io::Cursor::new(&d), 1).unwrap();
    let chap = s.toc_owned()[0].clone();
    assert!(s.read_chunk(&chap).is_err());
}

#[test]
fn reader_encrypted() {
    let d = cor("encrypted_chunk.hzo");
    let mut s = HonzoStream::open(std::io::Cursor::new(&d), 1).unwrap();
    let chap = s.toc_owned()[0].clone();
    let result = s.read_chunk(&chap);
    assert!(matches!(result, Err(HonzoError::EncryptedChunk { .. })));
}

#[test]
fn parse_meta() {
    let d = fix("novel.hzo");
    let mut s = HonzoStream::open(std::io::Cursor::new(&d), 1).unwrap();
    let meta: HonzoMeta = rmp_serde::from_slice(&s.meta_bytes().unwrap()).unwrap();
    assert!(meta.title.is_some());
    assert!(!meta.authors.is_empty());
    assert_eq!(meta.language, "en");
}

#[test]
fn parse_meta_multilang() {
    let d = fix("multilang.hzo");
    let reader = Reader::new(&d, 1).unwrap();
    let meta: HonzoMeta = rmp_serde::from_slice(reader.meta_bytes().unwrap()).unwrap();
    let title = meta.title.unwrap();
    assert!(title.contains_key("en"));
    assert!(title.contains_key("ja"));
    assert!(title.contains_key("ar"));
}

#[test]
fn parse_extra_entries() {
    let d = fix("with_anno.hzo");
    let p = honzo_core::HonzoParser::new(&d, 1).unwrap();
    let entries = honzo_std::parse_extra(p.extra_bytes().unwrap()).unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(&entries[0].tag, b"ANNO");
    assert_eq!(entries[0].namespace, "org.nisoku.anno");
}

#[test]
fn stream_extra_bytes() {
    let d = fix("with_anno.hzo");
    let mut s = HonzoStream::open(std::io::Cursor::new(&d), 1).unwrap();
    let p = honzo_core::HonzoParser::new(&d, 1).unwrap();
    assert_eq!(s.extra_bytes().unwrap(), p.extra_bytes().unwrap());
}

#[test]
fn chapters_iter() {
    let d = fix("novel.hzo");
    let mut s = HonzoStream::open(std::io::Cursor::new(&d), 1).unwrap();
    let chapters: Vec<_> = s.chapters().collect::<Result<Vec<_>, _>>().unwrap();
    assert_eq!(chapters.len(), 3);
}

#[test]
fn chapters_filter_non_chap() {
    let d = fix("manga.hzo");
    let mut s = HonzoStream::open(std::io::Cursor::new(&d), 1).unwrap();
    let chapters: Vec<_> = s.chapters().collect::<Result<Vec<_>, _>>().unwrap();
    assert_eq!(chapters.len(), 0);
}

#[test]
fn compressed_zstd_roundtrip() {
    let d = fix("compressed_zstd.hzo");
    let mut s = HonzoStream::open(std::io::Cursor::new(&d), 1).unwrap();
    assert_eq!(s.head().chunk_count, 4);
    let toc = s.toc_owned();
    for entry in toc.iter().skip(1).take(3) {
        assert!(!s.read_chunk(entry).unwrap().is_empty());
    }
}

#[test]
fn build_sidx_roundtrip() {
    let chapters = [
        (0u32, "hello world"),
        (1u32, "hello rust"),
        (2u32, "world of rust"),
    ];
    let sidx = build_sidx(&chapters).unwrap();
    let idx: std::collections::BTreeMap<String, Vec<(u32, u32)>> =
        rmp_serde::from_slice(&sidx).unwrap();
    assert!(idx.contains_key("hello"));
    assert!(idx.contains_key("world"));
    assert!(idx.contains_key("rust"));
    assert_eq!(idx["hello"].len(), 2);
}

#[test]
fn generate_covt_thumbnail() {
    let d = fix("novel.hzo");
    let p = honzo_core::HonzoParser::new(&d, 1).unwrap();
    if let Some(entry) = p.find_chunk(b"COVR") {
        let covt = generate_covt(p.chunk_bytes(&entry).unwrap()).unwrap();
        assert!(!covt.is_empty());
    }
}

#[test]
fn compute_reading_time_works() {
    assert_eq!(compute_reading_time(0), 1);
    assert_eq!(compute_reading_time(238), 1);
    assert_eq!(compute_reading_time(239), 2);
    assert_eq!(compute_reading_time(476), 2);
}

#[test]
fn new_uuid_format() {
    let uuid = new_uuid();
    assert_eq!(uuid.len(), 36);
    assert_eq!(uuid.chars().filter(|&c| c == '-').count(), 4);
}

#[test]
fn zero_chunks_stream() {
    let d = cor("zero_chunks.hzo");
    let s = HonzoStream::open(std::io::Cursor::new(&d), 1).unwrap();
    assert_eq!(s.head().chunk_count, 0);
    assert!(s.toc().is_empty());
}

#[test]
fn extra_unknown_namespace() {
    let d = cor("unknown_extra_ns.hzo");
    let mut s = HonzoStream::open(std::io::Cursor::new(&d), 1).unwrap();
    let entries = honzo_std::parse_extra(&s.extra_bytes().unwrap()).unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].namespace, "com.unknown.thing");
    assert!(honzo_std::find_extra(&entries, "com.unknown.thing").is_some());
    assert!(honzo_std::find_extra(&entries, "org.nisoku.anno").is_none());
}
