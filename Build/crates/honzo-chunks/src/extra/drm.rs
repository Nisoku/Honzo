use honzo_core::HonzoError;
use serde::{Deserialize, Serialize};

pub const NAMESPACE: &str = super::DRM_NAMESPACE;

/// DRM envelope stored as an EXTRA entry under `org.nisoku.drm`.
///
/// Fields use `#[serde(default)]` on optionals and are serialized with
/// `rmp_serde::to_vec_named` so that the msgpack map keys match the
/// field names, consistent with ANNO/SYNC conventions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrmEnvelope {
    /// Encryption scheme identifier (e.g. "AES-256-CBC+ RSA-OAEP")
    pub scheme: String,
    /// IDs of chunks that are encrypted
    pub encrypted_chunks: Vec<u32>,
    /// RSA-OAEP-wrapped AES-256 CEK
    pub key_envelope: Vec<u8>,
    /// Optional license URL
    #[serde(default)]
    pub license_url: Option<String>,
    /// Optional expiry timestamp (Unix epoch seconds)
    #[serde(default)]
    pub expires_at: Option<u64>,
}

pub fn parse_drm(body: &[u8]) -> Result<DrmEnvelope, HonzoError> {
    rmp_serde::from_slice(body).map_err(|_| HonzoError::Truncated)
}

pub fn build_drm(envelope: &DrmEnvelope) -> Result<Vec<u8>, HonzoError> {
    rmp_serde::to_vec_named(envelope).map_err(|_| HonzoError::Truncated)
}
