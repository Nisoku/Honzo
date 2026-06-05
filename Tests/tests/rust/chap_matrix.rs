use honzo_chunks::data::chap::{
    chunk_name, is_chap_tag, is_note_tag, is_text_chunk, validate_text_chunk, CHAP_TAG, NOTE_TAG,
};

#[test]
fn recognizes_chap_and_note_tags() {
    assert!(is_chap_tag(&CHAP_TAG));
    assert!(is_note_tag(&NOTE_TAG));
    assert!(is_text_chunk(&CHAP_TAG));
    assert!(is_text_chunk(&NOTE_TAG));
    assert_eq!(chunk_name(&CHAP_TAG), Some("chapter"));
    assert_eq!(chunk_name(&NOTE_TAG), Some("note"));
    assert_eq!(chunk_name(b"IMG_"), None);
}

#[test]
fn validates_utf8_text_chunks() {
    let text = validate_text_chunk(b"chapter text").unwrap();
    assert_eq!(text, "chapter text");
    assert!(validate_text_chunk(&[0xff, 0xfe]).is_err());
}

#[test]
fn non_text_tags_are_rejected() {
    assert!(!is_text_chunk(b"IMG_"));
    assert!(!is_chap_tag(b"FOO "));
    assert_eq!(chunk_name(b"IMG_"), None);
}

#[test]
fn validate_multibyte_utf8() {
    let sample = "こんにちは"; // multibyte UTF-8
    let bytes = sample.as_bytes();
    let decoded = validate_text_chunk(bytes).unwrap();
    assert_eq!(decoded, sample);
}
