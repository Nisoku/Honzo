import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';

const __dirname = dirname(fileURLToPath(import.meta.url));

let _honzo = null;

export async function initHonzo() {
  if (_honzo) return _honzo;
  const wasmBytes = readFileSync(resolve(__dirname, 'wasm/honzo_wasm_bg.wasm'));
  _honzo = await import('./wasm/honzo_wasm.js');
  _honzo.initSync({ module: wasmBytes });
  return _honzo;
}

export function getHonzo() {
  if (!_honzo) throw new Error('initHonzo() must be called first');
  return _honzo;
}

// MsgPack helpers

function concat(arrays) {
  const totalLen = arrays.reduce((s, a) => s + a.length, 0);
  const result = new Uint8Array(totalLen);
  let offset = 0;
  for (const a of arrays) {
    result.set(a, offset);
    offset += a.length;
  }
  return result;
}

export function encodeStr(s) {
  const bytes = new TextEncoder().encode(s);
  const buf = new Uint8Array(1 + bytes.length);
  buf[0] = 0xa0 | bytes.length;
  buf.set(bytes, 1);
  return buf;
}

export function encodeMap(entries) {
  const pairs = [];
  for (const [k, v] of entries) {
    pairs.push(encodeStr(k), v);
  }
  return concat([new Uint8Array([0x80 | entries.length]), ...pairs]);
}

export function encodeArray(items) {
  return concat([new Uint8Array([0x90 | items.length]), ...items]);
}

export function encodeUint(n) {
  if (typeof n === 'bigint') {
    const buf = new Uint8Array(9);
    buf[0] = 0xcf;
    new DataView(buf.buffer).setBigUint64(1, n, false);
    return buf;
  }
  if (n >= 0 && n <= 0x7f) return new Uint8Array([n]);
  if (n <= 0xffff) {
    const buf = new Uint8Array(3);
    buf[0] = 0xcd;
    buf[1] = (n >> 8) & 0xff;
    buf[2] = n & 0xff;
    return buf;
  }
  if (n <= 0xffffffff) {
    const buf = new Uint8Array(5);
    buf[0] = 0xce;
    new DataView(buf.buffer).setUint32(1, n, false);
    return buf;
  }
  throw new Error('number too large');
}

export function encodeNil() {
  return new Uint8Array([0xc0]);
}

export function u8toArray(u8) {
  return Array.from(u8);
}

// Build helpers

export function buildAnnotations(annos) {
  return u8toArray(
    encodeArray(
      annos.map(({ chunkId, offset, length, type, note, color }) =>
        encodeMap([
          ['chunk_id', encodeUint(chunkId)],
          ['offset', encodeUint(offset)],
          ['length', encodeUint(length)],
          ['type', encodeStr(type)],
          ...(note !== undefined && note !== null ? [['note', encodeStr(note)]] : []),
          ...(color !== undefined && color !== null ? [['color', encodeStr(color)]] : []),
        ]),
      ),
    ),
  );
}

export function buildSyncCues(cues) {
  return u8toArray(
    encodeArray(
      cues.map((c) =>
        encodeMap([
          ['sync_type', encodeUint(c.syncType ?? 0)],
          ['chunk_id', encodeUint(c.chunkId)],
          ['offset', encodeUint(c.offset)],
          ['timestamp_ms', encodeUint(c.timestampMs)],
          ...(c.mediaId !== undefined && c.mediaId !== null
            ? [['media_id', encodeStr(c.mediaId)]]
            : []),
          ...(c.durationMs !== undefined && c.durationMs !== null
            ? [['duration_ms', encodeUint(c.durationMs)]]
            : []),
        ]),
      ),
    ),
  );
}

export function buildDrmEnvelope(scheme, encryptedChunks, keyEnvelope, licenseUrl, expiresAt) {
  const entries = [
    ['scheme', encodeStr(scheme)],
    ['encrypted_chunks', encodeArray((encryptedChunks || []).map((id) => encodeUint(id)))],
    ['key_envelope', encodeArray((keyEnvelope || []).map((b) => encodeUint(b)))],
  ];
  if (licenseUrl !== undefined && licenseUrl !== null) {
    entries.push(['license_url', encodeStr(licenseUrl)]);
  }
  if (expiresAt !== undefined && expiresAt !== null) {
    entries.push(['expires_at', encodeUint(expiresAt)]);
  }
  return u8toArray(encodeMap(entries));
}

export function buildExtraEntry(namespace, body) {
  const nsBytes = new TextEncoder().encode(namespace.padEnd(4, '\x00').slice(0, 4));
  const lenBuf = new Uint8Array(4);
  new DataView(lenBuf.buffer).setUint32(0, body.length, true);
  return u8toArray(concat([nsBytes, lenBuf, new Uint8Array(body)]));
}

export function buildHonzo({ meta, chunks, annotations, syncCues, extra, flags, layout, drm, pmapEntries }) {
  const honzo = getHonzo();
  return honzo.honzo_build({
    chunks: chunks || [],
    meta: meta || { title: { en: 'Test' }, authors: ['Tester'], language: 'en' },
    annotations: annotations || null,
    sync_cues: syncCues || null,
    pmap_entries: pmapEntries || null,
    extra: extra || null,
    flags: flags || 0,
    language: 'en',
    auto_sidx: false,
    auto_covt: false,
    layout: layout ?? 0,
    drm: drm || null,
  });
}

export function open(file) {
  return new (getHonzo().HonzoWasm)(file, 1);
}

export function openWithPrivateKey(file, privateKeyDer) {
  return getHonzo().HonzoWasm.with_private_key(file, 1, privateKeyDer);
}
