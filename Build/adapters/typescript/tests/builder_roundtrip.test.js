import { describe, expect, it, beforeAll } from '@jest/globals';
import { initHonzo, buildHonzo, open } from './helpers.mjs';

beforeAll(async () => {
  await initHonzo();
});

function txt(str) {
  return Array.from(new TextEncoder().encode(str));
}

describe('Chunk round-trip', () => {
  it('builds and reads a single Markdown CHAP chunk', () => {
    const data = txt('# Hello\n\nThis is a chapter.');
    const file = buildHonzo({
      chunks: [{ tag: 'CHAP', data, content_type_kind: 1, content_type_value: 0 }],
    });
    const reader = open(file);

    expect(reader.chunk_count()).toBe(1);
    const chunk = reader.get_chunk(0);
    expect(chunk).toEqual(new Uint8Array(data));
  });

  it('builds and reads multiple chunks of different types', () => {
    const chap = txt('Chapter one');
    const note = txt('Footnote text');
    const file = buildHonzo({
      chunks: [
        { tag: 'CHAP', data: chap, content_type_kind: 1, content_type_value: 0 },
        { tag: 'NOTE', data: note, content_type_kind: 1, content_type_value: 0 },
      ],
    });
    const reader = open(file);

    expect(reader.chunk_count()).toBe(2);
    expect(reader.get_chunk(0)).toEqual(new Uint8Array(chap));
    expect(reader.get_chunk(1)).toEqual(new Uint8Array(note));
  });

  it('builds a MATH chunk with LaTeX', () => {
    const latex = txt('E = mc^2');
    const file = buildHonzo({
      chunks: [{ tag: 'MATH', data: latex, content_type_kind: 2, content_type_value: 1 }],
    });
    const reader = open(file);

    expect(reader.chunk_count()).toBe(1);
    expect(reader.get_chunk(0)).toEqual(new Uint8Array(latex));
  });

  it('builds a MATH chunk with MathML', () => {
    const mathml = txt('<math><mi>E</mi></math>');
    const file = buildHonzo({
      chunks: [{ tag: 'MATH', data: mathml, content_type_kind: 2, content_type_value: 0 }],
    });
    const reader = open(file);

    expect(reader.chunk_count()).toBe(1);
    expect(reader.get_chunk(0)).toEqual(new Uint8Array(mathml));
  });

  it('builds a CHAP with HTML content type', () => {
    const html = txt('<p>Hello</p>');
    const file = buildHonzo({
      chunks: [{ tag: 'CHAP', data: html, content_type_kind: 1, content_type_value: 1 }],
    });
    const reader = open(file);

    expect(reader.get_chunk(0)).toEqual(new Uint8Array(html));
  });

  it('handles empty chunk data', () => {
    const file = buildHonzo({
      chunks: [{ tag: 'CHAP', data: [], content_type_kind: 1, content_type_value: 0 }],
    });
    const reader = open(file);

    expect(reader.chunk_count()).toBe(1);
    expect(reader.get_chunk(0)).toEqual(new Uint8Array(0));
  });

  it('builds chunks with LZ4 compression', () => {
    const data = txt('Some compressible data '.repeat(10));
    const file = buildHonzo({
      chunks: [
        {
          tag: 'CHAP',
          data,
          compression: 1,
          content_type_kind: 1,
          content_type_value: 0,
        },
      ],
    });
    const reader = open(file);

    expect(reader.chunk_count()).toBe(1);
    const chunk = reader.get_chunk(0);
    expect(chunk).toEqual(new Uint8Array(data));
  });
});

describe('get_chunk error handling', () => {
  it('throws for out-of-bounds index', () => {
    const file = buildHonzo({
      chunks: [{ tag: 'CHAP', data: [1, 2, 3], content_type_kind: 1, content_type_value: 0 }],
    });
    const reader = open(file);

    expect(() => reader.get_chunk(1)).toThrow(/out of bounds/);
    expect(() => reader.get_chunk(999)).toThrow(/out of bounds/);
    expect(() => reader.get_chunk(-1)).toThrow(/out of bounds/);
  });
});

describe('get_toc', () => {
  it('returns empty array for no chunks', () => {
    const file = buildHonzo({});
    const reader = open(file);

    const toc = reader.get_toc();
    expect(toc).toEqual([]);
  });

  it('returns TOC entries for each chunk', () => {
    const file = buildHonzo({
      chunks: [
        { tag: 'CHAP', data: txt('Ch1'), content_type_kind: 1, content_type_value: 0 },
        { tag: 'NOTE', data: txt('N1'), content_type_kind: 1, content_type_value: 0 },
      ],
    });
    const reader = open(file);

    const toc = reader.get_toc();
    expect(toc.length).toBe(2);
    expect(toc[0].chunk_type).toBe('CHAP');
    expect(toc[0].chunk_id).toBe(0);
    expect(toc[0].size_raw).toBe(3);
    expect(toc[1].chunk_type).toBe('NOTE');
    expect(toc[1].chunk_id).toBe(1);
  });

  it('includes size_compressed and size_raw in TOC', () => {
    const data = txt('Some chapter content');
    const file = buildHonzo({
      chunks: [{ tag: 'CHAP', data, content_type_kind: 1, content_type_value: 0 }],
    });
    const reader = open(file);

    const toc = reader.get_toc();
    expect(toc[0].size_raw).toBe(data.length);
    expect(toc[0].size_compressed).toBe(data.length);
  });
});

describe('compression_name_for_chunk', () => {
  it('returns None for uncompressed chunks', () => {
    const file = buildHonzo({
      chunks: [{ tag: 'CHAP', data: [1, 2, 3], content_type_kind: 1, content_type_value: 0 }],
    });
    const reader = open(file);

    expect(reader.compression_name_for_chunk(0)).toBe('None');
  });

  it('returns Lz4 for compressed chunks', () => {
    const data = txt('Compressible '.repeat(20));
    const file = buildHonzo({
      chunks: [
        {
          tag: 'CHAP',
          data,
          compression: 1,
          content_type_kind: 1,
          content_type_value: 0,
        },
      ],
    });
    const reader = open(file);

    expect(reader.compression_name_for_chunk(0)).toBe('Lz4');
  });

  it('returns Unknown for out-of-bounds index', () => {
    const file = buildHonzo({});
    const reader = open(file);

    expect(reader.compression_name_for_chunk(0)).toBe('Unknown');
  });
});

describe('content_type_name_for_chunk', () => {
  it('returns Markdown for CHAP with markdown type', () => {
    const file = buildHonzo({
      chunks: [{ tag: 'CHAP', data: [1], content_type_kind: 1, content_type_value: 0 }],
    });
    const reader = open(file);

    expect(reader.content_type_name_for_chunk(0)).toBe('Markdown');
  });

  it('returns Html for CHAP with html type', () => {
    const file = buildHonzo({
      chunks: [{ tag: 'CHAP', data: [1], content_type_kind: 1, content_type_value: 1 }],
    });
    const reader = open(file);

    expect(reader.content_type_name_for_chunk(0)).toBe('Html');
  });

  it('returns MathML for MATH with mathml type', () => {
    const file = buildHonzo({
      chunks: [{ tag: 'MATH', data: [1], content_type_kind: 2, content_type_value: 0 }],
    });
    const reader = open(file);

    expect(reader.content_type_name_for_chunk(0)).toBe('MathML');
  });

  it('returns LaTeX for MATH with latex type', () => {
    const file = buildHonzo({
      chunks: [{ tag: 'MATH', data: [1], content_type_kind: 2, content_type_value: 1 }],
    });
    const reader = open(file);

    expect(reader.content_type_name_for_chunk(0)).toBe('LaTeX');
  });
});

describe('get_chapter_text', () => {
  it('returns chapter text for a CHAP chunk', () => {
    const data = txt('Hello world');
    const file = buildHonzo({
      chunks: [{ tag: 'CHAP', data, content_type_kind: 1, content_type_value: 0 }],
    });
    const reader = open(file);

    const texts = reader.get_chapters_text();
    expect(texts.length).toBe(1);
    expect(texts[0]).toBe('Hello world');
  });

  it('returns text for a NOTE chunk', () => {
    const data = txt('A footnote');
    const file = buildHonzo({
      chunks: [{ tag: 'NOTE', data, content_type_kind: 1, content_type_value: 0 }],
    });
    const reader = open(file);

    const texts = reader.get_chapters_text();
    expect(texts.length).toBe(1);
    expect(texts[0]).toBe('A footnote');
  });

  it('skips non-chapter chunks', () => {
    const file = buildHonzo({
      chunks: [
        { tag: 'CHAP', data: txt('Chapter 1'), content_type_kind: 1, content_type_value: 0 },
        {
          tag: 'IMG_',
          data: [1, 2, 3],
          content_type_kind: 1,
          content_type_value: 0,
          cover_type: 1,
        },
      ],
    });
    const reader = open(file);

    const texts = reader.get_chapters_text();
    expect(texts.length).toBe(1);
  });

  it('get_chapter_text returns text by index', () => {
    const data = txt('A chapter');
    const file = buildHonzo({
      chunks: [{ tag: 'CHAP', data, content_type_kind: 1, content_type_value: 0 }],
    });
    const reader = open(file);

    expect(reader.get_chapter_text(0)).toBe('A chapter');
  });

  it('strips HTML tags for HTML chapters', () => {
    const html = txt('<p>Hello <b>world</b></p>');
    const file = buildHonzo({
      chunks: [{ tag: 'CHAP', data: html, content_type_kind: 1, content_type_value: 1 }],
    });
    const reader = open(file);

    const text = reader.get_chapter_text(0);
    expect(text).toContain('Hello');
    expect(text).toContain('world');
    expect(text).not.toContain('<');
  });
});
