---
type: api
title: "WASM / TypeScript API Reference"
description: "The TypeScript API for Honzo in browsers and Node.js via WebAssembly"
source: "https://nisoku.org/Honzo/api/wasm/"
path: /api/wasm/
updated: 2026-09-05
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-09-05T22:08:41.759Z"
---
---
title: "WASM / TypeScript API Reference"
description: "The TypeScript API for Honzo in browsers and Node.js via WebAssembly"
---

The WASM binding wraps `honzo-core` and `honzo-io` into a JavaScript API usable from browsers and Node.js.

## Installation

```bash
npm install @nisoku/honzo
```

## Exported types

- `createReader` -- Parse a Honzo file from bytes
- `buildHonzo` -- Build a Honzo file from chunks
- `HonzoReader` -- Reader instance
- `HonzoHead` -- File header fields
- `HonzoMeta` -- Metadata fields
- `TocEntry` -- TOC entry fields
- `DrmKeyPair` -- DRM key pair for encryption
- `BuildConfig` -- Builder configuration

## createReader

| Symbol         | Signature                                | Returns                | Notes               |
| -------------- | ---------------------------------------- | ---------------------- | ------------------- |
| `createReader` | `(buffer: Uint8Array, version?: number)` | `Promise<HonzoReader>` | Parse a Honzo file. |

## HonzoReader

| Symbol                 | Signature                  | Returns      | Notes                                     |
| ---------------------- | -------------------------- | ------------ | ----------------------------------------- |
| `reader.chunkCount`    | `get`                      | `number`     | Number of chunks.                         |
| `reader.layoutMode`    | `get`                      | `number`     | `0`=reflowable, `1`=fixed, `2`=scroll.    |
| `reader.formatVersion` | `get`                      | `number`     | Format version.                           |
| `reader.getMeta`       | `()`                       | `HonzoMeta`  | Book metadata (title, creator, etc.).     |
| `reader.readChunk`     | `(index: number)`          | `Uint8Array` | Read and decompress a chunk by TOC index. |
| `reader.readChunkRaw`  | `(index: number)`          | `Uint8Array` | Read chunk without decompression.         |
| `reader.getTocEntry`   | `(index: number)`          | `TocEntry`   | Get TOC entry metadata.                   |
| `reader.withDrmKey`    | `(privateKey: Uint8Array)` | `void`       | Set X25519 private key for DRM.           |

## HonzoMeta

| Field            | Type                                  | Description                |
| ---------------- | ------------------------------------- | -------------------------- |
| `title`          | `Record<string, string> \| undefined` | Localized title map.       |
| `creator`        | `Record<string, string> \| undefined` | Localized creator map.     |
| `language`       | `string \| undefined`                 | Primary language (BCP 47). |
| `description`    | `Record<string, string> \| undefined` | Localized description.     |
| `publisher`      | `string \| undefined`                 | Publisher name.            |
| `published`      | `string \| undefined`                 | Publication date.          |
| `rights`         | `string \| undefined`                 | Copyright / license text.  |
| `subject`        | `string[] \| undefined`               | Subject categories.        |
| `series`         | `string \| undefined`                 | Series name.               |
| `seriesPosition` | `number \| undefined`                 | Position in series.        |
| `edition`        | `number \| undefined`                 | Edition number.            |

## TocEntry

| Field         | Type     | Description                             |
| ------------- | -------- | --------------------------------------- |
| `chunkType`   | `string` | 4-byte type tag (e.g., `"CHAP"`).       |
| `compression` | `number` | `0`=none, `1`=lz4.                      |
| `markupType`  | `number` | `0`=markdown, `1`=html.                 |
| `size`        | `number` | Stored size (compressed if applicable). |
| `origSize`    | `number` | Uncompressed size.                      |

## buildHonzo

| Symbol       | Signature               | Returns               | Notes                           |
| ------------ | ----------------------- | --------------------- | ------------------------------- |
| `buildHonzo` | `(config: BuildConfig)` | `Promise<Uint8Array>` | Build a Honzo file from config. |

### BuildConfig

| Field        | Type                     | Default     | Description            |
| ------------ | ------------------------ | ----------- | ---------------------- |
| `chunks`     | `ChunkInput[]`           | required    | Array of chunk inputs. |
| `meta`       | `Partial<HonzoMeta>`     | `{}`        | Book metadata.         |
| `layoutMode` | `number`                 | `0`         | Layout mode.           |
| `drm`        | `DrmConfig \| undefined` | `undefined` | DRM configuration.     |

### ChunkInput

| Field         | Type         | Description                  |
| ------------- | ------------ | ---------------------------- |
| `type`        | `string`     | 4-byte type tag.             |
| `data`        | `Uint8Array` | Chunk content.               |
| `compression` | `number`     | `0`=none, `1`=lz4.           |
| `markupType?` | `number`     | Markup type for CHAP chunks. |
| `coverType?`  | `number`     | Cover type for COVR/COVT.    |

### DrmConfig

| Field             | Type          | Description                 |
| ----------------- | ------------- | --------------------------- |
| `encryptedChunks` | `number[]`    | TOC indices to encrypt.     |
| `recipients`      | `Recipient[]` | Authorized recipients.      |
| `licenseUrl?`     | `string`      | Optional license URL.       |
| `expiry?`         | `string`      | Optional expiry (ISO 8601). |

### Recipient

| Field       | Type         | Description                   |
| ----------- | ------------ | ----------------------------- |
| `publicKey` | `Uint8Array` | X25519 public key (32 bytes). |
| `id`        | `string`     | Recipient identifier.         |

## Error handling

Async functions throw `HonzoError` with a `code` property:

| Code                                     | Meaning                    |
| ---------------------------------------- | -------------------------- |
| ::: tag "InvalidMagic" color:#ef4444     | Not a Honzo file           |
| ::: tag "InvalidVersion" color:#ef4444   | Unsupported format version |
| ::: tag "DrmError" color:#ef4444         | DRM-related failure        |
| ::: tag "CompressionError" color:#ef4444 | Decompression failure      |

## Example

```typescript
import { createReader, buildHonzo } from "@nisoku/honzo";

// Read
const response = await fetch("book.hzo");
const buf = new Uint8Array(await response.arrayBuffer());
const reader = await createReader(buf);
console.log(reader.chunkCount, reader.getMeta().title?.en);

// Build
const hzo = await buildHonzo({
  meta: { title: { en: "My Book" } },
  chunks: [
    {
      type: "CHAP",
      data: new TextEncoder().encode("# Hello"),
      compression: 1,
      markupType: 0,
    },
  ],
});
```

## Related

::: grids
::: grid
::: button "Rust API" ./rust.md icon:code
:::
::: grid
::: button "C API" ./c.md icon:code
:::
::: grid
::: button "Format Specification" ../format/ icon:book
:::
:::
