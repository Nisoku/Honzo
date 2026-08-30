---
type: concept
title: "Layout Modes"
description: "Reflowable, fixed, and scroll layout modes in Honzo"
source: "https://nisoku.org/Honzo/format/layout/"
path: /format/layout/
updated: 2026-08-30
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-08-30T03:55:13.922Z"
---
---
title: "Layout Modes"
description: "Reflowable, fixed, and scroll layout modes in Honzo"
---

Honzo supports three layout modes. The mode is set in the HEAD `layout_mode` field and applies to the entire file.

## Modes

| Code | Mode       | Description                                                                    |
| ---- | ---------- | ------------------------------------------------------------------------------ |
| 0    | Reflowable | Content wraps to fit the viewport. This is the typical ebook experience.       |
| 1    | Fixed      | Content uses fixed coordinates. This resembles a PDF.                          |
| 2    | Scroll     | Content appears as one continuous scrollable flow. This works like a web page. |

## Reflowable (0)

This is the default. Text reflows to fit the reader's screen size, font preference, and orientation.

- Chapters render one at a time.
- The reader can change text size and font.
- Images scale to fit the viewport.
- The reader application handles pagination.

Use this for novels, text heavy books, and anything that should adapt to the device.

## Fixed (1)

Content uses fixed positions and sizes, typically measured in pixels. Each chapter corresponds to a page with known dimensions.

- All content has absolute positions.
- Font size and layout cannot change.
- The reader application renders each chapter as is.
- This mode works for comics, children's books, and textbooks with complex layouts.

## Scroll (2)

All chapters combine into a single scrollable view.

- No page breaks appear between chapters.
- The reader application typically shows a continuous scroll.
- This mode works for long form articles, documentation, and web first content.
- Search and annotation span all chapters seamlessly.

## Reading Layout Modes

```rust
use honzo_core::HonzoParser;

let p = HonzoParser::new(&data, 1).unwrap();
match p.head().layout_mode {
    0 => println!("Reflowable"),
    1 => println!("Fixed layout"),
    2 => println!("Scroll"),
    _ => println!("Unknown mode"),
}
```

```typescript
import { createReader } from "@nisoku/honzo";
const reader = await createReader(buf);
// reader.layoutMode: 0 = reflowable, 1 = fixed, 2 = scroll
```

## Next Steps

::: grids
::: grid
::: button "Wire Format" ./wire-format.md icon:binary
:::
::: grid
::: button "Chunk Types" ./chunk-types.md icon:package
:::
:::
