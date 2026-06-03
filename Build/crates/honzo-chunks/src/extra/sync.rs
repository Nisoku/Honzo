use honzo_core::HonzoError;
use serde::{Deserialize, Serialize};

pub const NAMESPACE: &str = super::SYNC_NAMESPACE;

/// Represents the type of synchronization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[repr(u8)]
pub enum SyncType {
    /// Audio synchronization (text-to-audio)
    #[default]
    Audio = 0,

    /// Video synchronization (text-to-video)
    Video = 1,

    /// Animation synchronization
    Animation = 2,

    /// Page turn synchronization (for pagination)
    Page = 3,

    /// Custom synchronization type
    Custom = 255,
}

/// Represents a synchronization cue point
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncCue {
    /// The type of synchronization
    #[serde(default)]
    pub sync_type: SyncType,

    /// ID of the chunk this cue applies to
    pub chunk_id: u32,

    /// Byte offset within the chunk
    pub offset: u32,

    /// Timestamp in milliseconds (or page number for Page sync type)
    pub timestamp_ms: u64,

    /// Optional identifier for the sync media
    #[serde(default)]
    pub media_id: Option<String>,

    /// Optional duration in milliseconds for this cue
    #[serde(default)]
    pub duration_ms: Option<u64>,

    /// Optional custom data for the cue (e.g., JSON metadata)
    #[serde(default)]
    pub metadata: Option<SyncMetadata>,
}

/// Custom metadata for sync cues
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SyncMetadata {
    /// String metadata
    String(String),

    /// Number metadata
    Number(u64),

    /// Boolean metadata
    Boolean(bool),

    /// Array of values
    Array(Vec<SyncMetadata>),

    /// Key-value pairs
    Map(Vec<(String, SyncMetadata)>),
}

/// Represents a synchronization track (collection of cues for a specific media)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncTrack {
    /// Unique identifier for this track
    pub track_id: String,

    /// Type of synchronization for this track
    pub track_type: SyncType,

    /// Optional media identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_id: Option<String>,

    /// Optional media duration in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_duration_ms: Option<u64>,

    /// List of synchronization cues in this track
    pub cues: Vec<SyncCue>,

    /// Optional track metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SyncMetadata>,
}

/// Represents a complete synchronization document
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncDocument {
    /// Format version
    pub version: u8,

    /// List of synchronization tracks
    pub tracks: Vec<SyncTrack>,

    /// Global metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SyncMetadata>,
}

/// Validates a sync cue
pub fn validate_cue(cue: &SyncCue) -> Result<(), HonzoError> {
    if let Some(duration) = cue.duration_ms {
        if duration == 0 {
            return Err(HonzoError::InvalidSyncCue);
        }
    }

    // For page syncs, validate page number range
    if cue.sync_type == SyncType::Page && cue.timestamp_ms > 100000 {
        // Arbitrary max page count (100,000 pages)
        return Err(HonzoError::InvalidSyncCue);
    }

    Ok(())
}

/// Validates a sync track
pub fn validate_track(track: &SyncTrack) -> Result<(), HonzoError> {
    // Allow empty cues for now
    // if track.cues.is_empty() {
    //     return Err(HonzoError::Truncated);
    // }

    // Validate all cues in the track
    for cue in &track.cues {
        // Ensure cue type matches track type
        if cue.sync_type != track.track_type && track.track_type != SyncType::Custom {
            return Err(HonzoError::InvalidSyncCue);
        }

        validate_cue(cue)?;
    }

    Ok(())
}

/// Validates a sync document
pub fn validate_document(doc: &SyncDocument) -> Result<(), HonzoError> {
    if doc.version != 1 {
        return Err(HonzoError::InvalidSyncCue);
    }

    // Allow empty tracks for now
    // if doc.tracks.is_empty() {
    //     return Err(HonzoError::Truncated);
    // }

    // Validate all tracks
    for track in &doc.tracks {
        validate_track(track)?;
    }

    Ok(())
}

/// Parses sync cues from binary data (legacy format)
pub fn parse_sync(body: &[u8]) -> Result<Vec<SyncCue>, HonzoError> {
    if body.is_empty() {
        return Ok(Vec::new());
    }

    let cues: Vec<SyncCue> = rmp_serde::from_slice(body).map_err(|e| {
        eprintln!("Failed to deserialize sync cues: {:?}", e);
        HonzoError::Truncated
    })?;

    // Validate all cues
    for cue in &cues {
        if let Err(e) = validate_cue(cue) {
            eprintln!("Invalid sync cue: {:?}", cue);
            return Err(e);
        }
    }

    Ok(cues)
}

/// Parses a sync document from binary data
pub fn parse_sync_document(body: &[u8]) -> Result<SyncDocument, HonzoError> {
    if body.is_empty() {
        return Ok(SyncDocument {
            version: 1,
            tracks: Vec::new(),
            metadata: None,
        });
    }

    let doc: SyncDocument = rmp_serde::from_slice(body).map_err(|e| {
        eprintln!("Failed to deserialize sync document: {:?}", e);
        HonzoError::Truncated
    })?;

    if let Err(e) = validate_document(&doc) {
        eprintln!("Invalid sync document: {:?}", doc);
        return Err(e);
    }

    Ok(doc)
}

/// Builds binary data from sync cues (legacy format)
pub fn build_sync(cues: &[SyncCue]) -> Result<Vec<u8>, HonzoError> {
    if cues.is_empty() {
        return Ok(Vec::new());
    }

    // Validate all cues before building
    for cue in cues {
        if let Err(e) = validate_cue(cue) {
            eprintln!("Invalid sync cue during build: {:?}", cue);
            return Err(e);
        }
    }

    rmp_serde::to_vec_named(cues).map_err(|e| {
        eprintln!("Failed to serialize sync cues: {:?}", e);
        HonzoError::Truncated
    })
}

/// Builds binary data from a sync document
pub fn build_sync_document(doc: &SyncDocument) -> Result<Vec<u8>, HonzoError> {
    if let Err(e) = validate_document(doc) {
        eprintln!("Invalid sync document during build: {:?}", doc);
        return Err(e);
    }

    rmp_serde::to_vec_named(doc).map_err(|e| {
        eprintln!("Failed to serialize sync document: {:?}", e);
        HonzoError::Truncated
    })
}

/// Creates a new audio sync cue
pub fn new_audio_cue(chunk_id: u32, offset: u32, timestamp_ms: u64) -> SyncCue {
    SyncCue {
        sync_type: SyncType::Audio,
        chunk_id,
        offset,
        timestamp_ms,
        media_id: None,
        duration_ms: None,
        metadata: None,
    }
}

/// Creates a new video sync cue
pub fn new_video_cue(chunk_id: u32, offset: u32, timestamp_ms: u64) -> SyncCue {
    SyncCue {
        sync_type: SyncType::Video,
        chunk_id,
        offset,
        timestamp_ms,
        media_id: None,
        duration_ms: None,
        metadata: None,
    }
}

/// Creates a new page sync cue (for pagination)
pub fn new_page_cue(chunk_id: u32, offset: u32, page_number: u32) -> SyncCue {
    SyncCue {
        sync_type: SyncType::Page,
        chunk_id,
        offset,
        timestamp_ms: page_number as u64,
        media_id: Some("page".to_string()),
        duration_ms: None,
        metadata: None,
    }
}

/// Creates a new media segment cue with duration
pub fn new_media_segment_cue(
    sync_type: SyncType,
    chunk_id: u32,
    offset: u32,
    timestamp_ms: u64,
    duration_ms: u64,
    media_id: &str,
) -> SyncCue {
    SyncCue {
        sync_type,
        chunk_id,
        offset,
        timestamp_ms,
        media_id: Some(media_id.to_string()),
        duration_ms: Some(duration_ms),
        metadata: None,
    }
}

/// Creates a new sync track
pub fn new_sync_track(
    track_id: &str,
    track_type: SyncType,
    media_id: Option<&str>,
    media_duration_ms: Option<u64>,
) -> SyncTrack {
    SyncTrack {
        track_id: track_id.to_string(),
        track_type,
        media_id: media_id.map(|s| s.to_string()),
        media_duration_ms,
        cues: Vec::new(),
        metadata: None,
    }
}

/// Creates a new sync document
pub fn new_sync_document() -> SyncDocument {
    SyncDocument {
        version: 1,
        tracks: Vec::new(),
        metadata: None,
    }
}

/// Converts sync cues to a more readable format for debugging
pub fn sync_cues_to_debug_string(cues: &[SyncCue]) -> String {
    cues.iter()
        .map(|cue| {
            format!(
                "SyncCue {{ type: {:?}, chunk: {}, offset: {}, time: {}ms, media: {:?}, duration: {:?}, metadata: {:?} }}",
                cue.sync_type,
                cue.chunk_id,
                cue.offset,
                cue.timestamp_ms,
                cue.media_id,
                cue.duration_ms,
                cue.metadata
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Filters sync cues by type
pub fn filter_sync_cues(cues: &[SyncCue], sync_type: SyncType) -> Vec<SyncCue> {
    cues.iter()
        .filter(|cue| cue.sync_type == sync_type)
        .cloned()
        .collect()
}

/// Filters sync cues by media ID
pub fn filter_sync_cues_by_media(cues: &[SyncCue], media_id: &str) -> Vec<SyncCue> {
    cues.iter()
        .filter(|cue| cue.media_id.as_deref() == Some(media_id))
        .cloned()
        .collect()
}

/// Finds the sync cue closest to a given timestamp
pub fn find_closest_cue(cues: &[SyncCue], timestamp_ms: u64) -> Option<&SyncCue> {
    cues.iter()
        .min_by_key(|cue| cue.timestamp_ms.abs_diff(timestamp_ms))
}

/// Finds the sync cue for a specific page number
pub fn find_page_cue(cues: &[SyncCue], page_number: u32) -> Option<&SyncCue> {
    cues.iter()
        .find(|cue| cue.sync_type == SyncType::Page && cue.timestamp_ms == page_number as u64)
}

/// Sorts sync cues by timestamp
pub fn sort_sync_cues(cues: &mut [SyncCue]) {
    cues.sort_by_key(|a| a.timestamp_ms);
}

/// Merges multiple sets of sync cues
pub fn merge_sync_cues(cues_sets: &[&[SyncCue]]) -> Vec<SyncCue> {
    let mut merged = Vec::new();
    for cues in cues_sets {
        merged.extend_from_slice(cues);
    }
    sort_sync_cues(&mut merged);
    merged
}

/// Converts legacy sync cues to a sync document
pub fn legacy_cues_to_document(cues: Vec<SyncCue>) -> SyncDocument {
    let mut doc = new_sync_document();

    // Group cues by type
    let mut audio_cues = Vec::new();
    let mut video_cues = Vec::new();
    let mut page_cues = Vec::new();
    let mut custom_cues = Vec::new();

    for cue in cues {
        match cue.sync_type {
            SyncType::Audio => audio_cues.push(cue),
            SyncType::Video => video_cues.push(cue),
            SyncType::Page => page_cues.push(cue),
            SyncType::Animation | SyncType::Custom => custom_cues.push(cue),
        }
    }

    // Create tracks for each type
    if !audio_cues.is_empty() {
        let mut track = new_sync_track("audio", SyncType::Audio, None, None);
        track.cues = audio_cues;
        doc.tracks.push(track);
    }

    if !video_cues.is_empty() {
        let mut track = new_sync_track("video", SyncType::Video, None, None);
        track.cues = video_cues;
        doc.tracks.push(track);
    }

    if !page_cues.is_empty() {
        let mut track = new_sync_track("pages", SyncType::Page, Some("page"), None);
        track.cues = page_cues;
        doc.tracks.push(track);
    }

    if !custom_cues.is_empty() {
        let mut track = new_sync_track("custom", SyncType::Custom, None, None);
        track.cues = custom_cues;
        doc.tracks.push(track);
    }

    doc
}
