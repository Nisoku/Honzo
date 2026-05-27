use honzo_core::{FontEmbedding, HonzoError};

pub const FONT_TAG: [u8; 4] = *b"FONT";

pub fn is_font_tag(tag: &[u8; 4]) -> bool {
    *tag == FONT_TAG
}

/// Validate font bytes by checking known magic bytes.
pub fn validate_font(bytes: &[u8]) -> Result<&[u8], HonzoError> {
    if guess_font_format(bytes).is_none() {
        return Err(HonzoError::Truncated);
    }
    Ok(bytes)
}

/// Validate font bytes and map error to u8 code for FFI.
pub fn validate_font_bytes(bytes: &[u8]) -> Result<(), u8> {
    match validate_font(bytes) {
        Ok(..) => Ok(()),
        Err(HonzoError::Truncated) => Err(7),
        Err(..) => Err(255),
    }
}

/// Guess the font format from magic bytes.
pub fn guess_font_format(bytes: &[u8]) -> Option<&'static str> {
    if bytes.len() < 4 {
        return None;
    }
    if &bytes[..4] == b"wOFF" {
        return Some("font/woff");
    }
    if &bytes[..4] == b"wOF2" {
        return Some("font/woff2");
    }
    if &bytes[..4] == b"OTTO" {
        return Some("font/otf");
    }
    if bytes.starts_with(&[0x00, 0x01, 0x00, 0x00])
        || bytes.starts_with(&[0x00, 0x01, 0x00, 0x01])
        || bytes.starts_with(b"true")
        || bytes.starts_with(b"typ1")
    {
        return Some("font/ttf");
    }
    None
}

/// Parse font_embedding and font_license_url from a TOC entry at cursor.
/// Advances cursor past the consumed bytes.
pub fn read_font_toc_fields<'a>(
    buf: &'a [u8],
    cursor: &mut usize,
) -> Result<(FontEmbedding, Option<&'a str>), HonzoError> {
    if *cursor + 3 > buf.len() {
        return Err(HonzoError::Truncated);
    }
    let embedding = FontEmbedding::from_u8(buf[*cursor])?;
    *cursor += 1;
    let url_len = u16::from_le_bytes([buf[*cursor], buf[*cursor + 1]]) as usize;
    *cursor += 2;
    let license_url = if url_len > 0 {
        let end = *cursor + url_len;
        if end > buf.len() {
            return Err(HonzoError::Truncated);
        }
        let s = core::str::from_utf8(&buf[*cursor..end]).map_err(|_| HonzoError::Truncated)?;
        *cursor = end;
        Some(s)
    } else {
        None
    };
    Ok((embedding, license_url))
}

/// Serialize font_embedding and font_license_url into the output buffer.
pub fn write_font_toc_fields(
    out: &mut Vec<u8>,
    embedding: FontEmbedding,
    license_url: Option<&str>,
) {
    out.push(embedding as u8);
    if let Some(url) = license_url {
        out.extend_from_slice(&(url.len() as u16).to_le_bytes());
        out.extend_from_slice(url.as_bytes());
    } else {
        out.extend_from_slice(&0u16.to_le_bytes());
    }
}

pub fn chunk_name() -> &'static str {
    "font"
}
