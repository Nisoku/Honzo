import { describe, expect, it, beforeAll } from '@jest/globals';
import { initHonzo, buildSyncCues, buildHonzo, open } from './helpers.mjs';

beforeAll(async () => {
  await initHonzo();
});

describe('Sync cue round-trip', () => {
  it('builds and reads a single sync cue', () => {
    const cues = buildSyncCues([{ syncType: 0, chunkId: 0, offset: 0, timestampMs: 5000 }]);
    const file = buildHonzo({ syncCues: cues, flags: 0x80 });
    const reader = open(file);

    expect(reader.has_sync()).toBe(true);
    const result = reader.get_sync_cues();
    expect(result.length).toBe(1);
    expect(result[0].chunk_id).toBe(0);
    expect(result[0].offset).toBe(0);
    expect(result[0].timestamp_ms).toBe(5000);
  });

  it('round-trips multiple sync cues', () => {
    const cues = buildSyncCues([
      { syncType: 0, chunkId: 0, offset: 0, timestampMs: 0 },
      { syncType: 0, chunkId: 0, offset: 100, timestampMs: 3000 },
      { syncType: 0, chunkId: 1, offset: 0, timestampMs: 10000 },
    ]);
    const file = buildHonzo({ syncCues: cues, flags: 0x80 });
    const reader = open(file);

    const result = reader.get_sync_cues();
    expect(result.length).toBe(3);
    expect(result[0].timestamp_ms).toBe(0);
    expect(result[1].timestamp_ms).toBe(3000);
    expect(result[2].timestamp_ms).toBe(10000);
  });

  it('handles zero timestamp', () => {
    const cues = buildSyncCues([{ syncType: 0, chunkId: 0, offset: 0, timestampMs: 0 }]);
    const file = buildHonzo({ syncCues: cues, flags: 0x80 });
    const reader = open(file);

    expect(reader.get_sync_cues()[0].timestamp_ms).toBe(0);
  });

  it('round-trips sync cue with media_id', () => {
    const cues = buildSyncCues([
      { syncType: 0, chunkId: 0, offset: 0, timestampMs: 5000, mediaId: 'audio_track_1' },
    ]);
    const file = buildHonzo({ syncCues: cues, flags: 0x80 });
    const reader = open(file);

    expect(reader.get_sync_cues()[0].media_id).toBe('audio_track_1');
  });

  it('round-trips sync cue with duration_ms', () => {
    const cues = buildSyncCues([
      { syncType: 0, chunkId: 0, offset: 0, timestampMs: 5000, durationMs: 3000 },
    ]);
    const file = buildHonzo({ syncCues: cues, flags: 0x80 });
    const reader = open(file);

    expect(reader.get_sync_cues()[0].duration_ms).toBe(3000);
  });

  it('round-trips sync cues with all optional fields', () => {
    const cues = buildSyncCues([
      {
        syncType: 1,
        chunkId: 0,
        offset: 100,
        timestampMs: 10000,
        mediaId: 'video_track',
        durationMs: 5000,
      },
    ]);
    const file = buildHonzo({ syncCues: cues, flags: 0x80 });
    const reader = open(file);

    const result = reader.get_sync_cues();
    expect(result[0].chunk_id).toBe(0);
    expect(result[0].offset).toBe(100);
    expect(result[0].timestamp_ms).toBe(10000);
    expect(result[0].media_id).toBe('video_track');
    expect(result[0].duration_ms).toBe(5000);
  });

  it('handles various sync types', () => {
    const cues = buildSyncCues([
      { syncType: 0, chunkId: 0, offset: 0, timestampMs: 1000 },
      { syncType: 1, chunkId: 1, offset: 0, timestampMs: 2000 },
      { syncType: 2, chunkId: 2, offset: 0, timestampMs: 3000 },
      { syncType: 3, chunkId: 3, offset: 0, timestampMs: 4000 },
    ]);
    const file = buildHonzo({ syncCues: cues, flags: 0x80 });
    const reader = open(file);

    const result = reader.get_sync_cues();
    expect(result.length).toBe(4);
    expect(result[0].chunk_id).toBe(0);
    expect(result[1].chunk_id).toBe(1);
    expect(result[2].chunk_id).toBe(2);
    expect(result[3].chunk_id).toBe(3);
  });

  it('handles empty sync cues', () => {
    const cues = buildSyncCues([]);
    const file = buildHonzo({ syncCues: cues, flags: 0x80 });
    const reader = open(file);

    expect(reader.get_sync_cues()).toEqual([]);
  });

  it('works with large timestamp_ms values', () => {
    const largeMs = 4294967295;
    const cues = buildSyncCues([{ syncType: 0, chunkId: 0, offset: 0, timestampMs: largeMs }]);
    const file = buildHonzo({ syncCues: cues, flags: 0x80 });
    const reader = open(file);

    expect(reader.get_sync_cues()[0].timestamp_ms).toBe(largeMs);
  });
});
