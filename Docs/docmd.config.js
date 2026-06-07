export default {
  title: "Honzo",
  url: "https://nisoku.org/Honzo",
  logo: { alt: "Honzo", href: "./" },
  favicon: "",
  theme: {
    name: "ruby",
    defaultMode: "system",
    enableModeToggle: true,
    positionMode: "top",
    codeHighlight: true,
    customCss: ['/assets/css/theme.css'],
    copyWidgets: {
      enabled: true,
      raw: true,
      context: true,
    },
  },
  layout: {
    footer: {
      style: "complete",
      description: "A binary ebook format for simplicity, performance, and portability.",
      branding: true,
      columns: [
        {
          title: "Resources",
          links: [
            { text: "Getting Started", url: "./getting-started/quickstart" },
            { text: "Format Spec", url: "./format/" },
            { text: "CLI Commands", url: "./cli/" },
          ],
        },
        {
          title: "Community",
          links: [
            { text: "GitHub", url: "https://github.com/Nisoku/Honzo" },
            { text: "Issues", url: "https://github.com/Nisoku/Honzo/issues" },
            { text: "Discussions", url: "https://github.com/Nisoku/Honzo/discussions" },
          ],
        },
      ],
    },
  },
  plugins: {
    search: {
      semantic: true,
      showConfidence: true,
    },
    seo: {
      defaultDescription:
        "Honzo is a binary ebook format for simplicity, performance, and portability. Zero-copy parsing, pull-based streaming, per-chunk compression.",
      openGraph: { defaultImage: "" },
      twitter: { cardType: "summary_large_image" },
    },
    sitemap: {
      defaultChangefreq: "weekly",
      defaultPriority: 0.8,
    },
    analytics: {},
    mermaid: {},
    git: {},
    llms: {
      fullContext: true,
    },
  },
  search: true,
  minify: true,
  autoTitleFromH1: true,
  copyCode: true,
  pageNavigation: true,
  navigation: [
    { title: "Home", path: "/", icon: "home" },
    {
      title: "Getting Started",
      icon: "rocket",
      collapsible: false,
      children: [
        { title: "Quick Start", path: "/getting-started/quickstart", icon: "play" },
        { title: "Installation", path: "/getting-started/installation", icon: "download" },
        { title: "Core Concepts", path: "/getting-started/concepts", icon: "book" },
      ],
    },
    {
      title: "Format Specification",
      icon: "file-text",
      path: "/format/",
      collapsible: false,
      children: [
        { title: "Wire Format", path: "/format/wire-format", icon: "binary" },
        { title: "Chunk Types", path: "/format/chunk-types", icon: "package" },
        { title: "Compression", path: "/format/compression", icon: "zap" },
        { title: "DRM & Encryption", path: "/format/drm", icon: "lock" },
        { title: "Layout Modes", path: "/format/layout", icon: "columns" },
      ],
    },
    {
      title: "API Reference",
      icon: "code",
      path: "/api/",
      collapsible: false,
      children: [
        { title: "Rust", path: "/api/rust", icon: "box" },
        { title: "WASM / TypeScript", path: "/api/wasm", icon: "cpu" },
        { title: "C", path: "/api/c", icon: "terminal" },
      ],
    },
    {
      title: "CLI Reference",
      icon: "terminal",
      path: "/cli/",
      collapsible: false,
    },
    {
      title: "Features",
      icon: "star",
      path: "/features/",
      collapsible: false,
      children: [
        { title: "Annotations", path: "/features/annotations", icon: "pen-tool" },
        { title: "Search Index", path: "/features/search", icon: "search" },
        { title: "Sync Tracks", path: "/features/sync", icon: "clock" },
        { title: "Streaming", path: "/features/streaming", icon: "wind" },
      ],
    },
    {
      title: "Conversion",
      icon: "git-merge",
      path: "/conversion/",
      collapsible: false,
      children: [
        { title: "EPUB", path: "/conversion/epub", icon: "book" },
        { title: "MOBI", path: "/conversion/mobi", icon: "book-open" },
        { title: "PDF", path: "/conversion/pdf", icon: "file-text" },
      ],
    },
    {
      title: "Demo Apps",
      icon: "eye",
      path: "/demo-docs/",
      collapsible: false,
      children: [
        { title: "Reader", path: "/demo/", icon: "book", external: true },
        { title: "Maker", path: "/demo/maker.html", icon: "wrench", external: true },
        { title: "Inspect", path: "/demo/inspect.html", icon: "search", external: true },
        { title: "Convert", path: "/demo/convert.html", icon: "git-merge", external: true },
      ],
    },
    {
      title: "Contributing",
      icon: "heart",
      path: "/contributing/",
      collapsible: false,
      children: [
        { title: "Building", path: "/contributing/building", icon: "network" },
        { title: "Architecture", path: "/contributing/architecture", icon: "git-commit" },
      ],
    },
    {
      title: "GitHub",
      path: "https://github.com/Nisoku/Honzo",
      icon: "github",
      external: true,
    },
  ],
  footer: "Built with [docmd](https://docmd.io). [View on GitHub](https://github.com/Nisoku/Honzo).",
  editLink: {
    enabled: true,
    baseUrl: "https://github.com/Nisoku/Honzo/edit/main/Docs/docs",
    text: "Edit this page",
  },
};
