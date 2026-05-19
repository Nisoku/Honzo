use honzo_core::HonzoError;
use serde::{Deserialize, Serialize};

// TODO: Make sure all EXTRA chunks have namespace consts
pub const NAMESPACE: &str = "org.nisoku.anno";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Annotation {
    pub chunk_id: u32,
    pub offset: u32,
    pub length: u32,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

pub fn parse_anno(body: &[u8]) -> Result<Vec<Annotation>, HonzoError> {
    rmp_serde::from_slice(body).map_err(|_| HonzoError::Truncated)
}

pub fn build_anno(annotations: &[Annotation]) -> Result<Vec<u8>, HonzoError> {
    rmp_serde::to_vec(annotations).map_err(|_| HonzoError::Truncated)
}
