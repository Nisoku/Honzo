fn match_prefix(bytes: &[u8], magic: &[u8]) -> bool {
    bytes.len() >= magic.len() && bytes[..magic.len()] == *magic
}

pub fn guess_image_mime(bytes: &[u8]) -> Option<&'static str> {
    // PNM (PBM/PGM/PPM/PAM): 2-byte ASCII signature, check first
    if bytes.len() >= 2 && bytes[0] == b'P' && (b'1'..=b'7').contains(&bytes[1]) {
        return Some("image/x-portable-anymap");
    }

    if match_prefix(bytes, &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
        return Some("image/png");
    }

    if match_prefix(bytes, &[0xFF, 0xD8, 0xFF]) {
        return Some("image/jpeg");
    }

    if match_prefix(bytes, b"GIF87a") || match_prefix(bytes, b"GIF89a") {
        return Some("image/gif");
    }

    if match_prefix(bytes, b"BM") {
        return Some("image/bmp");
    }

    if match_prefix(bytes, &[0x49, 0x49, 0x2A, 0x00]) {
        return Some("image/tiff");
    }

    if match_prefix(bytes, &[0x4D, 0x4D, 0x00, 0x2A]) {
        return Some("image/tiff");
    }

    // WebP: RIFF container, check offset 8
    if match_prefix(bytes, b"RIFF") && bytes.len() >= 12 && match_prefix(&bytes[8..], b"WEBP") {
        return Some("image/webp");
    }

    // ICO
    if match_prefix(bytes, &[0x00, 0x00, 0x01, 0x00]) {
        return Some("image/x-icon");
    }

    None
}
