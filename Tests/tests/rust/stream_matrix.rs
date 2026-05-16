use crate::common::{corpus, fixture};
use honzo_core::HonzoError;
use honzo_std::{compute_reading_time, generate_covt, HonzoMeta, HonzoStream, HonzoReader};

#[test]
fn streams_all_chapters_in_novel_fixture() {
    let data = fixture("novel.hzo");
    let mut stream = HonzoStream::open(std::io::Cursor::new(&data), 1).unwrap();
    let chapters: Vec<_> = stream.chapters().collect::<Result<Vec<_>, _>>().unwrap();

    assert_eq!(chapters.len(), 3);
    assert!(chapters.iter().all(|chapter| !chapter.is_empty()));
}

#[test]
fn decompresses_zlib_and_zstd_fixtures() {
    let zlib_data = fixture("compressed_zlib.hzo");
    let zstd_data = fixture("compressed_zstd.hzo");

    let mut zlib = HonzoStream::open(std::io::Cursor::new(&zlib_data), 1).unwrap();
    let mut zstd = HonzoStream::open(std::io::Cursor::new(&zstd_data), 1).unwrap();

    let zlib_entries = zlib.toc_owned();
    let zstd_entries = zstd.toc_owned();
    let zlib_entry = zlib_entries[1].clone();
    let zstd_entry = zstd_entries[1].clone();

    assert!(zlib.read_chunk(&zlib_entry).unwrap().len() > zlib_entry.size_compressed as usize);
    assert!(zstd.read_chunk(&zstd_entry).unwrap().len() > zstd_entry.size_compressed as usize);
}

#[test]
fn exposes_meta_and_extra_bytes() {
    let data = fixture("with_anno.hzo");
    let mut stream = HonzoStream::open(std::io::Cursor::new(&data), 1).unwrap();
    let parser = honzo_core::HonzoParser::new(&data, 1).unwrap();

    assert_eq!(stream.meta_bytes().unwrap(), parser.meta_bytes().unwrap());
    assert_eq!(stream.extra_bytes().unwrap(), parser.extra_bytes().unwrap());
}

#[test]
fn verifies_crc_and_encrypted_chunk_errors() {
    let crc_data = corpus("crc_mismatch.hzo");
    let encrypted_data = corpus("encrypted_chunk.hzo");

    let mut crc_stream = HonzoStream::open(std::io::Cursor::new(&crc_data), 1).unwrap();
    let mut encrypted_stream = HonzoStream::open(std::io::Cursor::new(&encrypted_data), 1).unwrap();

    let crc_entries = crc_stream.toc_owned();
    let encrypted_entries = encrypted_stream.toc_owned();
    let crc_entry = crc_entries[0].clone();
    let encrypted_entry = encrypted_entries[0].clone();

    assert_eq!(crc_entry.chunk_type, *b"CHAP");
    assert!(matches!(crc_stream.read_chunk(&crc_entry), Err(HonzoError::CrcMismatch { .. })));
    assert!(matches!(encrypted_stream.read_chunk(&encrypted_entry), Err(HonzoError::EncryptedChunk { .. })));
}

#[test]
fn parses_metadata_and_utils() {
    let data = fixture("multilang.hzo");
    let mut stream = HonzoStream::open(std::io::Cursor::new(&data), 1).unwrap();
    let meta: HonzoMeta = rmp_serde::from_slice(&stream.meta_bytes().unwrap()).unwrap();

    assert!(meta.title.as_ref().unwrap().contains_key("en"));
    assert!(meta.title.as_ref().unwrap().contains_key("ja"));
    assert!(meta.title.as_ref().unwrap().contains_key("ar"));
    assert_eq!(compute_reading_time(0), 1);
    assert_eq!(compute_reading_time(476), 2);
}

#[test]
fn generates_cover_thumbnail_and_reads_via_reader() {
    let data = fixture("novel.hzo");
    let parser = honzo_core::HonzoParser::new(&data, 1).unwrap();
    let cover = parser.find_chunk(b"COVR").unwrap();
    let cover_bytes = parser.chunk_bytes(&cover).unwrap();
    let covt = generate_covt(cover_bytes).unwrap();

    assert!(!covt.is_empty());
    let reader = HonzoReader::new(&data, 1).unwrap();
    assert_eq!(reader.head().chunk_count, 5);
}