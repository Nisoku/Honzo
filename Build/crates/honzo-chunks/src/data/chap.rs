use honzo_core::HonzoError;

pub const CHAP_TAG: [u8; 4] = *b"CHAP";
pub const NOTE_TAG: [u8; 4] = *b"NOTE";

pub fn is_chap_tag(tag: &[u8; 4]) -> bool {
    *tag == CHAP_TAG
}

pub fn is_note_tag(tag: &[u8; 4]) -> bool {
    *tag == NOTE_TAG
}

pub fn is_text_chunk(tag: &[u8; 4]) -> bool {
    is_chap_tag(tag) || is_note_tag(tag)
}

pub fn chunk_name(tag: &[u8; 4]) -> Option<&'static str> {
    match *tag {
        CHAP_TAG => Some("chapter"),
        NOTE_TAG => Some("note"),
        _ => None,
    }
}

pub fn validate_text_chunk(bytes: &[u8]) -> Result<&str, HonzoError> {
    core::str::from_utf8(bytes).map_err(|_| HonzoError::Truncated)
}
