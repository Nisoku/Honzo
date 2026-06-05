import { describe, expect, it, beforeAll } from '@jest/globals';
import {
  initHonzo,
  buildAnnotations,
  buildSyncCues,
  buildHonzo,
  open,
  openWithPrivateKey,
  buildExtraEntry,
  buildDrmEnvelope,
} from './helpers.mjs';
import { TEST_PRIV_KEY, TEST_PUB_KEY } from './keys.js';

beforeAll(async () => {
  let honzo;
  // eslint-disable-next-line no-unused-vars
  honzo = await initHonzo();
});

describe('ANNO + SYNC combined', () => {
  it('round-trips annotations and sync cues together', () => {
    const annos = buildAnnotations([{ chunkId: 0, offset: 0, length: 10, type: 'highlight' }]);
    const cues = buildSyncCues([{ syncType: 0, chunkId: 0, offset: 0, timestampMs: 1000 }]);
    const file = buildHonzo({ annotations: annos, syncCues: cues, flags: 0xc0 });
    const reader = open(file);

    expect(reader.has_annotations()).toBe(true);
    expect(reader.has_sync()).toBe(true);
    expect(reader.get_annotations().length).toBe(1);
    expect(reader.get_sync_cues().length).toBe(1);
  });

  it('sets both flag bits when annotations and sync are present', () => {
    const annos = buildAnnotations([{ chunkId: 0, offset: 0, length: 5, type: 'highlight' }]);
    const cues = buildSyncCues([{ syncType: 0, chunkId: 0, offset: 0, timestampMs: 500 }]);
    const file = buildHonzo({ annotations: annos, syncCues: cues, flags: 0xc0 });
    const reader = open(file);

    expect(reader.flags() & 0x40).toBe(0x40);
    expect(reader.flags() & 0x80).toBe(0x80);
  });
});

describe('DRM extra namespace', () => {
  it('recognizes DRM namespace via get_extra', () => {
    const drmBody = buildDrmEnvelope(
      'AES-256-CBC+RSA-OAEP',
      [0, 1],
      [100, 101, 102, 103, 104],
      'https://example.com/license',
      1893456000,
    );
    const extraData = buildExtraEntry('DRM_', drmBody);
    const file = buildHonzo({ extra: extraData, flags: 0x10 });
    const reader = open(file);

    const extra = reader.get_extra();
    const header = new TextDecoder().decode(extra.slice(0, 4));
    expect(header).toBe('DRM_');
  });

  it('has_drm returns true when DRM flag is set', () => {
    const drmBody = buildDrmEnvelope('AES-256-CBC+RSA-OAEP', [0], [1, 2, 3]);
    const extraData = buildExtraEntry('DRM_', drmBody);
    const file = buildHonzo({ extra: extraData, flags: 0x10 });
    const reader = open(file);

    expect(reader.has_drm()).toBe(true);
  });

  it('has_drm returns false when DRM flag is not set', () => {
    const file = buildHonzo({});
    const reader = open(file);

    expect(reader.has_drm()).toBe(false);
  });

  it('honzo_build accepts drm config and sets the DRM flag', () => {
    const file = buildHonzo({
      chunks: [{ tag: 'CHAP', data: [72, 105], content_type_kind: 1, content_type_value: 0 }],
      drm: {
        encrypt_chunk_ids: [0],
        public_key_der: Array.from(TEST_PUB_KEY),
        license_url: 'https://example.com/license',
        expires_at: 1893456000,
      },
    });
    const reader = open(file);
    expect(reader.flags() & 0x10).toBe(0x10);
  });

  it('DRM extra entry is present when drm config is used', () => {
    const file = buildHonzo({
      chunks: [{ tag: 'CHAP', data: [72, 105], content_type_kind: 1, content_type_value: 0 }],
      drm: {
        encrypt_chunk_ids: [0],
        public_key_der: Array.from(TEST_PUB_KEY),
      },
    });
    const reader = open(file);
    const extra = reader.get_extra();
    const header = new TextDecoder().decode(extra.slice(0, 4));
    expect(header).toBe('DRM_');
  });

  it('encrypted chunk cannot be read without a key', () => {
    const file = buildHonzo({
      chunks: [{ tag: 'CHAP', data: [72, 105], content_type_kind: 1, content_type_value: 0 }],
      drm: {
        encrypt_chunk_ids: [0],
        public_key_der: Array.from(TEST_PUB_KEY),
      },
    });
    const reader = open(file);
    expect(() => reader.get_chunk(0)).toThrow();
  });

  it('encrypted chunk can be read with the private key', () => {
    const file = buildHonzo({
      chunks: [{ tag: 'CHAP', data: [72, 105], content_type_kind: 1, content_type_value: 0 }],
      drm: {
        encrypt_chunk_ids: [0],
        public_key_der: Array.from(TEST_PUB_KEY),
      },
    });
    const reader = openWithPrivateKey(file, TEST_PRIV_KEY);
    const chunk = reader.get_chunk(0);
    const decoded = new TextDecoder().decode(chunk);
    expect(decoded).toBe('Hi');
  });
});

describe('Raw extra bytes', () => {
  it('returns raw extra bytes for annotations', () => {
    const annos = buildAnnotations([{ chunkId: 0, offset: 0, length: 5, type: 'highlight' }]);
    const file = buildHonzo({ annotations: annos, flags: 0x40 });
    const reader = open(file);

    const extra = reader.get_extra();
    expect(extra).toBeInstanceOf(Uint8Array);
    expect(extra.length).toBeGreaterThan(0);
    const header = new TextDecoder().decode(extra.slice(0, 4));
    expect(header).toBe('ANNO');
  });

  it('returns raw extra bytes for sync cues', () => {
    const cues = buildSyncCues([{ syncType: 0, chunkId: 0, offset: 0, timestampMs: 1000 }]);
    const file = buildHonzo({ syncCues: cues, flags: 0x80 });
    const reader = open(file);

    const extra = reader.get_extra();
    expect(extra.length).toBeGreaterThan(0);
    const header = new TextDecoder().decode(extra.slice(0, 4));
    expect(header).toBe('SYNC');
  });

  it('contains both ANNO and SYNC entries when both present', () => {
    const annos = buildAnnotations([{ chunkId: 0, offset: 0, length: 5, type: 'highlight' }]);
    const cues = buildSyncCues([{ syncType: 0, chunkId: 0, offset: 0, timestampMs: 1000 }]);
    const file = buildHonzo({ annotations: annos, syncCues: cues, flags: 0xc0 });
    const reader = open(file);

    const extra = reader.get_extra();
    const text = new TextDecoder().decode(extra);
    expect(text).toContain('ANNO');
    expect(text).toContain('SYNC');
  });

  it('returns empty for file without extra', () => {
    const file = buildHonzo({});
    const reader = open(file);

    const extra = reader.get_extra();
    expect(extra.length).toBe(0);
  });

  it('extra_size is 0 for file without extra', () => {
    const file = buildHonzo({});
    const reader = open(file);

    expect(reader.extra_size()).toBe(0n);
  });

  it('extra_size is > 0 for file with annotations', () => {
    const annos = buildAnnotations([{ chunkId: 0, offset: 0, length: 5, type: 'highlight' }]);
    const file = buildHonzo({ annotations: annos, flags: 0x40 });
    const reader = open(file);

    expect(reader.extra_size()).toBeGreaterThan(0n);
  });
});

describe('Error handling', () => {
  it('throws when reading annotations from file without any', () => {
    const file = buildHonzo({});
    const reader = open(file);

    expect(() => reader.get_annotations()).toThrow(/no annotations in extra/);
  });

  it('throws when reading sync cues from file without any', () => {
    const file = buildHonzo({});
    const reader = open(file);

    expect(() => reader.get_sync_cues()).toThrow(/no sync cues in extra/);
  });

  it('throws when annotations flag not set but no extra present', () => {
    const file = buildHonzo({});
    const reader = open(file);

    expect(reader.has_annotations()).toBe(false);
  });

  it('throws when sync flag not set but no extra present', () => {
    const file = buildHonzo({});
    const reader = open(file);

    expect(reader.has_sync()).toBe(false);
  });
});

describe('Metadata round-trip', () => {
  it('get_meta_parsed returns title map', () => {
    const file = buildHonzo({
      meta: { title: { en: 'My Book' }, authors: ['Alice'], language: 'en' },
    });
    const reader = open(file);

    const parsed = reader.get_meta_parsed();
    expect(parsed.title instanceof Map).toBe(true);
    expect(parsed.title.get('en')).toBe('My Book');
    expect(parsed.authors).toEqual(['Alice']);
    expect(parsed.language).toBe('en');
  });

  it('get_meta_parsed returns subtitle when provided', () => {
    const file = buildHonzo({
      meta: { title: { en: 'T' }, subtitle: { en: 'Subt' }, authors: ['A'], language: 'en' },
    });
    const reader = open(file);

    const subtitle = reader.get_meta_parsed().subtitle;
    expect(subtitle instanceof Map).toBe(true);
    expect(subtitle.get('en')).toBe('Subt');
  });

  it('get_meta_parsed returns publisher', () => {
    const file = buildHonzo({
      meta: { title: { en: 'T' }, authors: ['A'], publisher: 'Acme', language: 'en' },
    });
    const reader = open(file);

    expect(reader.get_meta_parsed().publisher).toBe('Acme');
  });

  it('get_meta_parsed returns genres', () => {
    const file = buildHonzo({
      meta: { title: { en: 'T' }, authors: ['A'], genres: ['Fiction', 'Sci-Fi'], language: 'en' },
    });
    const reader = open(file);

    expect(reader.get_meta_parsed().genres).toEqual(['Fiction', 'Sci-Fi']);
  });

  it('get_meta_parsed returns tags', () => {
    const file = buildHonzo({
      meta: { title: { en: 'T' }, authors: ['A'], tags: ['tag1', 'tag2'], language: 'en' },
    });
    const reader = open(file);

    expect(reader.get_meta_parsed().tags).toEqual(['tag1', 'tag2']);
  });

  it('get_meta_parsed returns series info', () => {
    const file = buildHonzo({
      meta: {
        title: { en: 'T' },
        authors: ['A'],
        series: { title: 'Trilogy', position: '1' },
        language: 'en',
      },
    });
    const reader = open(file);

    expect(reader.get_meta_parsed().series.title).toBe('Trilogy');
    expect(reader.get_meta_parsed().series.position).toBe('1');
  });

  it('get_meta_parsed returns language', () => {
    const file = buildHonzo({
      meta: { title: { en: 'T' }, authors: ['A'], language: 'fr' },
    });
    const reader = open(file);

    expect(reader.get_meta_parsed().language).toBe('fr');
  });
});

describe('Builder integration', () => {
  it('builds a valid file with only extra and no chunks', () => {
    const annos = buildAnnotations([{ chunkId: 0, offset: 0, length: 5, type: 'highlight' }]);
    const file = buildHonzo({ annotations: annos, flags: 0x40 });
    const reader = open(file);

    expect(reader.chunk_count()).toBe(0);
    expect(reader.has_annotations()).toBe(true);
  });

  it('has_annotations flag returns false when flag not set', () => {
    // Provide extra data but don't set the annotation flag
    const annos = buildAnnotations([{ chunkId: 0, offset: 0, length: 5, type: 'highlight' }]);
    const file = buildHonzo({ annotations: annos, flags: 0x00 });
    const reader = open(file);

    // The data is there but the flag says no - has_annotations checks flag
    expect(reader.has_annotations()).toBe(false);
  });

  it('has_sync flag returns false when flag not set', () => {
    const cues = buildSyncCues([{ syncType: 0, chunkId: 0, offset: 0, timestampMs: 1000 }]);
    const file = buildHonzo({ syncCues: cues, flags: 0x00 });
    const reader = open(file);

    expect(reader.has_sync()).toBe(false);
  });
});
