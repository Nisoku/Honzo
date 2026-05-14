use honzo_core::{Compression, HonzoError, TocEntry};
use std::io::Read;

pub fn decompress(
    data: &[u8],
    compression: Compression,
    raw_size: u32,
) -> Result<Vec<u8>, HonzoError> {
    match compression {
        Compression::None => Ok(data.to_vec()),
        Compression::Zlib => {
            let mut decoder = flate2::read::ZlibDecoder::new(std::io::Cursor::new(data));
            let mut out = Vec::with_capacity(raw_size as usize);
            decoder.read_to_end(&mut out).map_err(|_| HonzoError::Truncated)?;
            Ok(out)
        }
        Compression::Zstd => {
            let mut decoder =
                zstd::stream::Decoder::new(std::io::Cursor::new(data)).map_err(|_| HonzoError::Truncated)?;
            let mut out = Vec::with_capacity(raw_size as usize);
            decoder.read_to_end(&mut out).map_err(|_| HonzoError::Truncated)?;
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
