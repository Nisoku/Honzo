use honzo_core::HonzoError;
use serde::{Deserialize, Serialize};

pub const NAMESPACE: &str = "org.nisoku.drm";

// Placeholder schema until the AES-256-CBC envelope format is finalized.
// TODO: Finalize the envelope format and update this schema accordingly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrmEnvelope {
    pub algorithm: String,
    pub iv: Vec<u8>,
    pub ciphertext: Vec<u8>,
}

pub fn parse_drm(body: &[u8]) -> Result<DrmEnvelope, HonzoError> {
    rmp_serde::from_slice(body).map_err(|_| HonzoError::Truncated)
}

pub fn build_drm(envelope: &DrmEnvelope) -> Result<Vec<u8>, HonzoError> {
    rmp_serde::to_vec(envelope).map_err(|_| HonzoError::Truncated)
}
