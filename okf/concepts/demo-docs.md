---
type: concept
title: "Demo Apps"
description: "Web-based demo applications for the Honzo ecosystem"
source: "https://nisoku.org/Honzo/demo-docs/"
path: /demo-docs/
updated: 2026-08-27
okf:
  generated_by: "@docmd/plugin-okf"
  generated_at: "2026-08-27T06:29:06.905Z"
---
---
title: "Demo Apps"
description: "Web-based demo applications for the Honzo ecosystem"
---

The Honzo demo suite provides four web-based applications for interacting with Honzo files. All demos use the WASM binding for in-browser parsing.

## Demos

::: grids
::: grid
::: card "Reader" icon:book
Full-featured ebook reader with chapter navigation, cover rendering, and layout adaptation.

- Chapter navigation with sidebar TOC
- Multi-language metadata display
- Cover image rendering
- Chapter content rendering (Markdown/HTML)
- Layout mode adaptation (reflowable/fixed/scroll)
- Page turn animations

Built with [Sairin](https://nisoku.org/Sairin/) (reactive UI) and [Satori](https://nisoku.org/Satori/docs/) (observability/logging).

::: button "Open Reader" ../demo/ icon:external-link
:::
:::
::: grid
::: card "Maker" icon:wrench
Online GUI for building Honzo files interactively.

- Chapter editor with Markdown preview
- Metadata editor (title, author, language)
- Cover upload
- Per-chapter compression control
- Drag-and-drop chapter reordering
- Download the resulting `.hzo` file

::: button "Open Maker" ../demo/maker.html icon:external-link
:::
:::
::: grid
::: card "Inspect" icon:search
Low-level Honzo file inspector for debugging and learning the format.

- Hex view of HEAD + TOC sections
- Decoded field display (version, chunk count, layout mode)
- Per-chunk detail (tag, compression, size, offset)
- EXTRA section browser
- META section JSON viewer
- Chunk data hex dump

::: button "Open Inspect" ../demo/inspect.html icon:external-link
:::
:::
::: grid
::: card "Convert" icon:git-merge
Browser-based EPUB to Honzo converter.

- Drag-and-drop EPUB upload
- Conversion progress indicator
- Metadata preview before conversion
- Download the resulting `.hzo` file
- All processing is client-side (no server upload)

::: button "Open Convert" ../demo/convert.html icon:external-link
:::
:::
:::

## Building the demos

```bash
just demo build       # production build (includes WASM)
just demo check       # test + build
```

For development:

```bash
cd Demo
npm install
npm run dev           # development server
```

The demos are separate pages under the same Vite project. Navigate between them from the demo menu.

## Next Steps

::: grids
::: grid
::: button "CLI Reference" ../cli/index.md icon:terminal
:::
::: grid
::: button "Format Specification" ../format/ icon:book
:::
:::
