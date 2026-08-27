---
type: concept
title: "Format Specification"
description: "Honzo binary format overview"
source: "https://nisoku.org/Honzo/format/"
path: /format/
updated: 2026-08-27
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-08-27T06:42:02.796Z"
---
---
title: "Format Specification"
description: "Honzo binary format overview"
---

This section covers the Honzo binary format at the byte level. You do not need this information to use Honzo. You need it to implement a parser or builder in a language without one.

## File Layout

A Honzo file has six sections in sequence:

```txt
Offset  Section    Size        Description
------  -------    ----        -----------
0       HEAD       48          File header with magic, version, counts, and offsets
52      TOC        32 x N      Table of contents where N equals chunk_count
52+N*32 DATA       variable    Chunk payload data
        EXTRA      variable    Extra entries for extensible metadata
        META       variable    Book metadata encoded as MessagePack
```

## Key Design Decisions

::: grids
::: grid
::: card "Fixed Size Entries" icon:binary
HEAD and TOC entries are fixed size. Parse by pointer cast. No allocation.
:::
:::
::: grid
::: card "Absolute Offsets" icon:map-pin
Every section header contains absolute byte offsets. Seek and read without scanning.
:::
:::
::: grid
::: card "META Is Last" icon:edit
Append only metadata edits. Edit the title without touching a single byte of content.
:::
:::
::: grid
::: card "EXTRA Between DATA and META" icon:layers
Extensible entries for annotations, DRM, and sync. Preserves append semantics.
:::
:::
:::

## Byte Order

All multi byte integers are little endian. This matches x86, ARM, and WASM.

## Pages in This Section

::: grids
::: grid
::: button "Wire Format" ./wire-format.md icon:binary
:::
::: grid
::: button "Chunk Types" ./chunk-types.md icon:package
:::
::: grid
::: button "Compression" ./compression.md icon:zap
:::
::: grid
::: button "DRM & Encryption" ./drm.md icon:lock
:::
::: grid
::: button "Layout Modes" ./layout.md icon:columns
:::
:::
