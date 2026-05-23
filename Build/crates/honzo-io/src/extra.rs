use honzo_chunks::extra::is_known_namespace;
use honzo_core::HonzoError;

pub struct ExtraEntry {
    pub tag: [u8; 4],
    pub namespace: String,
    pub body: Vec<u8>,
    pub known: bool,
}

pub fn parse_extra(bytes: &[u8]) -> Result<Vec<ExtraEntry>, HonzoError> {
    let mut entries = Vec::new();
    let mut cursor = 0usize;

    while cursor + 6 <= bytes.len() {
        let mut tag = [0u8; 4];
        tag.copy_from_slice(&bytes[cursor..cursor + 4]);
        cursor += 4;

        let namespace_len = u16::from_le_bytes([bytes[cursor], bytes[cursor + 1]]) as usize;
        cursor += 2;
        if cursor + namespace_len > bytes.len() {
            return Err(HonzoError::Truncated);
        }
        let namespace = std::str::from_utf8(&bytes[cursor..cursor + namespace_len])
            .map_err(|_| HonzoError::Truncated)?
            .to_string();
        let known = is_known_namespace(&namespace);
        cursor += namespace_len;

        if cursor + 4 > bytes.len() {
            return Err(HonzoError::Truncated);
        }
        let body_len = u32::from_le_bytes([
            bytes[cursor],
            bytes[cursor + 1],
            bytes[cursor + 2],
            bytes[cursor + 3],
        ]) as usize;
        cursor += 4;
        if cursor + body_len > bytes.len() {
            return Err(HonzoError::Truncated);
        }
        let body = bytes[cursor..cursor + body_len].to_vec();
        cursor += body_len;

        entries.push(ExtraEntry {
            tag,
            namespace,
            body,
            known,
        });
    }

    Ok(entries)
}

pub fn find_extra<'a>(entries: &'a [ExtraEntry], namespace: &str) -> Option<&'a ExtraEntry> {
    entries.iter().find(|entry| entry.namespace == namespace)
}
