---
type: concept
title: Features
description: "Advanced Honzo features: annotations, search, sync, and streaming"
source: "https://nisoku.org/Honzo/features/"
path: /features/
updated: 2026-08-30
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-08-30T04:03:40.901Z"
---
---
title: "Features"
description: "Advanced Honzo features: annotations, search, sync, and streaming"
---

Beyond the core format, Honzo provides several advanced features for modern ebook experiences.

## Available features

::: grids
::: grid
::: card "Annotations" icon:bookmark
Portable highlights, bookmarks, and notes via `org.nisoku.anno`.

::: button "Read more" ./annotations.md icon:arrow-right
:::
:::
::: grid
::: card "Search Index" icon:search
Inverted term index via `org.nisoku.sidx` for full-text search.

::: button "Read more" ./search.md icon:arrow-right
:::
:::
::: grid
::: card "Sync Tracks" icon:music
Audio, video, and animation synchronization via `org.nisoku.sync`.

::: button "Read more" ./sync.md icon:arrow-right
:::
:::
::: grid
::: card "Streaming" icon:play
Pull-based chapter decoding for minimal memory usage.

::: button "Read more" ./streaming.md icon:arrow-right
:::
:::
:::

## Feature storage

All features are stored as EXTRA entries or special chunk types within the Honzo file. This has three benefits.

::: grids
::: grid
::: card "Portability"
Sync tracks, annotations, and search indices travel with the file.
:::
:::
::: grid
::: card "Editability"
Append only via tail mutability. No data section rewrite needed.
:::
:::
::: grid
::: card "Extensibility"
Custom namespaces are preserved on round-trip.
:::
:::
:::
