import { esc, direction } from "./state.js";

export function collectMeta() {
  const title = document.getElementById("mf_title")?.value?.trim() || "Untitled";
  const authors = (document.getElementById("mf_authors")?.value?.trim() || "")
    .split(",").map((s) => s.trim()).filter(Boolean);
  const language = document.getElementById("mf_language")?.value?.trim() || "en";
  const publisher = document.getElementById("mf_publisher")?.value?.trim() || undefined;
  const description = document.getElementById("mf_description")?.value?.trim() || undefined;
  const subtitle = document.getElementById("mf_subtitle")?.value?.trim() || undefined;
  const edition = document.getElementById("mf_edition")?.value?.trim() || undefined;
  const sourceUrl = document.getElementById("mf_source_url")?.value?.trim() || undefined;
  const license = document.getElementById("mf_license")?.value?.trim() || undefined;
  const origTitle = document.getElementById("mf_original_title")?.value?.trim() || undefined;
  const origLang = document.getElementById("mf_original_lang")?.value?.trim() || undefined;
  const origAuthors = (document.getElementById("mf_original_authors")?.value?.trim() || "")
    .split(",").map((s) => s.trim()).filter(Boolean);
  const wc = document.getElementById("mf_word_count")?.value?.trim();
  const rt = document.getElementById("mf_reading_time")?.value?.trim();
  const wordCount = wc ? parseInt(wc, 10) : undefined;
  const readingTime = rt ? parseInt(rt, 10) : undefined;
  const minVer = parseInt(document.getElementById("mf_minVer")?.value, 10) || 1;

  const meta = {
    title: { [language === "en" ? "en" : language]: title },
    language, direction, authors,
    ...(publisher && { publisher }),
    ...(description && { description: { [language]: description } }),
    ...(subtitle && { subtitle: { [language]: subtitle } }),
    ...(edition && { edition }),
    ...(sourceUrl && { source_url: sourceUrl }),
    ...(license && { license }),
    ...(origTitle && { original_title: origTitle }),
    ...(origLang && { original_lang: origLang }),
    ...(origAuthors.length > 0 && { original_authors: origAuthors }),
    ...(wordCount !== undefined && !isNaN(wordCount) && { word_count: wordCount }),
    ...(readingTime !== undefined && !isNaN(readingTime) && { reading_time_mins: readingTime }),
  };

  const seriesTitle = document.getElementById("mf_series_title")?.value?.trim();
  if (seriesTitle) {
    const seriesPos = document.getElementById("mf_series_pos")?.value?.trim() || "";
    const seriesArc = document.getElementById("mf_series_arc")?.value?.trim();
    meta.series = { title: seriesTitle, position: seriesPos, ...(seriesArc && { arc: seriesArc }) };
  }

  const idEls = document.querySelectorAll("#idList > .maker-id-row");
  const ids = [];
  for (const el of idEls) {
    const type = el.querySelector(".id-type")?.value?.trim();
    const value = el.querySelector(".id-value")?.value?.trim();
    if (type && value) ids.push({ id_type: type, value });
  }
  if (ids.length > 0) meta.identifiers = ids;

  const contribEls = document.querySelectorAll("#contributorList > .maker-contrib-row");
  const contributors = [];
  for (const el of contribEls) {
    const name = el.querySelector(".contrib-name")?.value?.trim();
    const role = el.querySelector(".contrib-role")?.value?.trim();
    if (name) contributors.push({ name, ...(role && { role }) });
  }
  if (contributors.length > 0) meta.contributors = contributors;

  const genres = collectTags("genres");
  const tags = collectTags("tags");
  if (genres.length > 0) meta.genres = genres;
  if (tags.length > 0) meta.tags = tags;

  return { meta, minVer };
}

export function collectTags(id) {
  const container = document.getElementById("tags_" + id);
  if (!container) return [];
  const items = [];
  for (const span of container.querySelectorAll(".tag")) {
    const text = span.querySelector(".tag-text")?.textContent?.trim() || span.textContent.replace("\u00d7", "").trim();
    if (text) items.push(text);
  }
  return items;
}

export function addIdentifierRow(type, value) {
  const container = document.getElementById("idList");
  const row = document.createElement("div");
  row.className = "maker-id-row";
  row.style.cssText = "display:flex;gap:8px;margin-bottom:6px;align-items:end";
  row.innerHTML = `
    <div style="flex:1"><input type="text" class="id-type" value="${esc(type || "")}" placeholder="uuid, isbn, doi..." style="width:100%;padding:6px 10px;border:1px solid var(--border);border-radius:6px;font-size:0.85rem;font-family:inherit" /></div>
    <div style="flex:2"><input type="text" class="id-value" value="${esc(value || "")}" placeholder="Value" style="width:100%;padding:6px 10px;border:1px solid var(--border);border-radius:6px;font-size:0.85rem;font-family:inherit" /></div>
    <button class="btn btn-secondary" data-remove="id" style="padding:4px 10px;font-size:0.8rem;height:auto;flex-shrink:0">\u00d7</button>`;
  container.appendChild(row);
}

export function addContributorRow(name, role) {
  const container = document.getElementById("contributorList");
  const row = document.createElement("div");
  row.className = "maker-contrib-row";
  row.style.cssText = "display:flex;gap:8px;margin-bottom:6px;align-items:end";
  row.innerHTML = `
    <div style="flex:2"><input type="text" class="contrib-name" value="${esc(name || "")}" placeholder="Name" style="width:100%;padding:6px 10px;border:1px solid var(--border);border-radius:6px;font-size:0.85rem;font-family:inherit" /></div>
    <div style="flex:1"><input type="text" class="contrib-role" value="${esc(role || "")}" placeholder="Role" style="width:100%;padding:6px 10px;border:1px solid var(--border);border-radius:6px;font-size:0.85rem;font-family:inherit" /></div>
    <button class="btn btn-secondary" data-remove="contrib" style="padding:4px 10px;font-size:0.8rem;height:auto;flex-shrink:0">\u00d7</button>`;
  container.appendChild(row);
}

export function addTag(id) {
  const input = document.getElementById(`new_${id}`);
  const value = input?.value?.trim();
  if (value) {
    const container = document.getElementById(`tags_${id}`);
    if (container) {
      container.insertAdjacentHTML("beforeend",
        `<span class="tag"><span class="tag-text">${esc(value)}</span> <span class="tag-remove" data-tag-id="${id}" data-tag-value="${esc(value)}">\u00d7</span></span>`);
    }
    input.value = "";
  }
}
