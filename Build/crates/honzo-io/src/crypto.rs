use honzo_core::HonzoError;

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use hkdf::Hkdf;
use sha2::Sha256;

use x25519_dalek::{EphemeralSecret, PublicKey, StaticSecret};

use getrandom::fill as getrandom;

/// Generate a random 256-bit content encryption key.
pub fn generate_cek() -> Result<[u8; 32], HonzoError> {
    let mut cek = [0u8; 32];
    getrandom(&mut cek).map_err(|_| HonzoError::CryptoError("rng failed"))?;
    Ok(cek)
}

/// Generate random bytes (e.g. for nonce).
pub fn random_bytes(len: usize) -> Result<Vec<u8>, HonzoError> {
    let mut buf = vec![0u8; len];
    getrandom(&mut buf).map_err(|_| HonzoError::CryptoError("rng failed"))?;
    Ok(buf)
}

/// Encrypt `data` with AES-256-GCM using the given `key` and a random nonce.
/// Returns [12-byte nonce || ciphertext || 16-byte tag].
pub fn encrypt_chunk(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, HonzoError> {
    let nonce_bytes = random_bytes(12)?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    let cipher =
        Aes256Gcm::new_from_slice(key).map_err(|_| HonzoError::CryptoError("invalid aes key"))?;

    let ciphertext = cipher
        .encrypt(nonce, data)
        .map_err(|_| HonzoError::CryptoError("aes encryption failed"))?;

    let mut out = Vec::with_capacity(12 + ciphertext.len());
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

/// Decrypt `data` (expected format: [12-byte nonce || ciphertext || 16-byte tag]) with AES-256-GCM.
pub fn decrypt_chunk(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, HonzoError> {
    if data.len() < 12 + 16 {
        return Err(HonzoError::CryptoError("truncated encrypted data"));
    }
    let nonce = Nonce::from_slice(&data[..12]);
    let ciphertext = &data[12..];

    let cipher =
        Aes256Gcm::new_from_slice(key).map_err(|_| HonzoError::CryptoError("invalid aes key"))?;

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| HonzoError::CryptoError("aes decryption failed"))?;

    Ok(plaintext)
}

/// Derive a 256-bit AES-GCM wrapping key from an ECDH shared secret using HKDF-SHA256.
fn derive_wrapping_key(shared_secret: &[u8]) -> Result<[u8; 32], HonzoError> {
    let hk = Hkdf::<Sha256>::new(Some(b"honzo-cek-wrap"), shared_secret);
    let mut key = [0u8; 32];
    hk.expand(b"cek-key", &mut key)
        .map_err(|_| HonzoError::CryptoError("hkdf expand failed"))?;
    Ok(key)
}

/// Wrap a CEK for a recipient's X25519 public key.
///
/// Returns `key_envelope`: [`32-byte ephemeral public key` || `AES-GCM encrypted CEK`].
pub fn wrap_cek(cek: &[u8; 32], recipient_public_key: &[u8]) -> Result<Vec<u8>, HonzoError> {
    if recipient_public_key.len() != 32 {
        return Err(HonzoError::CryptoError(
            "invalid recipient public key length",
        ));
    }

    // Generate ephemeral X25519 key pair
    let ephem_secret = EphemeralSecret::random();
    let ephem_public = PublicKey::from(&ephem_secret);

    // ECDH agreement
    let recipient_pub = PublicKey::from(<[u8; 32]>::try_from(recipient_public_key).unwrap());
    let shared_secret = ephem_secret.diffie_hellman(&recipient_pub);

    // Derive wrapping key via HKDF
    let wrapping_key = derive_wrapping_key(shared_secret.as_bytes())?;

    // Encrypt CEK with the derived wrapping key
    let encrypted_cek = encrypt_chunk(cek, &wrapping_key)?;

    // Pack: [32-byte ephem pub key][encrypted CEK]
    let mut out = Vec::with_capacity(32 + encrypted_cek.len());
    out.extend_from_slice(ephem_public.as_bytes());
    out.extend_from_slice(&encrypted_cek);
    Ok(out)
}

/// Unwrap a CEK using our X25519 private key and the key envelope.
///
/// `key_envelope` format: [`32-byte ephemeral public key` || `AES-GCM encrypted CEK`].
pub fn unwrap_cek(key_envelope: &[u8], private_key: &[u8]) -> Result<[u8; 32], HonzoError> {
    if key_envelope.len() < 32 + 12 + 16 {
        return Err(HonzoError::CryptoError("truncated key envelope"));
    }
    if private_key.len() != 32 {
        return Err(HonzoError::CryptoError("invalid private key length"));
    }

    // Parse: [32-byte ephem pub key][encrypted CEK]
    let ephem_pub_bytes: [u8; 32] = key_envelope[..32]
        .try_into()
        .map_err(|_| HonzoError::CryptoError("invalid ephem pub key"))?;
    let encrypted_cek = &key_envelope[32..];

    // Reconstruct our static private key and the ephemeral public key
    let private_key_arr: [u8; 32] = private_key
        .try_into()
        .map_err(|_| HonzoError::CryptoError("invalid private key length"))?;
    let our_secret = StaticSecret::from(private_key_arr);
    let ephem_pub = PublicKey::from(ephem_pub_bytes);

    // ECDH agreement
    let shared_secret = our_secret.diffie_hellman(&ephem_pub);

    // Derive the same wrapping key
    let wrapping_key = derive_wrapping_key(shared_secret.as_bytes())?;

    // Decrypt the CEK
    let decrypted = decrypt_chunk(encrypted_cek, &wrapping_key)?;
    if decrypted.len() != 32 {
        return Err(HonzoError::CryptoError("unwrapped key size mismatch"));
    }
    let mut cek = [0u8; 32];
    cek.copy_from_slice(&decrypted);
    Ok(cek)
}
