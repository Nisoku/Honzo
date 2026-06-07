import init, { HonzoWasm, honzo_build } from "../wasm/honzo_wasm.js";
import { createIcons } from "lucide";
import { icons } from "../icons.js";
import { bindClass, bindEvent, bindText } from "@nisoku/sairin";
import { derived, path, signal } from "@nisoku/sairin";
import { esc } from "../shared/esc.js";
import { formatSize } from "../shared/format.js";
import { download } from "../shared/download.js";

let wasmReady = false;
let reader = null;

const fileLoaded = signal(path("inspect", "fileLoaded"), false);
const fileName = signal(path("inspect", "fileName"), "");
const fileSize = signal(path("inspect", "fileSize"), 0);
const fileInfoData = signal(path("inspect", "fileInfoData"), null);
const tocData = signal(path("inspect", "tocData"), []);
const metaData = signal(path("inspect", "metaData"), null);
const extraData = signal(path("inspect", "extraData"), null);
const chunksData = signal(path("inspect", "chunksData"), []);
const originalMeta = signal(path("inspect", "originalMeta"), null);

const statusVisible = signal(path("inspect", "statusVisible"), false);
const statusKind = signal(path("inspect", "statusKind"), "");
const statusMessage = signal(path("inspect", "statusMessage"), "");

const statusClass = derived(
  path("inspect", "statusClass"),
  () => `status${statusVisible.get() ? ` active ${statusKind.get()}` : ""}`,
);

const filePanelClass = derived(
  path("inspect", "filePanelClass"),
  () => `panel${fileLoaded.get() ? " visible" : ""}`,
);
const tocPanelClass = derived(
  path("inspect", "tocPanelClass"),
  () => `panel${fileLoaded.get() ? " visible" : ""}`,
);
const metaPanelClass = derived(
  path("inspect", "metaPanelClass"),
  () => `panel${fileLoaded.get() ? " visible" : ""}`,
);
const extraPanelClass = derived(
  path("inspect", "extraPanelClass"),
  () =>
    `panel${fileLoaded.get() && extraData.get()?.length > 0 ? " visible" : ""}`,
);

const saveDisabled = derived(
  path("inspect", "saveDisabled"),
  () => !fileLoaded.get(),
);
const revertDisabled = derived(
  path("inspect", "revertDisabled"),
  () => !fileLoaded.get(),
);

const dropZone = document.getElementById("dropZone");
const fileInput = document.getElementById("fileInput");
const statusEl = document.getElementById("status");
const statusTextEl = document.getElementById("statusText");
const filePanel = document.getElementById("filePanel");
const fileInfo = document.getElementById("fileInfo");
const tocPanel = document.getElementById("tocPanel");
const chunkCount = document.getElementById("chunkCount");
const tocBody = document.getElementById("tocBody");
const metaPanel = document.getElementById("metaPanel");
const metaFields = document.getElementById("metaFields");
const extraPanel = document.getElementById("extraPanel");
const extraInfo = document.getElementById("extraInfo");
const revertBtn = document.getElementById("revertBtn");
const saveBtn = document.getElementById("saveBtn");

bindClass(statusEl, statusClass);
bindText(statusTextEl, statusMessage);
bindClass(filePanel, filePanelClass);
bindClass(tocPanel, tocPanelClass);
bindClass(metaPanel, metaPanelClass);
bindClass(extraPanel, extraPanelClass);

const origBindDisabled = (el, signal) => {
  const update = () => (el.disabled = signal.get());
  update();
  signal.subscribe?.(update) ?? (signal._onSet = update);
};

origBindDisabled(saveBtn, saveDisabled);
origBindDisabled(revertBtn, revertDisabled);

bindEvent(dropZone, "click", () => fileInput.click());
bindEvent(dropZone, "dragover", (e) => {
  e.preventDefault();
  dropZone.classList.add("dragover");
});
bindEvent(dropZone, "dragleave", () => dropZone.classList.remove("dragover"));
bindEvent(dropZone, "drop", (e) => {
  e.preventDefault();
  dropZone.classList.remove("dragover");
  loadFile(e.dataTransfer?.files?.[0]);
});
bindEvent(fileInput, "change", (e) => {
  if (e.target.files?.[0]) loadFile(e.target.files[0]);
});
bindEvent(saveBtn, "click", onSave);
bindEvent(revertBtn, "click", onRevert);

createIcons({ icons });

bindEvent(metaPanel, "click", (e) => {
  const addId = e.target.closest("[data-add-id]");
  if (addId) {
    const container = document.getElementById("idList");
    if (container) {
      container.insertAdjacentHTML("beforeend", `
      <div class="field" style="display:flex;gap:0.5rem;align-items:end">
        <div style="flex:1"><label>Type</label><input type="text" class="id-type" value="" /></div>
        <div style="flex:2"><label>Value</label><input type="text" class="id-value" value="" /></div>
        <button class="btn btn-secondary" data-remove="inspect-id" style="padding:0.4rem 0.6rem;font-size:0.8rem">×</button>
      </div>`);
    }
  }
  const rem = e.target.closest("[data-remove]");
  if (rem) rem.closest(".field")?.remove();
  const tagBtn = e.target.closest("[data-tag-add]");
  if (tagBtn) {
    const id = tagBtn.dataset.tagAdd;
    const input = document.getElementById(`new_${id}`);
    const value = input?.value?.trim();
    if (value) {
      const container = document.getElementById(`tags_${id}`);
      if (container) {
        container.insertAdjacentHTML("beforeend",
          `<span class="tag"><span class="tag-text">${esc(value)}</span> <span class="tag-remove" data-tag-id="${id}">×</span></span>`);
      }
      input.value = "";
    }
  }
  const tagRem = e.target.closest(".tag-remove");
  if (tagRem) tagRem.parentElement.remove();
});

async function ensureWasm() {
  if (!wasmReady) {
    await init();
    wasmReady = true;
  }
}

async function loadFile(file) {
  if (!file) {
    showStatus("error", "No file selected");
    return;
  }
  if (!file.name.endsWith(".hzo")) {
    showStatus("error", "Please select a .hzo file");
    return;
  }

  showStatus("loading", `Loading ${file.name}...`);

  try {
    await ensureWasm();
    const buf = await file.arrayBuffer();
    reader = new HonzoWasm(new Uint8Array(buf), 1);

    fileName.set(file.name);
    fileSize.set(buf.byteLength);

    const extra = reader.get_extra();
    const chunks = buildChunksData(reader);
    const meta = reader.get_meta_parsed();

    fileInfoData.set({
      versionMajor: reader.version_major(),
      versionMinor: reader.version_minor(),
      minVer: reader.min_reader_version(),
      flags: reader.flags(),
      chunkCount: reader.chunk_count(),
      tocSize: reader.toc_size(),
      dataSize: reader.data_size(),
      extraSize: reader.extra_size(),
      metaSize: reader.meta_size(),
    });

    tocData.set(reader.get_toc());
    metaData.set(meta);
    originalMeta.set(JSON.parse(JSON.stringify(meta)));
    extraData.set(extra);
    chunksData.set(chunks);

    renderFileInfo();
    renderToc();
    renderMeta();
    renderExtra();

    fileLoaded.set(true);
    showStatus(
      "success",
      `Successfully loaded: ${file.name} (${formatSize(buf.byteLength)})`,
    );
  } catch (e) {
    console.error("Error loading file:", e);
    showStatus("error", `Failed to load file: ${e.message || String(e)}`);
  }
}

function buildChunksData(r) {
  const toc = r.get_toc();
  return toc.map((e, i) => ({
    tag: e.chunk_type,
    data: Array.from(r.get_chunk(i) || new Uint8Array(0)),
    compression: e.compression,
    content_type_kind: e.content_type_kind,
    content_type_value: e.content_type_value,
    cover_type: e.cover_type,
    alt_text: e.alt_text || null,
    font_embedding: e.font_embedding,
    font_license_url: e.font_license_url || null,
  }));
}

function renderFileInfo() {
  const d = fileInfoData.get();
  if (!d || !reader) return;

  const layout = reader.layout_mode_name();
  const comp = reader.compression_name();
  const badge = (on, label) =>
    `<span class="flag-badge ${on ? "on" : "off"}">${on ? "Yes" : "No"}</span>`;

  fileInfo.innerHTML = `
    <div class="info-grid">
      <div class="info-item">
        <span class="label">File Size</span>
        <div class="value">${formatSize(fileSize.get())}</div>
      </div>
      <div class="info-item">
        <span class="label">Format Version</span>
        <div class="value">${d.versionMajor}.${d.versionMinor}</div>
      </div>
      <div class="info-item">
        <span class="label">Min Reader Version</span>
        <div class="value">${d.minVer}</div>
      </div>
      <div class="info-item">
        <span class="label">Chunks</span>
        <div class="value">${d.chunkCount}</div>
      </div>
      <div class="info-item">
        <span class="label">Layout Mode</span>
        <div class="value">${layout}</div>
      </div>
      <div class="info-item">
        <span class="label">Default Compression</span>
        <div class="value">${comp}</div>
      </div>
      <div class="info-item">
        <span class="label">TOC Size</span>
        <div class="value">${formatSize(d.tocSize)}</div>
      </div>
      <div class="info-item">
        <span class="label">Data Size</span>
        <div class="value">${formatSize(d.dataSize)}</div>
      </div>
      <div class="info-item">
        <span class="label">Extra Data Size</span>
        <div class="value">${formatSize(d.extraSize)}</div>
      </div>
      <div class="info-item">
        <span class="label">Metadata Size</span>
        <div class="value">${formatSize(d.metaSize)}</div>
      </div>
      <div class="info-item" style="grid-column: 1 / -1">
        <span class="label">Features</span>
        <div class="value features-grid">
          <div>
            <span>Search Index:</span>
            ${badge(reader.has_sidx(), "Search Index")}
          </div>
          <div>
            <span>DRM:</span>
            ${badge(reader.has_drm(), "DRM")}
          </div>
          <div>
            <span>Annotations:</span>
            ${badge(reader.has_annotations(), "Annotations")}
          </div>
          <div>
            <span>Sync:</span>
            ${badge(reader.has_sync(), "Sync")}
          </div>
        </div>
      </div>
    </div>
  `;
}

function renderToc() {
  const toc = tocData.get();
  const cc = fileInfoData.get()?.chunkCount || 0;
  chunkCount.textContent = `(${cc} total)`;

  tocBody.innerHTML = toc
    .map((e, i) => {
      const tag =
        typeof e.chunk_type === "string"
          ? e.chunk_type
          : new TextDecoder().decode(new Uint8Array(e.chunk_type));
      const comp = reader.compression_name_for_chunk(i);
      const type = reader.content_type_name_for_chunk(i);

      return `<tr>
      <td>${i}</td>
      <td><strong>${esc(tag)}</strong></td>
      <td>${formatSize(Number(e.size_compressed))}</td>
      <td>${formatSize(Number(e.size_raw))}</td>
      <td>${comp}</td>
      <td>${type}</td>
      <td>0x${e.flags.toString(16).padStart(4, "0")}</td>
    </tr>`;
    })
    .join("");
}

function renderMeta() {
  const meta = metaData.get();
  if (!meta || typeof meta !== "object") {
    metaFields.innerHTML = "<p style='color:#888'>No metadata</p>";
    return;
  }
  let html = "";
  html += field("Title", "title", meta.title, true, "text");
  html += field("Subtitle", "subtitle", meta.subtitle, true, "text");
  html += field("Authors", "authors", meta.authors, false, "csv");
  html += field("Language", "language", meta.language, false, "text");
  html += field("Publisher", "publisher", meta.publisher, true, "text");
  html += field(
    "Description",
    "description",
    meta.description,
    true,
    "textarea",
  );
  html += field("Source URL", "source_url", meta.source_url, true, "text");
  html += field("License", "license", meta.license, true, "text");
  html += field("Edition", "edition", meta.edition, true, "text");
  html += field("Word Count", "word_count", meta.word_count, true, "number");
  html += field(
    "Reading Time (min)",
    "reading_time_mins",
    meta.reading_time_mins,
    true,
    "number",
  );
  html += tagField("Genres", "genres", meta.genres);
  html += tagField("Tags", "tags", meta.tags);

  html += `<h3>Identifiers</h3><div id="idList">`;
  const ids = meta.identifiers || [];
  html += ids
    .map(
      (id, i) => `
    <div class="field" style="display:flex;gap:0.5rem;align-items:end">
      <div style="flex:1"><label>Type</label><input type="text" class="id-type" value="${esc(id.id_type || "")}" /></div>
      <div style="flex:2"><label>Value</label><input type="text" class="id-value" value="${esc(id.value || "")}" /></div>
      <button class="btn btn-secondary" data-remove="inspect-id" style="padding:0.4rem 0.6rem;font-size:0.8rem">×</button>
    </div>`,
    )
    .join("");
  html += `</div><button class="btn btn-secondary" data-add-id="1" style="font-size:0.85rem;margin-top:0.3rem">+ Add Identifier</button>`;

  html += `<h3>Series</h3>`;
  if (meta.series) {
    html += field(
      "Series Title",
      "series_title",
      meta.series.title,
      true,
      "text",
    );
    html += field(
      "Series Position",
      "series_pos",
      meta.series.position,
      true,
      "text",
    );
    html += field("Series Arc", "series_arc", meta.series.arc, true, "text");
  } else {
    html += `<p style="color:#888;font-size:0.9rem">No series info</p>`;
  }
  metaFields.innerHTML = html;

  metaFields.innerHTML = html;
}

function field(label, id, value, optional, type) {
  const v = value !== null && value !== undefined ? value : "";
  const displayVal =
    typeof v === "object" && v !== null ? Object.values(v)[0] || "" : String(v);
  const input =
    type === "textarea"
      ? `<textarea id="mf_${id}">${esc(displayVal)}</textarea>`
      : type === "csv"
        ? `<input type="text" id="mf_${id}" value="${esc(Array.isArray(v) ? v.join(", ") : displayVal)}" />`
        : `<input type="${type}" id="mf_${id}" value="${esc(displayVal)}" />`;
  return `<div class="field"><label for="mf_${id}">${label}${optional ? "" : " *"}</label>${input}</div>`;
}

function tagField(label, id, values) {
  const items = Array.isArray(values) ? values : [];
  return `<div class="field"><label>${label}</label>
    <div class="tag-list" id="tags_${id}">${items.map((t) => `<span class="tag"><span class="tag-text">${esc(t)}</span> <span class="tag-remove" data-tag-id="${id}">×</span></span>`).join("")}</div>
    <div class="tag-input"><input type="text" id="new_${id}" placeholder="Add ${label.toLowerCase()}" />
    <button data-tag-add="${id}">Add</button></div>
  </div>`;
}

function renderExtra() {
  const extra = extraData.get();
  if (!extra || extra.length === 0) {
    extraInfo.innerHTML = "<p style='color:#888'>No extra data</p>";
    return;
  }
  extraInfo.innerHTML = `<div class="info-grid">
    <div><span class="label">Extra Size</span><div class="value">${formatSize(extra.length)}</div></div>
    <div><span class="label">Entries</span><div class="value">? (binary)</div></div>
  </div>
  <details><summary style="cursor:pointer;margin-top:0.5rem;color:#888">View Hex</summary>
    <pre style="font-size:0.75rem;overflow-x:auto;background:#f8f8f8;padding:0.5rem;border-radius:4px;margin-top:0.3rem;max-height:200px">${bytesToHex(extra)}</pre>
  </details>`;
}

function collectMeta() {
  const meta = JSON.parse(JSON.stringify(originalMeta.get()));
  setStr(meta, "title", document.getElementById("mf_title")?.value);
  setStr(meta, "subtitle", document.getElementById("mf_subtitle")?.value);
  const authors = document.getElementById("mf_authors")?.value || "";
  meta.authors = authors
    .split(",")
    .map((s) => s.trim())
    .filter(Boolean);
  meta.language = document.getElementById("mf_language")?.value || "en";
  setStr(meta, "publisher", document.getElementById("mf_publisher")?.value);
  setStr(meta, "description", document.getElementById("mf_description")?.value);
  setStr(meta, "source_url", document.getElementById("mf_source_url")?.value);
  setStr(meta, "license", document.getElementById("mf_license")?.value);
  setStr(meta, "edition", document.getElementById("mf_edition")?.value);
  setNum(meta, "word_count", document.getElementById("mf_word_count")?.value);
  setNum(
    meta,
    "reading_time_mins",
    document.getElementById("mf_reading_time_mins")?.value,
  );
  meta.genres = collectTags("genres");
  meta.tags = collectTags("tags");

  const idEls = document.querySelectorAll("#idList > div");
  const ids = [];
  for (const el of idEls) {
    const type = el.querySelector(".id-type")?.value?.trim();
    const value = el.querySelector(".id-value")?.value?.trim();
    if (type && value) ids.push({ id_type: type, value });
  }
  meta.identifiers = ids.length > 0 ? ids : undefined;

  const st = document.getElementById("mf_series_title")?.value?.trim();
  if (st) {
    meta.series = {
      title: st,
      position:
        document.getElementById("mf_series_pos")?.value?.trim() || undefined,
      arc: document.getElementById("mf_series_arc")?.value?.trim() || undefined,
    };
  } else {
    meta.series = undefined;
  }
  return meta;
}

function setStr(obj, field, val) {
  if (!val || !val.trim()) {
    delete obj[field];
    return;
  }
  const v = val.trim();
  if (
    obj[field] &&
    typeof obj[field] === "object" &&
    !Array.isArray(obj[field])
  ) {
    obj[field] = { ...obj[field] };
    const keys = Object.keys(obj[field]);
    if (keys.length > 0) obj[field][keys[0]] = v;
    else obj[field] = { en: v };
  } else {
    obj[field] = v;
  }
}

function setNum(obj, field, val) {
  const n = parseInt(val, 10);
  if (isNaN(n)) {
    delete obj[field];
    return;
  }
  obj[field] = n;
}

function collectTags(id) {
  const container = document.getElementById("tags_" + id);
  if (!container) return undefined;
  const items = [];
  for (const span of container.querySelectorAll(".tag")) {
    const text =
      span.querySelector(".tag-text")?.textContent?.trim() ||
      span.textContent.replace("×", "").trim();
    if (text) items.push(text);
  }
  return items.length > 0 ? items : undefined;
}

function onSave() {
  if (!reader) {
    showStatus("error", "No file loaded to save");
    return;
  }

  showStatus("loading", "Saving changes...");

  try {
    const meta = collectMeta();
    const chunks = chunksData.get().map((c) => ({
      tag: c.tag,
      data: new Uint8Array(c.data),
      compression: c.compression,
      content_type_kind: c.content_type_kind,
      content_type_value: c.content_type_value,
      alt_text: c.alt_text,
      font_embedding: c.font_embedding,
      font_license_url: c.font_license_url,
    }));

    const extra = extraData.get();
    const result = honzo_build({
      chunks,
      meta,
      extra: extra?.length ? new Uint8Array(extra) : undefined,
      language: meta.language || "en",
      auto_sidx: true,
    });

    const outputFilename = fileName.get().replace(/\.hzo$/i, "_edited.hzo");
    download(result, outputFilename);
    showStatus("success", `File saved successfully as ${outputFilename}`);
  } catch (e) {
    console.error("Error saving file:", e);
    showStatus("error", `Failed to save file: ${e.message || String(e)}`);
  }
}

function onRevert() {
  const orig = originalMeta.get();
  if (orig) {
    metaData.set(JSON.parse(JSON.stringify(orig)));
    renderMeta();
    showStatus("success", "Metadata reverted to original");
  } else {
    showStatus("error", "No original metadata to revert to");
  }
}

function showStatus(kind, msg) {
  statusVisible.set(true);
  statusKind.set(kind);
  statusMessage.set(msg);

  if (kind === "success" || kind === "loading") {
    setTimeout(() => {
      if (statusKind.get() === kind) {
        statusVisible.set(false);
      }
    }, 5000);
  }
}

function bytesToHex(bytes) {
  const b = new Uint8Array(bytes || []);
  let s = "";
  for (let i = 0; i < Math.min(b.length, 512); i++) {
    if (i > 0 && i % 32 === 0) s += "\n";
    s += b[i].toString(16).padStart(2, "0") + " ";
  }
  if (b.length > 512) s += `\n... (${b.length - 512} more bytes)`;
  return s;
}
