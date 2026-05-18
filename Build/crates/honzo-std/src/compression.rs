use honzo_core::{Compression, HonzoError, TocEntry};

pub fn decompress(
    data: &[u8],
    compression: Compression,
    _raw_size: u32,
) -> Result<Vec<u8>, HonzoError> {
    match compression {
        Compression::None => Ok(data.to_vec()),
        Compression::Lz4 => {
            let out =
                lz4_flex::decompress_size_prepended(data).map_err(|_| HonzoError::Truncated)?;
            Ok(out)
        }
    }
}

pub fn verify_crc32(data: &[u8], expected: u32, chunk_id: u32) -> Result<(), HonzoError> {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(data);
    let got = hasher.finalize();
    if got != expected {
        return Err(HonzoError::CrcMismatch {
            chunk_id,
            expected,
            got,
        });
    }
    Ok(())
}

pub fn verify_entry_crc32(entry: &TocEntry, data: &[u8]) -> Result<(), HonzoError> {
    if entry.chunk_type != *b"CHAP" {
        return Ok(());
    }
    verify_crc32(data, entry.crc32, entry.chunk_id)
}
