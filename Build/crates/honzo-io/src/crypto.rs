use aes::Aes256;
use cbc::{Decryptor, Encryptor};
use cipher::block_padding::Pkcs7;
use cipher::{BlockModeDecrypt, BlockModeEncrypt, KeyIvInit};
use honzo_core::HonzoError;
use rand::rngs::OsRng;
use rand::RngCore;

use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey};
use rsa::Oaep;
use rsa::{RsaPrivateKey, RsaPublicKey};
use sha2::Sha256;

type Aes256CbcEnc = Encryptor<Aes256>;
type Aes256CbcDec = Decryptor<Aes256>;

/// Generate a random 256-bit content encryption key.
pub fn generate_cek() -> Result<[u8; 32], HonzoError> {
    let mut cek = [0u8; 32];
    OsRng.fill_bytes(&mut cek);
    Ok(cek)
}

/// Generate random bytes (e.g. for IV).
pub fn random_bytes(len: usize) -> Result<Vec<u8>, HonzoError> {
    let mut buf = vec![0u8; len];
    OsRng.fill_bytes(&mut buf);
    Ok(buf)
}

/// Encrypt `data` with AES-256-CBC using the given `key` and a random IV.
/// Returns `[16-byte IV || ciphertext]`.
pub fn encrypt_chunk(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, HonzoError> {
    let iv = random_bytes(16)?;
    let iv_arr: &[u8; 16] = &iv
        .as_slice()
        .try_into()
        .map_err(|_| HonzoError::CryptoError("iv size"))?;

    let mut buf = data.to_vec();
    buf.extend(std::iter::repeat_n(0u8, 16));

    let ciphertext = Aes256CbcEnc::new(key.into(), iv_arr.into())
        .encrypt_padded::<Pkcs7>(&mut buf, data.len())
        .map_err(|_| HonzoError::CryptoError("aes encryption failed"))?;

    let mut out = Vec::with_capacity(16 + ciphertext.len());
    out.extend_from_slice(&iv);
    out.extend_from_slice(ciphertext);
    Ok(out)
}

/// Decrypt `data` (expected format: `[16-byte IV || ciphertext]`) with AES-256-CBC.
pub fn decrypt_chunk(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, HonzoError> {
    if data.len() < 16 {
        return Err(HonzoError::CryptoError("truncated encrypted data"));
    }
    let iv_arr: &[u8; 16] = &data[..16].try_into().unwrap();
    let ciphertext = &data[16..];

    let mut buf = ciphertext.to_vec();
    let plaintext = Aes256CbcDec::new(key.into(), iv_arr.into())
        .decrypt_padded::<Pkcs7>(&mut buf)
        .map_err(|_| HonzoError::CryptoError("aes decryption failed"))?;

    Ok(plaintext.to_vec())
}

/// Wrap (RSA-OAEP encrypt) a CEK with a DER-encoded RSA public key.
pub fn wrap_cek(cek: &[u8; 32], public_key_der: &[u8]) -> Result<Vec<u8>, HonzoError> {
    let public_key = RsaPublicKey::from_public_key_der(public_key_der)
        .map_err(|_| HonzoError::CryptoError("invalid RSA public key"))?;
    public_key
        .encrypt(&mut OsRng, Oaep::new::<Sha256>(), cek)
        .map_err(|_| HonzoError::CryptoError("RSA-OAEP wrap failed"))
}

/// Unwrap (RSA-OAEP decrypt) a CEK using a DER-encoded RSA private key.
fn parse_private_key_der(bytes: &[u8]) -> Result<RsaPrivateKey, HonzoError> {
    RsaPrivateKey::from_pkcs8_der(bytes)
        .map_err(|_| HonzoError::CryptoError("invalid RSA private key"))
}

pub fn unwrap_cek(key_envelope: &[u8], private_key_der: &[u8]) -> Result<[u8; 32], HonzoError> {
    let private_key = parse_private_key_der(private_key_der)?;
    let decrypted = private_key
        .decrypt(Oaep::new::<Sha256>(), key_envelope)
        .map_err(|_| HonzoError::CryptoError("RSA-OAEP unwrap failed"))?;
    if decrypted.len() != 32 {
        return Err(HonzoError::CryptoError("unwrapped key size mismatch"));
    }
    let mut cek = [0u8; 32];
    cek.copy_from_slice(&decrypted);
    Ok(cek)
}
