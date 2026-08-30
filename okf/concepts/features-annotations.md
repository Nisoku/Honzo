---
type: concept
title: Annotations
description: "Portable highlights, bookmarks, and notes via org.nisoku.anno"
source: "https://nisoku.org/Honzo/features/annotations/"
path: /features/annotations/
updated: 2026-08-30
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-08-30T03:37:06.089Z"
---
---
title: "Annotations"
description: "Portable highlights, bookmarks, and notes via org.nisoku.anno"
---

The `org.nisoku.anno` EXTRA namespace stores highlights, bookmarks, and notes directly in the Honzo file. Annotations travel with the ebook and survive re-conversion.

## Data format

Annotations are stored as a MessagePack map:

```python
{
  "version": 1,
  "annotations": [
    {
      "id": "uuid-annotation-1",
      "type": "highlight",
      "chunk": 0,
      "start": 1024,
      "end": 1080,
      "text": "The highlighted text excerpt",
      "note": "Reader's note about this passage",
      "color": "#FFEB3B",
      "created": "2025-01-15T10:30:00Z",
      "modified": "2025-01-15T10:30:00Z"
    }
  ]
}
```

### Annotation types

| Type        | Description                                 |
| ----------- | ------------------------------------------- |
| `highlight` | Text highlight with start/end byte offsets  |
| `bookmark`  | Position bookmark (page or location marker) |
| `note`      | A note attached to a specific position      |
| `underline` | Underlined text passage                     |
| `drawing`   | Freeform drawing on a page                  |

### Fields

| Field      | Type      | Description                           |
| ---------- | --------- | ------------------------------------- |
| `id`       | `string`  | UUID                                  |
| `type`     | `string`  | Annotation type                       |
| `chunk`    | `int`     | TOC index of the annotated chunk      |
| `start`    | `int`     | Byte offset within decompressed chunk |
| `end`      | `int`     | End byte offset                       |
| `text`     | `string`  | The annotated text excerpt            |
| `note`     | `string?` | Reader's note                         |
| `color`    | `string?` | Display color (hex)                   |
| `created`  | `string`  | ISO 8601 timestamp                    |
| `modified` | `string?` | Last modification timestamp           |

## Storing annotations

```rust
use honzo_io::{HonzoBuilder, ExtraEntry};

// Add annotation data as an EXTRA entry
let anno_data = serde_json::to_vec(&annotation_map).unwrap();
let hzo = HonzoBuilder::new()
    .add_extra("org.nisoku.anno", &anno_data)
    .add_chapter("Chapter 1", ...)
    .finalize()
    .unwrap();
```

## Reading annotations

```rust
use honzo_core::HonzoParser;

let p = HonzoParser::new(&data, 1).unwrap();
for entry in p.extra_entries() {
    if entry.namespace() == "org.nisoku.anno" {
        let anno_bytes = p.read_extra(&entry).unwrap();
        // Parse as MessagePack or JSON
    }
}
```

## Design notes

Annotations reference chunks by TOC index. They do not reference chapter titles. This choice survives title changes.

Byte offsets are relative to the decompressed chunk data. If compression changes, offsets stay valid.

Multiple annotation entries are merged at read time by the consuming application.

Applications should preserve unknown annotation types rather than dropping them.
