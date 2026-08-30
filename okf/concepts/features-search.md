---
type: concept
title: "Search Index"
description: "Full-text search via the SIDX inverted index chunk"
source: "https://nisoku.org/Honzo/features/search/"
path: /features/search/
updated: 2026-08-30
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-08-30T03:34:24.078Z"
---
---
title: "Search Index"
description: "Full-text search via the SIDX inverted index chunk"
---

The `SIDX` chunk type stores an inverted search index for full-text search across the book's chapters.

## Data format

The SIDX chunk contains a MessagePack encoded inverted index:

```python
{
  "version": 1,
  "terms": {
    "example": [
      {"chunk": 0, "positions": [42, 150]},
      {"chunk": 2, "positions": [17]}
    ],
    "hello": [
      {"chunk": 0, "positions": [5]}
    ],
    "world": [
      {"chunk": 0, "positions": [11]},
      {"chunk": 1, "positions": [88, 203]}
    ]
  }
}
```

### Tokenization rules

Terms are lowercased. Whitespace is the delimiter. Punctuation is stripped. Tokens shorter than 2 characters are excluded. Common stop words may be excluded according to application choice.

### Position semantics

`positions` are byte offsets within the decompressed chapter data. This lets readers find the term in the inverted index, get chunk index and byte offset pairs, seek to that position in the decompressed chapter, and show surrounding context.

## Building a search index

```rust
use honzo_chunks::sidx::SearchIndex;

let mut index = SearchIndex::new();
index.add_term("hello", 0, 5);
index.add_term("world", 0, 11);
index.add_term("example", 0, 42);

let sidx_data = index.encode().unwrap();
```

Or from the CLI:

```bash
honzo-cli make book.hzo --chapters ch1.md ch2.md \
  --build-index
```

## Searching

```typescript
import { createReader } from "@nisoku/honzo";

const reader = await createReader(buf);
const sidx = reader.getSearchIndex();

// Find all occurrences of "example"
const results = sidx.search("example");
for (const hit of results) {
  const chunk = reader.readChunk(hit.chunk);
  const text = new TextDecoder().decode(chunk);
  const context = text.substring(
    Math.max(0, hit.position - 40),
    hit.position + 40,
  );
  console.log(`Chapter ${hit.chunk}: ...${context}...`);
}
```

## Design notes

The SIDX chunk is optional. Books without search simply do not include it.

Term positions reference decompressed byte offsets. They stay stable across compression changes.

Multiple languages are supported via UTF-8 byte offsets. Word segmentation is an application level concern.
