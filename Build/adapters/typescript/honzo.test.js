import { describe, expect, it, beforeAll } from '@jest/globals';
import { initHonzo, buildHonzo, open } from './helpers.mjs';

beforeAll(async () => {
  await initHonzo();
});

describe('HonzoWasm file header', () => {
  it('returns version_major and version_minor', () => {
    const file = buildHonzo({});
    const reader = open(file);

    expect(reader.version_major()).toBe(1);
    expect(reader.version_minor()).toBe(0);
  });

  it('returns min_reader_version', () => {
    const file = buildHonzo({});
    const reader = open(file);

    expect(reader.min_reader_version()).toBe(1);
  });
});

describe('HonzoWasm flags', () => {
  it('round-trips custom flag bits', () => {
    const file = buildHonzo({ flags: 0x0300 });
    const reader = open(file);

    expect(reader.flags() & 0x0300).toBe(0x0300);
  });

  it('has_annotations returns true when ANNO flag set', () => {
    const file = buildHonzo({ flags: 0x40 });
    const reader = open(file);

    expect(reader.has_annotations()).toBe(true);
  });

  it('has_annotations returns false when ANNO flag not set', () => {
    const file = buildHonzo({ flags: 0x00 });
    const reader = open(file);

    expect(reader.has_annotations()).toBe(false);
  });

  it('has_sync returns true when SYNC flag set', () => {
    const file = buildHonzo({ flags: 0x80 });
    const reader = open(file);

    expect(reader.has_sync()).toBe(true);
  });

  it('has_sync returns false when SYNC flag not set', () => {
    const file = buildHonzo({ flags: 0x00 });
    const reader = open(file);

    expect(reader.has_sync()).toBe(false);
  });
});

describe('HonzoWasm layout', () => {
  it('defaults to layout 0', () => {
    const file = buildHonzo({});
    const reader = open(file);

    expect(reader.layout_mode()).toBe(0);
  });

  it('round-trips layout value', () => {
    const file = buildHonzo({ layout: 1 });
    const reader = open(file);

    expect(reader.layout_mode()).toBe(1);
  });

  it('round-trips layout value 2', () => {
    const file = buildHonzo({ layout: 2 });
    const reader = open(file);

    expect(reader.layout_mode()).toBe(2);
  });
});

describe('HonzoWasm section sizes', () => {
  it('extra_size is 0 for file without extra', () => {
    const file = buildHonzo({});
    const reader = open(file);

    expect(reader.extra_size()).toBe(0n);
  });

  it('extra_size is > 0 for file with annotations', () => {
    const file = buildHonzo({ extra: [1, 2, 3] });
    const reader = open(file);

    expect(reader.extra_size()).toBeGreaterThan(0n);
  });

  it('meta_size is > 0 with default metadata', () => {
    const file = buildHonzo({});
    const reader = open(file);

    expect(reader.meta_size()).toBeGreaterThan(0n);
  });

  it('data_size is 0 when no chunks present', () => {
    const file = buildHonzo({});
    const reader = open(file);

    expect(reader.data_size()).toBe(0n);
  });

  it('toc_size is > 0 when chunks present', () => {
    const file = buildHonzo({ chunks: [{ tag: 'CHAP', data: [72, 105], content_type_kind: 1, content_type_value: 0 }] });
    const reader = open(file);

    expect(reader.toc_size()).toBeGreaterThan(0n);
  });
});

describe('HonzoWasm chunk count', () => {
  it('is 0 for empty file', () => {
    const file = buildHonzo({});
    const reader = open(file);

    expect(reader.chunk_count()).toBe(0);
  });

  it('equals the number of chunks added', () => {
    const file = buildHonzo({
      chunks: [
        { tag: 'CHAP', data: [72, 105], content_type_kind: 1, content_type_value: 0 },
        { tag: 'NOTE', data: [65, 66], content_type_kind: 1, content_type_value: 0 },
      ],
    });
    const reader = open(file);

    expect(reader.chunk_count()).toBe(2);
  });
});

describe('HonzoWasm get_meta', () => {
  it('returns raw metadata bytes', () => {
    const file = buildHonzo({});
    const reader = open(file);

    const meta = reader.get_meta();
    expect(meta).toBeInstanceOf(Uint8Array);
    expect(meta.length).toBeGreaterThan(0);
  });

  it('get_meta_parsed returns an object', () => {
    const file = buildHonzo({});
    const reader = open(file);

    const parsed = reader.get_meta_parsed();
    expect(parsed).toBeInstanceOf(Object);
    expect(parsed.title).toBeDefined();
  });
});
