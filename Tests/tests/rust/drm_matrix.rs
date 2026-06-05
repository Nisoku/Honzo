use honzo_chunks::extra::drm::{self, build_drm, parse_drm, DrmEnvelope};
use honzo_core::{Compression, CoverType, HonzoError, HonzoParser, MarkupType};
use honzo_io::{DrmConfig, HonzoBuilder, HonzoReader, HonzoStream};
use x25519_dalek::{PublicKey, StaticSecret};

/// Generate a test X25519 key pair. Returns (public_key, private_key).
fn generate_test_keypair() -> ([u8; 32], [u8; 32]) {
    let secret = StaticSecret::random();
    let public = PublicKey::from(&secret);
    (public.to_bytes(), secret.to_bytes())
}

fn build_drm_file(encrypt_ids: &[u32]) -> (Vec<u8>, [u8; 32]) {
    let (pub_key, priv_key) = generate_test_keypair();

    let meta = rmp_serde::to_vec(&honzo_io::HonzoMeta::default()).unwrap();
    let builder = HonzoBuilder::new()
        .set_meta(&meta)
        .set_drm_config(DrmConfig {
            encrypt_chunk_ids: encrypt_ids.to_vec(),
            recipient_public_key: pub_key.to_vec(),
            license_url: Some("https://example.com/license".to_string()),
            expires_at: Some(1893456000),
        })
        .add_chunk(
            *b"CHAP",
            b"Hello, World!",
            Compression::Lz4,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"CHAP",
            b"Chapter two content here",
            Compression::Lz4,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        );

    let data = builder.finalize().unwrap();
    (data, priv_key)
}

#[test]
fn drm_flag_is_set() {
    let (data, _) = build_drm_file(&[0]);
    let parser = HonzoParser::new(&data, 1).unwrap();
    assert!(parser.head().has_drm());
}

#[test]
fn drm_envelope_is_in_extra() {
    let (data, _) = build_drm_file(&[0]);
    let parser = HonzoParser::new(&data, 1).unwrap();
    let extra = parser.extra_bytes().unwrap();
    let entries = honzo_io::parse_extra(extra).unwrap();
    let entry = honzo_io::find_extra(&entries, drm::NAMESPACE).unwrap();
    let envelope = drm::parse_drm(&entry.body).unwrap();
    assert_eq!(envelope.scheme, "AES-256-GCM+X25519");
    assert_eq!(envelope.encrypted_chunks, vec![0]);
    assert!(envelope.license_url.is_some());
    assert!(envelope.expires_at.is_some());
    assert!(!envelope.key_envelope.is_empty());
}

#[test]
fn encrypted_chunk_fails_without_key() {
    let (data, _) = build_drm_file(&[0]);
    let reader = HonzoReader::new(&data, 1).unwrap();
    let toc = reader.toc();
    assert!(toc[0].is_encrypted());
    assert!(!toc[1].is_encrypted());

    match reader.chunk_bytes(&toc[0]) {
        Err(HonzoError::EncryptedChunk { chunk_id }) => assert_eq!(chunk_id, 0),
        _ => panic!("expected EncryptedChunk error"),
    }
    // Unencrypted chunk should still work
    reader.chunk_bytes(&toc[1]).unwrap();
}

#[test]
fn encrypted_chunk_decrypts_with_key() {
    let (data, priv_key) = build_drm_file(&[0]);
    let reader = HonzoReader::with_private_key(&data, 1, &priv_key).unwrap();
    let toc = reader.toc();

    let chunk0 = reader.chunk_bytes(&toc[0]).unwrap();
    assert_eq!(std::str::from_utf8(&chunk0).unwrap(), "Hello, World!");

    let chunk1 = reader.chunk_bytes(&toc[1]).unwrap();
    assert_eq!(
        std::str::from_utf8(&chunk1).unwrap(),
        "Chapter two content here"
    );
}

#[test]
fn encrypted_chunk_stream_fails_without_key() {
    let (data, _) = build_drm_file(&[0]);
    let mut stream = HonzoStream::open(std::io::Cursor::new(&data), 1).unwrap();
    let toc = stream.toc_owned();

    match stream.read_chunk(&toc[0]) {
        Err(HonzoError::EncryptedChunk { chunk_id }) => assert_eq!(chunk_id, 0),
        _ => panic!("expected EncryptedChunk error"),
    }
    stream.read_chunk(&toc[1]).unwrap();
}

#[test]
fn encrypted_chunk_stream_decrypts_with_key() {
    let (data, priv_key) = build_drm_file(&[0]);
    let mut stream =
        HonzoStream::open_with_private_key(std::io::Cursor::new(&data), 1, &priv_key).unwrap();
    let toc = stream.toc_owned();

    let chunk0 = stream.read_chunk(&toc[0]).unwrap();
    assert_eq!(std::str::from_utf8(&chunk0).unwrap(), "Hello, World!");

    let chunk1 = stream.read_chunk(&toc[1]).unwrap();
    assert_eq!(
        std::str::from_utf8(&chunk1).unwrap(),
        "Chapter two content here"
    );
}

#[test]
fn drm_encrypts_multiple_chunks() {
    let (pub_key, priv_key) = generate_test_keypair();
    let meta = rmp_serde::to_vec(&honzo_io::HonzoMeta::default()).unwrap();
    let data = HonzoBuilder::new()
        .set_meta(&meta)
        .set_drm_config(DrmConfig {
            encrypt_chunk_ids: vec![0, 1],
            recipient_public_key: pub_key.to_vec(),
            license_url: None,
            expires_at: None,
        })
        .add_chunk(
            *b"CHAP",
            b"First",
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"CHAP",
            b"Second",
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .add_chunk(
            *b"CHAP",
            b"Third (not encrypted)",
            Compression::None,
            MarkupType::Markdown,
            CoverType::Front,
            None,
            None,
            None,
        )
        .finalize()
        .unwrap();

    // Without key: first two chunks fail, third works
    let reader = HonzoReader::new(&data, 1).unwrap();
    let toc = reader.toc();
    assert!(toc[0].is_encrypted());
    assert!(toc[1].is_encrypted());
    assert!(!toc[2].is_encrypted());

    // With key: all chunks decrypt
    let reader = HonzoReader::with_private_key(&data, 1, &priv_key).unwrap();
    let toc = reader.toc();
    assert_eq!(
        std::str::from_utf8(&reader.chunk_bytes(&toc[0]).unwrap()).unwrap(),
        "First"
    );
    assert_eq!(
        std::str::from_utf8(&reader.chunk_bytes(&toc[1]).unwrap()).unwrap(),
        "Second"
    );
    assert_eq!(
        std::str::from_utf8(&reader.chunk_bytes(&toc[2]).unwrap()).unwrap(),
        "Third (not encrypted)"
    );
}

#[test]
fn drm_wrong_key_fails() {
    let (data, _) = build_drm_file(&[0]);
    let (_, wrong_priv) = generate_test_keypair();

    match HonzoReader::with_private_key(&data, 1, &wrong_priv) {
        Err(HonzoError::CryptoError(_)) => {}
        _ => panic!("expected CryptoError for wrong key"),
    }
}

#[test]
fn create_test_keypair_roundtrip() {
    let (pub_key, priv_key) = generate_test_keypair();

    // Verify keys can be parsed by reconstructing
    let secret = StaticSecret::from(priv_key);
    let public = PublicKey::from(&secret);
    assert_eq!(public.to_bytes(), pub_key);
}

#[test]
fn test_drm_envelope_creation() {
    let envelope = DrmEnvelope {
        scheme: "AES-256-GCM+X25519".to_string(),
        encrypted_chunks: vec![0, 1, 2],
        key_envelope: vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
        license_url: Some("https://example.com/license".to_string()),
        expires_at: Some(1893456000),
    };

    assert_eq!(envelope.scheme, "AES-256-GCM+X25519");
    assert_eq!(envelope.encrypted_chunks, vec![0, 1, 2]);
    assert!(!envelope.key_envelope.is_empty());
    assert_eq!(
        envelope.license_url,
        Some("https://example.com/license".to_string())
    );
    assert_eq!(envelope.expires_at, Some(1893456000));
}

#[test]
fn test_drm_envelope_optional_fields() {
    let envelope = DrmEnvelope {
        scheme: "AES-256-GCM+X25519".to_string(),
        encrypted_chunks: vec![],
        key_envelope: vec![1, 2, 3],
        license_url: None,
        expires_at: None,
    };

    assert!(envelope.license_url.is_none());
    assert!(envelope.expires_at.is_none());
}

#[test]
fn test_drm_serialization_roundtrip() {
    let envelope = DrmEnvelope {
        scheme: "AES-256-GCM+X25519".to_string(),
        encrypted_chunks: vec![0, 2, 4],
        key_envelope: vec![100; 256],
        license_url: Some("https://example.com/license".to_string()),
        expires_at: Some(1893456000),
    };

    let data = build_drm(&envelope).unwrap();
    let loaded = parse_drm(&data).unwrap();

    assert_eq!(loaded.scheme, "AES-256-GCM+X25519");
    assert_eq!(loaded.encrypted_chunks, vec![0, 2, 4]);
    assert_eq!(loaded.key_envelope, vec![100; 256]);
    assert_eq!(
        loaded.license_url,
        Some("https://example.com/license".to_string())
    );
    assert_eq!(loaded.expires_at, Some(1893456000));
}

#[test]
fn test_drm_serialization_without_optionals() {
    // Verify that omitting optionals (None) round-trips correctly
    let envelope = DrmEnvelope {
        scheme: "test".to_string(),
        encrypted_chunks: vec![1],
        key_envelope: vec![5; 10],
        license_url: None,
        expires_at: None,
    };

    let data = build_drm(&envelope).unwrap();
    let loaded = parse_drm(&data).unwrap();

    assert_eq!(loaded.scheme, "test");
    assert_eq!(loaded.encrypted_chunks, vec![1]);
    assert_eq!(loaded.key_envelope, vec![5; 10]);
    assert_eq!(loaded.license_url, None);
    assert_eq!(loaded.expires_at, None);
}

#[test]
fn test_build_drm_uses_named() {
    // Verify that serialization uses named keys (msgpack map, not positional)
    let envelope = DrmEnvelope {
        scheme: "test".to_string(),
        encrypted_chunks: vec![0],
        key_envelope: vec![1, 2, 3],
        license_url: None,
        expires_at: None,
    };
    let bytes = build_drm(&envelope).unwrap();
    // Should be a msgpack map (fixmap 0x80..0x8f or map16 0xde)
    assert!(
        (bytes[0] >= 0x80 && bytes[0] <= 0x8f) || bytes[0] == 0xde,
        "expected msgpack map, got byte {:#04x}",
        bytes[0]
    );
}
