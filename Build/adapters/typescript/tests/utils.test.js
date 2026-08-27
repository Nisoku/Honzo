import { describe, expect, it, beforeAll } from '@jest/globals';
import { initHonzo, getHonzo } from './helpers.mjs';

beforeAll(async () => {
  await initHonzo();
});

function bytes(s) {
  return Array.from(new TextEncoder().encode(s));
}

describe('normalize_search_term', () => {
  it('lowercases input', () => {
    const { normalize_search_term: fn } = getHonzo();
    const result = fn('Hello World', 'en');
    expect(result).toBe('hello world');
  });

  it('handles empty string', () => {
    const { normalize_search_term: fn } = getHonzo();
    expect(fn('', 'en')).toBe('');
  });

  it('preserves internal spaces', () => {
    const { normalize_search_term: fn } = getHonzo();
    expect(fn('foo  bar', 'en')).toBe('foo  bar');
  });
});

describe('validate_mathml', () => {
  it('returns true for valid MathML', () => {
    const { validate_mathml: fn } = getHonzo();
    expect(fn(bytes('<math><mi>x</mi></math>'))).toBe(true);
  });

  it('returns true for mathematical text (considered valid)', () => {
    const { validate_mathml: fn } = getHonzo();
    expect(fn(bytes('x+y'))).toBe(true);
  });

  it('returns true for empty input (considered valid)', () => {
    const { validate_mathml: fn } = getHonzo();
    expect(fn([])).toBe(true);
  });
});

describe('latex_to_mathml', () => {
  it('converts simple LaTeX to MathML string', () => {
    const { latex_to_mathml: fn } = getHonzo();
    const result = fn(bytes('x^2'));
    expect(result).toBeTruthy();
    expect(typeof result).toBe('string');
    expect(result).toContain('<math');
  });

  it('converts E=mc^2 to MathML', () => {
    const { latex_to_mathml: fn } = getHonzo();
    const result = fn(bytes('E=mc^2'));
    expect(result).toBeTruthy();
    expect(result).toContain('<math');
  });
});

describe('render_math', () => {
  it('renders MathML input', () => {
    const { render_math: fn } = getHonzo();
    const result = fn(bytes('<math><mi>x</mi></math>'), 0);
    expect(result).toBeTruthy();
    expect(typeof result).toBe('string');
  });

  it('renders LaTeX input as MathML', () => {
    const { render_math: fn } = getHonzo();
    const result = fn(bytes('x^2'), 1);
    expect(result).toBeTruthy();
    expect(typeof result).toBe('string');
    expect(result).toContain('<math');
  });
});

describe('validate_css', () => {
  it('returns true for valid CSS', () => {
    const { validate_css: fn } = getHonzo();
    expect(fn(bytes('body { color: red; }'))).toBe(true);
  });

  it('returns true for empty CSS', () => {
    const { validate_css: fn } = getHonzo();
    expect(fn([])).toBe(true);
  });
});

describe('validate_font', () => {
  it('returns false for invalid font bytes', () => {
    const { validate_font: fn } = getHonzo();
    expect(fn([0, 0, 0, 0])).toBe(false);
  });

  it('returns false for empty input', () => {
    const { validate_font: fn } = getHonzo();
    expect(fn([])).toBe(false);
  });
});

describe('guess_font_format', () => {
  it('returns undefined for unrecognized bytes', () => {
    const { guess_font_format: fn } = getHonzo();
    expect(fn([0, 1, 2, 3])).toBeUndefined();
  });

  it('returns MIME type for WOFF2', () => {
    const { guess_font_format: fn } = getHonzo();
    const woff2 = [0x77, 0x4f, 0x46, 0x32];
    const result = fn(woff2);
    expect(result).toBe('font/woff2');
  });

  it('returns MIME type for WOFF', () => {
    const { guess_font_format: fn } = getHonzo();
    const woff = [0x77, 0x4f, 0x46, 0x46];
    expect(fn(woff)).toBe('font/woff');
  });

  it('returns MIME type for TTF', () => {
    const { guess_font_format: fn } = getHonzo();
    const ttf = [0x00, 0x01, 0x00, 0x00];
    expect(fn(ttf)).toBe('font/ttf');
  });

  it('returns MIME type for OTF', () => {
    const { guess_font_format: fn } = getHonzo();
    const otf = [0x4f, 0x54, 0x54, 0x4f];
    expect(fn(otf)).toBe('font/otf');
  });
});
