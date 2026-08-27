import { describe, expect, it, beforeAll } from '@jest/globals';
import { initHonzo, buildAnnotations, buildHonzo, open } from './helpers.mjs';

beforeAll(async () => {
  await initHonzo();
});

describe('Annotation round-trip', () => {
  it('builds and reads a single annotation', () => {
    const annos = buildAnnotations([{ chunkId: 0, offset: 100, length: 50, type: 'highlight' }]);
    const file = buildHonzo({ annotations: annos, flags: 0x40 });
    const reader = open(file);

    expect(reader.has_annotations()).toBe(true);
    const result = reader.get_annotations();
    expect(result.length).toBe(1);
    expect(result[0].chunk_id).toBe(0);
    expect(result[0].offset).toBe(100);
    expect(result[0].length).toBe(50);
    expect(result[0].type).toBe('highlight');
  });

  it('round-trips multiple annotations', () => {
    const annos = buildAnnotations([
      { chunkId: 0, offset: 0, length: 10, type: 'highlight' },
      { chunkId: 0, offset: 20, length: 30, type: 'underline' },
      { chunkId: 1, offset: 0, length: 15, type: 'highlight' },
    ]);
    const file = buildHonzo({ annotations: annos, flags: 0x40 });
    const reader = open(file);

    const result = reader.get_annotations();
    expect(result.length).toBe(3);
    expect(result[0].chunk_id).toBe(0);
    expect(result[1].type).toBe('underline');
    expect(result[2].chunk_id).toBe(1);
    expect(result[2].offset).toBe(0);
  });

  it('round-trips annotation with optional note field', () => {
    const annos = buildAnnotations([
      { chunkId: 0, offset: 0, length: 10, type: 'highlight', note: 'Important passage' },
    ]);
    const file = buildHonzo({ annotations: annos, flags: 0x40 });
    const reader = open(file);

    const result = reader.get_annotations();
    expect(result[0].note).toBe('Important passage');
  });

  it('round-trips annotation with optional color field', () => {
    const annos = buildAnnotations([
      { chunkId: 0, offset: 0, length: 10, type: 'highlight', color: '#ff0000' },
    ]);
    const file = buildHonzo({ annotations: annos, flags: 0x40 });
    const reader = open(file);

    const result = reader.get_annotations();
    expect(result[0].color).toBe('#ff0000');
  });

  it('round-trips annotation with both note and color', () => {
    const annos = buildAnnotations([
      {
        chunkId: 0,
        offset: 0,
        length: 10,
        type: 'highlight',
        note: 'Important',
        color: '#00ff00',
      },
    ]);
    const file = buildHonzo({ annotations: annos, flags: 0x40 });
    const reader = open(file);

    const result = reader.get_annotations();
    expect(result[0].note).toBe('Important');
    expect(result[0].color).toBe('#00ff00');
  });

  it('handles empty annotations array', () => {
    const annos = buildAnnotations([]);
    const file = buildHonzo({ annotations: annos, flags: 0x40 });
    const reader = open(file);

    const result = reader.get_annotations();
    expect(result).toEqual([]);
  });

  it('works with large chunk_id values', () => {
    const annos = buildAnnotations([{ chunkId: 99999, offset: 0, length: 10, type: 'highlight' }]);
    const file = buildHonzo({ annotations: annos, flags: 0x40 });
    const reader = open(file);

    expect(reader.get_annotations()[0].chunk_id).toBe(99999);
  });

  it('works with various annotation types', () => {
    const types = ['highlight', 'underline', 'strikethrough', 'comment', 'note'];
    const annos = buildAnnotations(
      types.map((t, i) => ({ chunkId: i, offset: i * 10, length: 5, type: t })),
    );
    const file = buildHonzo({ annotations: annos, flags: 0x40 });
    const reader = open(file);

    const result = reader.get_annotations();
    expect(result.length).toBe(types.length);
    types.forEach((t, i) => {
      expect(result[i].type).toBe(t);
    });
  });
});
