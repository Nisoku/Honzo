#![cfg(test)]

use honzo_chunks::extra::drm::*;

#[test]
fn test_drm_envelope_creation() {
    let envelope = DrmEnvelope {
        algorithm: "AES-256-CBC".to_string(),
        iv: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        ciphertext: vec![20, 21, 22, 23, 24, 25],
    };

    assert_eq!(envelope.algorithm, "AES-256-CBC");
    assert_eq!(envelope.iv.len(), 16);
    assert_eq!(envelope.ciphertext.len(), 6);
}

#[test]
fn test_drm_serialization() {
    let envelope = DrmEnvelope {
        algorithm: "AES-256-CBC".to_string(),
        iv: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        ciphertext: vec![20, 21, 22, 23, 24, 25],
    };

    // Test serialization and deserialization

    // Serialize
    let data = build_drm(&envelope).unwrap();
    assert!(!data.is_empty());

    // Deserialize
    let loaded_envelope = parse_drm(&data).unwrap();

    assert_eq!(loaded_envelope.algorithm, "AES-256-CBC");
    assert_eq!(
        loaded_envelope.iv,
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
    );
    assert_eq!(loaded_envelope.ciphertext, vec![20, 21, 22, 23, 24, 25]);
}
