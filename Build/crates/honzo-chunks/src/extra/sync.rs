use honzo_core::HonzoError;
use serde::{Deserialize, Serialize};

pub const NAMESPACE: &str = super::SYNC_NAMESPACE;

// Placeholder schema for sync cues.
// TODO: Finalize the schema and implement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncCue {
    pub chunk_id: u32,
    pub offset: u32,
    pub timestamp_ms: u64,
}

pub fn parse_sync(body: &[u8]) -> Result<Vec<SyncCue>, HonzoError> {
    rmp_serde::from_slice(body).map_err(|_| HonzoError::Truncated)
}

pub fn build_sync(cues: &[SyncCue]) -> Result<Vec<u8>, HonzoError> {
    rmp_serde::to_vec(cues).map_err(|_| HonzoError::Truncated)
}
