#![cfg(test)]

use honzo_chunks::extra::sync::*;

#[test]
fn test_sync_cue_creation() {
    // Test audio cue creation
    let audio_cue = new_audio_cue(1, 100, 5000);
    assert_eq!(audio_cue.sync_type, SyncType::Audio);
    assert_eq!(audio_cue.chunk_id, 1);
    assert_eq!(audio_cue.offset, 100);
    assert_eq!(audio_cue.timestamp_ms, 5000);
    assert!(audio_cue.media_id.is_none());
    assert!(audio_cue.duration_ms.is_none());
    assert!(audio_cue.metadata.is_none());

    // Test video cue creation
    let video_cue = new_video_cue(2, 200, 10000);
    assert_eq!(video_cue.sync_type, SyncType::Video);
    assert_eq!(video_cue.chunk_id, 2);
    assert_eq!(video_cue.offset, 200);
    assert_eq!(video_cue.timestamp_ms, 10000);

    // Test page cue creation
    let page_cue = new_page_cue(3, 50, 38);
    assert_eq!(page_cue.sync_type, SyncType::Page);
    assert_eq!(page_cue.chunk_id, 3);
    assert_eq!(page_cue.offset, 50);
    assert_eq!(page_cue.timestamp_ms, 38);
    assert_eq!(page_cue.media_id, Some("page".to_string()));
}

#[test]
fn test_media_segment_cue() {
    let cue = new_media_segment_cue(SyncType::Audio, 1, 100, 5000, 2000, "audio1");

    assert_eq!(cue.sync_type, SyncType::Audio);
    assert_eq!(cue.chunk_id, 1);
    assert_eq!(cue.offset, 100);
    assert_eq!(cue.timestamp_ms, 5000);
    assert_eq!(cue.media_id, Some("audio1".to_string()));
    assert_eq!(cue.duration_ms, Some(2000));
}

#[test]
fn test_sync_track_creation() {
    let track = new_sync_track("audio1", SyncType::Audio, Some("media1"), Some(3600000));

    assert_eq!(track.track_id, "audio1");
    assert_eq!(track.track_type, SyncType::Audio);
    assert_eq!(track.media_id, Some("media1".to_string()));
    assert_eq!(track.media_duration_ms, Some(3600000));
    assert!(track.cues.is_empty());
    assert!(track.metadata.is_none());
}

#[test]
fn test_sync_document_creation() {
    let doc = new_sync_document();

    assert_eq!(doc.version, 1);
    assert!(doc.tracks.is_empty());
    assert!(doc.metadata.is_none());
}

#[test]
fn test_page_sync() {
    // Create page cues
    let cues = vec![new_page_cue(1, 0, 1), new_page_cue(1, 1000, 38)];

    // Test finding page cues
    assert_eq!(find_page_cue(&cues, 1).unwrap().offset, 0);
    assert_eq!(find_page_cue(&cues, 38).unwrap().offset, 1000);
    assert!(find_page_cue(&cues, 99).is_none());

    // Test sorting
    let mut unsorted = cues.clone();
    unsorted.push(new_page_cue(2, 500, 2)); // Add a cue out of order
    sort_sync_cues(&mut unsorted);

    // Verify sorted order
    assert_eq!(unsorted[0].timestamp_ms, 1);
    assert_eq!(unsorted[1].timestamp_ms, 2);
    assert_eq!(unsorted[2].timestamp_ms, 38);
}

#[test]
fn test_sync_validation() {
    // Test valid cue
    let valid_cue = new_audio_cue(1, 100, 5000);
    assert!(validate_cue(&valid_cue).is_ok());

    // Test timestamp 0 is valid (start of stream)
    let mut zero_ts_cue = valid_cue.clone();
    zero_ts_cue.timestamp_ms = 0;
    assert!(validate_cue(&zero_ts_cue).is_ok());

    // Test invalid duration
    let mut invalid_cue = valid_cue.clone();
    invalid_cue.duration_ms = Some(0);
    assert!(validate_cue(&invalid_cue).is_err());

    // Test invalid page number
    let mut invalid_page_cue = new_page_cue(1, 0, 1);
    invalid_page_cue.timestamp_ms = 100001; // Exceeds max page count
    assert!(validate_cue(&invalid_page_cue).is_err());
}

#[test]
fn test_sync_utility_functions() {
    // Create test cues
    let cues = vec![
        new_audio_cue(0, 100, 5000),
        new_audio_cue(0, 500, 10000),
        new_video_cue(1, 200, 15000),
    ];

    // Test filtering by type
    let audio_cues = filter_sync_cues(&cues, SyncType::Audio);
    assert_eq!(audio_cues.len(), 2);

    let video_cues = filter_sync_cues(&cues, SyncType::Video);
    assert_eq!(video_cues.len(), 1);

    // Test finding closest cue
    let closest = find_closest_cue(&cues, 7500).unwrap();
    assert_eq!(closest.timestamp_ms, 5000);

    let closest = find_closest_cue(&cues, 12500).unwrap();
    assert_eq!(closest.timestamp_ms, 10000);

    // Test sorting
    let mut unsorted = cues.clone();
    unsorted.push(new_audio_cue(0, 250, 2500)); // Add a cue out of order
    sort_sync_cues(&mut unsorted);

    // Verify sorted order
    assert_eq!(unsorted[0].timestamp_ms, 2500);
    assert_eq!(unsorted[1].timestamp_ms, 5000);
    assert_eq!(unsorted[2].timestamp_ms, 10000);
    assert_eq!(unsorted[3].timestamp_ms, 15000);

    // Test merging
    let more_cues = vec![new_audio_cue(0, 750, 7500)];

    let merged = merge_sync_cues(&[&cues, &more_cues]);
    assert_eq!(merged.len(), 4);

    // Verify merged cues are sorted
    let timestamps: Vec<u64> = merged.iter().map(|c| c.timestamp_ms).collect();
    assert_eq!(timestamps, vec![5000, 7500, 10000, 15000]);
}
