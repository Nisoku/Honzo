# TODO

- [ ] Web Converter freezes on larger files, make non-blocking
- [ ] Fix Content Chunks expanding based on chunk count rather than showing a scrolling list
- [ ] Migrate TypeScript adapter to modern Prettier config
- [ ] Move TS adapter tests to tests/ folder (+ keys/)
- [ ] implement pagination in web reader

## Comic/Manga/etc Support

- [ ] CBZ converter
- [ ] `org.nisoku.comic` EXTRA chunk (stuff from the `ComicInfo` spec: AlternateSeries, Teams, SeriesGroup, Page.Type per image)
- [ ] Add general fields to HonzoMeta struct (Count/Volume on SeriesMeta, Characters/Locations on HonzoMeta, monochrome on RenderHints or HonzoMeta, community_rating)
- [ ] Build the CBZ converter (cbz.rs): ZIP read, image filter/sort, ComicInfo.xml parse, HonzoBuilder assembly, CLI dispatch
- [ ] Update conversion docs with CBZ format card, steps, and metadata mapping
