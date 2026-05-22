use crate::common::{corpus, fixture};
use honzo_core::HonzoError;
use honzo_io::{compute_reading_time, generate_covt, HonzoMeta, HonzoReader, HonzoStream};

#[test]
fn streams_all_chapters_in_novel_fixture() {
    let data = fixture("novel.hzo");
    let mut stream = HonzoStream::open(std::io::Cursor::new(&data), 1).unwrap();
    let chapters: Vec<_> = stream.chapters().collect::<Result<Vec<_>, _>>().unwrap();

    assert_eq!(chapters.len(), 3);
    assert!(chapters.iter().all(|chapter| !chapter.is_empty()));
}

#[test]
fn decompresses_lz4_fixture() {
    let lz4_data = fixture("compressed_lz4.hzo");

    let mut lz4 = HonzoStream::open(std::io::Cursor::new(&lz4_data), 1).unwrap();
    let lz4_entries = lz4.toc_owned();
    let lz4_entry = lz4_entries[1];

    assert!(lz4.read_chunk(&lz4_entry).unwrap().len() > lz4_entry.size_compressed as usize);
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
    let crc_entry = crc_entries[0];
    let encrypted_entry = encrypted_entries[0];

    assert_eq!(crc_entry.chunk_type, *b"CHAP");
    assert!(matches!(
        crc_stream.read_chunk(&crc_entry),
        Err(HonzoError::CrcMismatch { .. })
    ));
    assert!(matches!(
        encrypted_stream.read_chunk(&encrypted_entry),
        Err(HonzoError::EncryptedChunk { .. })
    ));
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
