import { describe, expect, it, beforeAll } from '@jest/globals';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';
import { initHonzo, buildHonzo, open } from './helpers.mjs';

const __dirname = dirname(fileURLToPath(import.meta.url));

beforeAll(async () => {
  await initHonzo();
});

function txt(str) {
  return Array.from(new TextEncoder().encode(str));
}

describe('get_pmap from built files', () => {
  it('returns empty array when no PMAP entries added', () => {
    const file = buildHonzo({
      chunks: [{ tag: 'CHAP', data: txt('Hi'), content_type_kind: 1, content_type_value: 0 }],
    });
    const reader = open(file);
    expect(reader.get_pmap()).toEqual([]);
  });

  it('returns PMAP entries for added page mappings', () => {
    const file = buildHonzo({
      chunks: [{ tag: 'CHAP', data: txt('Chapter'), content_type_kind: 1, content_type_value: 0 }],
      pmapEntries: [
        { printPage: 1, chunkId: 0, byteOffset: 0 },
        { printPage: 5, chunkId: 0, byteOffset: 50 },
      ],
    });
    const reader = open(file);
    const pmap = reader.get_pmap();
    expect(pmap.length).toBe(2);
    expect(pmap[0].printPage).toBe(1);
    expect(pmap[0].chunkId).toBe(0);
    expect(pmap[0].byteOffset).toBe(0);
    expect(pmap[1].printPage).toBe(5);
    expect(pmap[1].chunkId).toBe(0);
    expect(pmap[1].byteOffset).toBe(50);
  });

  it('preserves insertion order of PMAP entries', () => {
    const file = buildHonzo({
      chunks: [{ tag: 'CHAP', data: txt('Ch'), content_type_kind: 1, content_type_value: 0 }],
      pmapEntries: [
        { printPage: 10, chunkId: 0, byteOffset: 0 },
        { printPage: 1, chunkId: 0, byteOffset: 0 },
        { printPage: 5, chunkId: 0, byteOffset: 0 },
      ],
    });
    const reader = open(file);
    const pmap = reader.get_pmap();
    expect(pmap.length).toBe(3);
    expect(pmap[0].printPage).toBe(10);
    expect(pmap[1].printPage).toBe(1);
    expect(pmap[2].printPage).toBe(5);
  });

  it('references multiple chunks', () => {
    const file = buildHonzo({
      chunks: [
        { tag: 'CHAP', data: txt('Ch1'), content_type_kind: 1, content_type_value: 0 },
        { tag: 'CHAP', data: txt('Ch2'), content_type_kind: 1, content_type_value: 0 },
      ],
      pmapEntries: [
        { printPage: 1, chunkId: 0, byteOffset: 0 },
        { printPage: 15, chunkId: 1, byteOffset: 10 },
      ],
    });
    const reader = open(file);
    const pmap = reader.get_pmap();
    expect(pmap.length).toBe(2);
    expect(pmap[0].chunkId).toBe(0);
    expect(pmap[1].chunkId).toBe(1);
  });
});

describe('get_pmap from fixture files', () => {
  it('reads PMAP from with_pmap fixture', () => {
    const bytes = readFileSync(resolve(__dirname, '../../../Tests/fixtures/with_pmap.hzo'));
    const reader = open(bytes);
    const pmap = reader.get_pmap();
    expect(pmap.length).toBe(6);
    expect(pmap[0].printPage).toBe(1);
  });

  it('returns empty PMAP from minimal fixture', () => {
    const bytes = readFileSync(resolve(__dirname, '../../../Tests/fixtures/minimal.hzo'));
    const reader = open(bytes);
    expect(reader.get_pmap()).toEqual([]);
  });
});
