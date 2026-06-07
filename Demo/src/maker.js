import init, { honzo_build } from "./wasm/honzo_wasm.js";
import { bindClass, bindEvent, bindText } from "@nisoku/sairin";
import { derived, path, signal } from "@nisoku/sairin";
import { icon, icons } from "./icons.js";
import { createIcons } from "lucide";
import { makerLog } from "./satori.js";

let wasmReady = false;

// State
const statusVisible = signal(path("maker", "statusVisible"), false);
const statusKind = signal(path("maker", "statusKind"), "");
const statusMessage = signal(path("maker", "statusMessage"), "");
const statusClass = derived(
  path("maker", "statusClass"),
  () => `status${statusVisible.get() ? ` active ${statusKind.get()}` : ""}`,
);
const buildInfoText = signal(path("maker", "buildInfoText"), "");

// Settings
let layoutMode = 0;
let defaultCompression = 0;
let direction = "ltr";

// Chunks
let chunks = [];
let chunkIdCounter = 0;

// PMAP
let pmapEntries = [];

// Saves
const SAVES_KEY = "honzo_maker_saves";
const AUTOSAVE_KEY = "honzo_maker_autosave";
let _autoSaveDirty = false;

function markDirty() {
  _autoSaveDirty = true;
}

// Periodic auto-save (every 2s if dirty)
setInterval(() => {
  if (_autoSaveDirty) {
    _autoSaveDirty = false;
    try {
      localStorage.setItem(AUTOSAVE_KEY, JSON.stringify(serializeState()));
    } catch (e) {
      // localStorage full or other error; auto-save silently fails
    }
  }
}, 2000);

function getSaves() {
  try {
    return JSON.parse(localStorage.getItem(SAVES_KEY) || "{}");
  } catch { return {}; }
}

function putSaves(saves) {
  localStorage.setItem(SAVES_KEY, JSON.stringify(saves));
}

function getVal(id) { return document.getElementById(id)?.value ?? ""; }

function setVal(id, val) {
  const el = document.getElementById(id);
  if (el) el.value = val ?? "";
}

function getChecked(id) { return document.getElementById(id)?.checked ?? false; }

function setChecked(id, val) {
  const el = document.getElementById(id);
  if (el) el.checked = !!val;
}

function setActiveOption(containerId, value) {
  const container = document.getElementById(containerId);
  if (!container) return;
  container.querySelectorAll(".setting-option").forEach((o) => o.classList.remove("active"));
  const opt = container.querySelector(`.setting-option[data-value="${value}"]`);
  if (opt) opt.classList.add("active");
}

function uint8ArrayToBase64(u8) {
  let binary = "";
  for (let i = 0; i < u8.length; i++) binary += String.fromCharCode(u8[i]);
  return btoa(binary);
}

function base64ToUint8Array(base64) {
  const binary = atob(base64);
  const u8 = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) u8[i] = binary.charCodeAt(i);
  return u8;
}

function syncFeaturePanels() {
  const panels = [
    ["flagDrm", "drmPanel"],
    ["flagAnno", "annoPanel"],
    ["flagSync", "syncPanel"],
  ];
  for (const [flagId, panelId] of panels) {
    const cb = document.getElementById(flagId);
    const panel = document.getElementById(panelId);
    if (cb && panel) panel.classList.toggle("active", cb.checked);
  }
}

function serializeState() {
  const idEls = document.querySelectorAll("#idList > .maker-id-row");
  const identifiers = [];
  for (const el of idEls) {
    const type = el.querySelector(".id-type")?.value?.trim();
    const value = el.querySelector(".id-value")?.value?.trim();
    if (type && value) identifiers.push({ type, value });
  }

  const contribEls = document.querySelectorAll("#contributorList > .maker-contrib-row");
  const contributors = [];
  for (const el of contribEls) {
    const name = el.querySelector(".contrib-name")?.value?.trim();
    const role = el.querySelector(".contrib-role")?.value?.trim();
    if (name) contributors.push({ name, role: role || "" });
  }

  return {
    savedAt: Date.now(),
    layoutMode,
    defaultCompression,
    direction,
    chunkIdCounter,
    chunks: chunks.map((ch) => ({
      ...ch,
      fileData: ch.fileData instanceof Uint8Array ? uint8ArrayToBase64(ch.fileData) : null,
    })),
    pmapEntries: pmapEntries.map((e) => ({ ...e })),
    mf_title: getVal("mf_title"),
    mf_authors: getVal("mf_authors"),
    mf_language: getVal("mf_language"),
    mf_publisher: getVal("mf_publisher"),
    mf_description: getVal("mf_description"),
    mf_subtitle: getVal("mf_subtitle"),
    mf_edition: getVal("mf_edition"),
    mf_word_count: getVal("mf_word_count"),
    mf_reading_time: getVal("mf_reading_time"),
    mf_source_url: getVal("mf_source_url"),
    mf_license: getVal("mf_license"),
    mf_original_title: getVal("mf_original_title"),
    mf_original_lang: getVal("mf_original_lang"),
    mf_original_authors: getVal("mf_original_authors"),
    mf_series_title: getVal("mf_series_title"),
    mf_series_pos: getVal("mf_series_pos"),
    mf_series_arc: getVal("mf_series_arc"),
    mf_minVer: getVal("mf_minVer"),
    identifiers,
    contributors,
    genres: collectTags("genres"),
    tags: collectTags("tags"),
    flagDrm: getChecked("flagDrm"),
    flagAnno: getChecked("flagAnno"),
    flagSync: getChecked("flagSync"),
    optSidx: getChecked("optSidx"),
    optCovt: getChecked("optCovt"),
    drmPublicKey: getVal("drmPublicKey"),
    drmLicenseUrl: getVal("drmLicenseUrl"),
    drmExpires: getVal("drmExpires"),
    annoStyle: getVal("annoStyle"),
    annoColor: getVal("annoColor"),
    annoUserAnno: getChecked("annoUserAnno"),
    syncUrl: getVal("syncUrl"),
    syncAuto: getChecked("syncAuto"),
    syncInterval: getVal("syncInterval"),
  };
}

function restoreTags(containerId, values) {
  const container = document.getElementById("tags_" + containerId);
  if (!container) return;
  container.innerHTML = "";
  for (const v of values || []) {
    container.insertAdjacentHTML(
      "beforeend",
      `<span class="tag"><span class="tag-text">${esc(v)}</span> <span class="tag-remove" data-tag-id="${containerId}" data-tag-value="${esc(v)}">×</span></span>`,
    );
  }
}

function deserializeState(state) {
  layoutMode = state.layoutMode ?? 0;
  defaultCompression = state.defaultCompression ?? 0;
  direction = state.direction ?? "ltr";
  chunkIdCounter = state.chunkIdCounter ?? 0;

  chunks = (state.chunks || []).map((ch) => ({
    ...ch,
    fileData: ch.fileData && typeof ch.fileData === "string" ? base64ToUint8Array(ch.fileData) : null,
  }));

  pmapEntries = (state.pmapEntries || []).map((e) => ({ ...e }));

  setVal("mf_title", state.mf_title);
  setVal("mf_authors", state.mf_authors);
  setVal("mf_language", state.mf_language);
  setVal("mf_publisher", state.mf_publisher);
  setVal("mf_description", state.mf_description);
  setVal("mf_subtitle", state.mf_subtitle);
  setVal("mf_edition", state.mf_edition);
  setVal("mf_word_count", state.mf_word_count);
  setVal("mf_reading_time", state.mf_reading_time);
  setVal("mf_source_url", state.mf_source_url);
  setVal("mf_license", state.mf_license);
  setVal("mf_original_title", state.mf_original_title);
  setVal("mf_original_lang", state.mf_original_lang);
  setVal("mf_original_authors", state.mf_original_authors);
  setVal("mf_series_title", state.mf_series_title);
  setVal("mf_series_pos", state.mf_series_pos);
  setVal("mf_series_arc", state.mf_series_arc);
  setVal("mf_minVer", state.mf_minVer);

  document.getElementById("idList").innerHTML = "";
  for (const id of state.identifiers || []) addIdentifierRow(id.type, id.value);

  document.getElementById("contributorList").innerHTML = "";
  for (const c of state.contributors || []) addContributorRow(c.name, c.role);

  restoreTags("genres", state.genres);
  restoreTags("tags", state.tags);

  setChecked("flagDrm", state.flagDrm);
  setChecked("flagAnno", state.flagAnno);
  setChecked("flagSync", state.flagSync);
  setChecked("optSidx", state.optSidx);
  setChecked("optCovt", state.optCovt);

  setVal("drmPublicKey", state.drmPublicKey);
  setVal("drmLicenseUrl", state.drmLicenseUrl);
  setVal("drmExpires", state.drmExpires);
  setVal("annoStyle", state.annoStyle);
  setVal("annoColor", state.annoColor);
  setChecked("annoUserAnno", state.annoUserAnno);
  setVal("syncUrl", state.syncUrl);
  setChecked("syncAuto", state.syncAuto);
  setVal("syncInterval", state.syncInterval);

  setActiveOption("layoutOptions", String(state.layoutMode ?? 0));
  setActiveOption("compressionOptions", String(state.defaultCompression ?? 0));
  setActiveOption("directionOptions", state.direction ?? "ltr");

  syncFeaturePanels();

  renderChunks();
  renderPmap();
  renderSaves();
}

function saveProject(name) {
  const saves = getSaves();
  if (saves[name]) {
    if (!confirm(`"${name}" already exists. Overwrite?`)) return;
  }
  try {
    const state = serializeState();
    saves[name] = state;
    putSaves(saves);
    // Also update auto-save
    localStorage.setItem(AUTOSAVE_KEY, JSON.stringify(state));
    showStatus("success", `Saved "${name}"`);
    renderSaves();
  } catch (e) {
    showStatus("error", `Save failed: ${e?.message || String(e)}`);
  }
}

function restoreProject(name) {
  const saves = getSaves();
  const state = saves[name];
  if (!state) {
    showStatus("error", `Save "${name}" not found`);
    return;
  }
  try {
    deserializeState(state);
    showStatus("success", `Restored "${name}"`);
  } catch (e) {
    showStatus("error", `Restore failed: ${e?.message || String(e)}`);
  }
}

function deleteSave(name) {
  if (!confirm(`Delete "${name}"?`)) return;
  const saves = getSaves();
  delete saves[name];
  putSaves(saves);
  showStatus("success", `Deleted "${name}"`);
  renderSaves();
}

function restoreAutosave() {
  try {
    const raw = localStorage.getItem(AUTOSAVE_KEY);
    if (!raw) { showStatus("error", "No auto-save found"); return; }
    deserializeState(JSON.parse(raw));
    showStatus("success", "Restored auto-saved project");
  } catch (e) {
    showStatus("error", `Auto-save restore failed: ${e?.message || String(e)}`);
  }
}

function deleteAutosave() {
  if (!confirm("Delete auto-save? This cannot be undone.")) return;
  localStorage.removeItem(AUTOSAVE_KEY);
  showStatus("success", "Auto-save deleted");
  renderSaves();
}

function renderSaves() {
  const list = document.getElementById("savesList");
  if (!list) return;

  // Check for auto-save
  let autoHtml = "";
  try {
    const raw = localStorage.getItem(AUTOSAVE_KEY);
    if (raw) {
      const auto = JSON.parse(raw);
      autoHtml = `<div class="maker-save-entry">
        <span class="maker-save-name">
          <i data-lucide="clock" width="14" height="14"></i>
          Auto-save (recent)
        </span>
        <span class="maker-save-date">${auto.savedAt ? new Date(auto.savedAt).toLocaleDateString() : ""}</span>
        <button class="btn btn-secondary maker-save-restore-btn" data-save-action="restore-autosave">Restore</button>
        <button class="btn btn-secondary maker-save-delete-btn" data-save-action="delete-autosave">Delete</button>
      </div>`;
    }
  } catch (e) {}

  const saves = getSaves();
  const names = Object.keys(saves);
  if (names.length === 0 && !autoHtml) {
    list.innerHTML = '<div class="maker-saves-empty">No saved projects yet.</div>';
    return;
  }
  list.innerHTML = (autoHtml ? autoHtml : "") + names
    .sort((a, b) => (saves[b].savedAt || 0) - (saves[a].savedAt || 0))
    .map(
      (name) => `<div class="maker-save-entry">
        <span class="maker-save-name">
          <i data-lucide="file-text" width="14" height="14"></i>
          ${esc(name)}
        </span>
        <span class="maker-save-date">${saves[name].savedAt ? new Date(saves[name].savedAt).toLocaleDateString() : ""}</span>
        <button class="btn btn-secondary maker-save-restore-btn" data-save-action="restore" data-save-name="${esc(name)}">Restore</button>
        <button class="btn btn-secondary maker-save-delete-btn" data-save-action="delete" data-save-name="${esc(name)}">Delete</button>
      </div>`,
    )
    .join("");
  createIcons({ icons });
}

// DOM refs
const statusEl = document.getElementById("status");
const statusTextEl = document.getElementById("statusText");
const buildInfoEl = document.getElementById("buildInfo");
const buildBtn = document.getElementById("buildBtn");
const chunksList = document.getElementById("chunksList");
const pmapBody = document.getElementById("pmapBody");
const pmapEmpty = document.getElementById("pmapEmpty");
const pmapTableWrap = document.getElementById("pmapTableWrap");
const pmapCount = document.getElementById("pmapCount");
const addPmapBtn = document.getElementById("addPmapBtn");
const layoutOptions = document.getElementById("layoutOptions");
const compressionOptions = document.getElementById("compressionOptions");
const directionOptions = document.getElementById("directionOptions");

// Bindings
bindClass(statusEl, statusClass);
bindText(statusTextEl, statusMessage);
bindText(buildInfoEl, buildInfoText);

// Chunk types
const CHUNK_TYPES = {
  CHAP: { label: "Chapter", icon: "📄", markup: true },
  NOTE: { label: "Note", icon: "📝", markup: true },
  IMG_: { label: "Image", icon: "🖼", binary: true },
  CSS_: { label: "CSS", icon: "🎨", text: true },
  FONT: { label: "Font", icon: "🔤", binary: true },
  COVR: { label: "Cover", icon: "🖼", binary: true },
  MATH: { label: "Math", icon: "∑", math: true },
};

// Init
function initMaker() {
  createIcons({ icons });

  // Restore auto-save if available
  let restored = false;
  try {
    const raw = localStorage.getItem(AUTOSAVE_KEY);
    if (raw) {
      const state = JSON.parse(raw);
      deserializeState(state);
      showStatus("success", "Restored auto-saved project");
      restored = true;
    }
  } catch (e) {
    // Corrupted auto-save, start fresh
  }

  if (!restored) {
    chunks.push(createChunk("CHAP", "Chapter 1"));
    renderChunks();
    renderPmap();
  }

  setupFeaturePanels();
  renderSaves();

  // Wire auto-save on form field changes
  const makerBody = document.querySelector(".maker-body");
  if (makerBody) {
    makerBody.addEventListener("input", markDirty);
    makerBody.addEventListener("change", markDirty);
  }
}

function setupFeaturePanels() {
  const flagDrm = document.getElementById("flagDrm");
  const flagAnno = document.getElementById("flagAnno");
  const flagSync = document.getElementById("flagSync");
  const drmPanel = document.getElementById("drmPanel");
  const annoPanel = document.getElementById("annoPanel");
  const syncPanel = document.getElementById("syncPanel");

  const toggle = (cb, panel) => {
    cb.addEventListener("change", () => {
      panel.classList.toggle("active", cb.checked);
    });
  };

  if (flagDrm && drmPanel) toggle(flagDrm, drmPanel);
  if (flagAnno && annoPanel) toggle(flagAnno, annoPanel);
  if (flagSync && syncPanel) toggle(flagSync, syncPanel);
}

function createChunk(type, title) {
  const id = ++chunkIdCounter;
  const base = { id, type, title: title || type, compression: defaultCompression };
  if (CHUNK_TYPES[type]?.markup) {
    base.content = "";
    base.contentType = "markdown";
    base.altText = null;
  } else if (type === "IMG_" || type === "COVR") {
    base.fileData = null;
    base.fileName = "";
    base.fileType = "";
    base.altText = null;
    base.coverType = type === "COVR" ? 0 : undefined;
  } else if (type === "CSS_") {
    base.content = "";
  } else if (type === "FONT") {
    base.fileData = null;
    base.fileName = "";
    base.fileType = "";
    base.fontEmbedding = 0;
    base.fontLicenseUrl = "";
  } else if (type === "MATH") {
    base.content = "";
    base.mathType = 0;
  }
  return base;
}

function addChunk(type) {
  const label = CHUNK_TYPES[type]?.label || type;
  const idx = chunks.length;
  chunks.push(createChunk(type, `${label} ${idx === 0 ? 1 : idx + 1}`));
  renderChunks();
}

function removeChunk(id) {
  if (chunks.length <= 1) {
    showStatus("error", "Must have at least one chunk");
    return;
  }
  chunks = chunks.filter((c) => c.id !== id);
  renderChunks();
}

function moveChunk(fromIdx, toIdx) {
  if (toIdx < 0 || toIdx >= chunks.length) return;
  const [item] = chunks.splice(fromIdx, 1);
  chunks.splice(toIdx, 0, item);
  renderChunks();
}

function updateChunkField(id, field, value) {
  const ch = chunks.find((c) => c.id === id);
  if (ch) ch[field] = value;
}

// Render chunks
function renderChunks() {
  chunksList.innerHTML = chunks
    .map((ch, i) => {
      const t = CHUNK_TYPES[ch.type] || { label: ch.type };
      return `<div class="chunk-card" draggable="true" data-idx="${i}" data-id="${ch.id}">
        <div class="chunk-header">
          <div class="chunk-drag" title="Drag to reorder">
            ${icon("GripVertical", 16)}
          </div>
          <span class="chunk-type-badge" data-type="${ch.type}">${ch.type}</span>
          <span class="chunk-title-display">${esc(ch.title)}</span>
          <div class="chunk-header-right">
            <span class="chunk-comp-badge">${ch.compression ? "LZ4" : "Raw"}</span>
            <button class="chunk-settings-btn icon-btn" data-action="settings" data-id="${ch.id}" title="Chunk settings">
              ${icon("Settings", 14)}
            </button>
            <button class="chunk-delete-btn icon-btn" data-action="delete" data-id="${ch.id}" title="Remove chunk">
              ${icon("Trash2", 14)}
            </button>
          </div>
        </div>
        ${renderChunkBody(ch, i)}
        <div class="chunk-settings-panel" id="settings_${ch.id}" style="display:none">
          <div class="chunk-settings-inner">
            <label>Compression: <select class="chunk-sel-comp" data-id="${ch.id}">
              <option value="0" ${ch.compression === 0 ? "selected" : ""}>None</option>
              <option value="1" ${ch.compression === 1 ? "selected" : ""}>LZ4</option>
            </select></label>
            ${ch.type === "COVR" || ch.type === "IMG_" ? `
            <label>Cover Type: <select class="chunk-sel-cover" data-id="${ch.id}">
              <option value="0" ${ch.coverType === 0 ? "selected" : ""}>Front</option>
              <option value="1" ${ch.coverType === 1 ? "selected" : ""}>Back</option>
              <option value="2" ${ch.coverType === 2 ? "selected" : ""}>Full Spread</option>
            </select></label>` : ""}
            ${ch.type === "IMG_" || ch.type === "COVR" ? `
            <label>Alt Text: <input type="text" class="chunk-input-alt" value="${esc(ch.altText || "")}" placeholder="Image description" data-id="${ch.id}" /></label>` : ""}
            ${ch.type === "FONT" ? `
            <label>Embedding: <select class="chunk-sel-embed" data-id="${ch.id}">
              <option value="0" ${ch.fontEmbedding === 0 ? "selected" : ""}>Allowed</option>
              <option value="1" ${ch.fontEmbedding === 1 ? "selected" : ""}>Print Only</option>
              <option value="2" ${ch.fontEmbedding === 2 ? "selected" : ""}>No Modify</option>
              <option value="3" ${ch.fontEmbedding === 3 ? "selected" : ""}>No Embed</option>
            </select></label>
            <label>License URL: <input type="text" class="chunk-input-license" value="${esc(ch.fontLicenseUrl || "")}" placeholder="https://..." data-id="${ch.id}" /></label>` : ""}
          </div>
        </div>
      </div>`;
    })
    .join("");
  markDirty();
}

function renderChunkBody(ch, i) {
  const t = CHUNK_TYPES[ch.type];
  if (!t) return "";
  if (t.markup) {
    return `<div class="chunk-body">
      <div class="chunk-title-row">
        <input type="text" class="chunk-title-input" value="${esc(ch.title)}" placeholder="Title" data-id="${ch.id}" />
        <select class="chunk-type-select" data-id="${ch.id}">
          <option value="markdown" ${ch.contentType === "markdown" ? "selected" : ""}>Markdown</option>
          <option value="html" ${ch.contentType === "html" ? "selected" : ""}>HTML</option>
        </select>
      </div>
      <textarea class="chunk-content" data-id="${ch.id}" placeholder="Write ${ch.contentType} content here..." rows="8">${esc(ch.content)}</textarea>
    </div>`;
  }
  if (t.binary) {
    const isCover = ch.type === "COVR";
    const isFont = ch.type === "FONT";
    const accept = isFont ? "font/*,.ttf,.otf,.woff,.woff2" : "image/*";
    return `<div class="chunk-body">
      <div class="chunk-title-row">
        <input type="text" class="chunk-title-input" value="${esc(ch.title)}" placeholder="${isCover ? "Cover title" : isFont ? "Font name" : "Image title"}" data-id="${ch.id}" />
      </div>
      <div class="chunk-file-area">
        ${ch.fileData ? `
          <div class="chunk-file-preview">
            <span class="chunk-file-name">${esc(ch.fileName)} (${formatSize(ch.fileData.length)})</span>
            <button class="btn btn-secondary" data-action="remove-file" data-id="${ch.id}" style="font-size:0.8rem;padding:4px 10px;height:auto">Remove</button>
          </div>` : `
          <div class="chunk-file-drop" data-id="${ch.id}">
            <span>Drop ${isCover ? "cover image" : isFont ? "font" : "image"} or click to browse</span>
            <input type="file" class="chunk-file-input" accept="${accept}" data-id="${ch.id}" />
          </div>`}
      </div>
    </div>`;
  }
  if (t.text) {
    return `<div class="chunk-body">
      <div class="chunk-title-row">
        <input type="text" class="chunk-title-input" value="${esc(ch.title)}" placeholder="Stylesheet name" data-id="${ch.id}" />
      </div>
      <textarea class="chunk-content chunk-css-content" data-id="${ch.id}" placeholder="/* CSS */" rows="8" spellcheck="false">${esc(ch.content)}</textarea>
    </div>`;
  }
  if (t.math) {
    return `<div class="chunk-body">
      <div class="chunk-title-row">
        <input type="text" class="chunk-title-input" value="${esc(ch.title)}" placeholder="Math title" data-id="${ch.id}" />
        <select class="chunk-math-select" data-id="${ch.id}">
          <option value="0" ${ch.mathType === 0 ? "selected" : ""}>MathML</option>
          <option value="1" ${ch.mathType === 1 ? "selected" : ""}>LaTeX</option>
        </select>
      </div>
      <textarea class="chunk-content chunk-math-content" data-id="${ch.id}" placeholder="${ch.mathType ? "\\sum_{n=1}^{\\infty} ..." : "<math>...</math>"}" rows="8" spellcheck="false">${esc(ch.content)}</textarea>
    </div>`;
  }
  return `<div class="chunk-body"><div class="chunk-title-row">
    <input type="text" class="chunk-title-input" value="${esc(ch.title)}" placeholder="Name" data-id="${ch.id}" />
  </div></div>`;
}

// PMAP
function addPmapEntry() {
  pmapEntries.push({ printPage: 1, chunkId: 0, byteOffset: 0 });
  renderPmap();
}

function removePmapEntry(idx) {
  pmapEntries.splice(idx, 1);
  renderPmap();
}

function renderPmap() {
  const hasEntries = pmapEntries.length > 0;
  pmapEmpty.style.display = hasEntries ? "none" : "block";
  pmapTableWrap.style.display = hasEntries ? "" : "none";
  pmapCount.textContent = `${pmapEntries.length} entry${pmapEntries.length !== 1 ? "s" : ""}`;
  pmapBody.innerHTML = pmapEntries
    .map((e, i) => `<tr>
      <td><input type="number" class="pmap-input pmap-print" value="${e.printPage}" min="1" data-idx="${i}" /></td>
      <td><input type="number" class="pmap-input pmap-chunk" value="${e.chunkId}" min="0" data-idx="${i}" /></td>
      <td><input type="number" class="pmap-input pmap-offset" value="${e.byteOffset}" min="0" data-idx="${i}" /></td>
      <td><button class="pmap-delete-btn icon-btn" data-action="delete-pmap" data-idx="${i}" title="Remove">${icon("X", 14)}</button></td>
    </tr>`)
    .join("");
  markDirty();
}

// Collect data for build
function collectMeta() {
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
    language,
    direction,
    authors,
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

  // Series
  const seriesTitle = document.getElementById("mf_series_title")?.value?.trim();
  if (seriesTitle) {
    const seriesPos = document.getElementById("mf_series_pos")?.value?.trim() || "";
    const seriesArc = document.getElementById("mf_series_arc")?.value?.trim();
    meta.series = {
      title: seriesTitle,
      position: seriesPos,
      ...(seriesArc && { arc: seriesArc }),
    };
  }

  // Identifiers
  const idEls = document.querySelectorAll("#idList > .maker-id-row");
  const ids = [];
  for (const el of idEls) {
    const type = el.querySelector(".id-type")?.value?.trim();
    const value = el.querySelector(".id-value")?.value?.trim();
    if (type && value) ids.push({ id_type: type, value });
  }
  if (ids.length > 0) meta.identifiers = ids;

  // Contributors
  const contribEls = document.querySelectorAll("#contributorList > .maker-contrib-row");
  const contributors = [];
  for (const el of contribEls) {
    const name = el.querySelector(".contrib-name")?.value?.trim();
    const role = el.querySelector(".contrib-role")?.value?.trim();
    if (name) contributors.push({ name, ...(role && { role }) });
  }
  if (contributors.length > 0) meta.contributors = contributors;

  // Genres & Tags
  const genres = collectTags("genres");
  const tags = collectTags("tags");
  if (genres.length > 0) meta.genres = genres;
  if (tags.length > 0) meta.tags = tags;

  return { meta, minVer };
}

function collectTags(id) {
  const container = document.getElementById("tags_" + id);
  if (!container) return [];
  const items = [];
  for (const span of container.querySelectorAll(".tag")) {
    const text = span.querySelector(".tag-text")?.textContent?.trim() || span.textContent.replace("×", "").trim();
    if (text) items.push(text);
  }
  return items;
}

function collectChunks() {
  const cards = chunksList.querySelectorAll(".chunk-card");
  const result = [];
  for (const card of cards) {
    const id = parseInt(card.dataset.id, 10);
    const ch = chunks.find((c) => c.id === id);
    if (!ch) continue;

    // Read fields from DOM
    const titleInput = card.querySelector(".chunk-title-input");
    const typeSelect = card.querySelector(".chunk-type-select");
    const mathSelect = card.querySelector(".chunk-math-select");
    const contentArea = card.querySelector(".chunk-content");
    const compSelect = card.querySelector(".chunk-sel-comp");
    const coverSelect = card.querySelector(".chunk-sel-cover");
    const altInput = card.querySelector(".chunk-input-alt");
    const embedSelect = card.querySelector(".chunk-sel-embed");
    const licenseInput = card.querySelector(".chunk-input-license");

    const title = titleInput?.value?.trim() || ch.title;
    const compression = compSelect ? parseInt(compSelect.value, 10) : (ch.compression || 0);
    const contentType = typeSelect?.value || (ch.contentType || "markdown");

    let data = null;
    let tag = ch.type;
    let content_type_kind = 1;
    let content_type_value = 0;
    let coverType = ch.coverType !== undefined ? ch.coverType : 0;
    let altText = ch.altText || null;
    let fontEmbedding = ch.fontEmbedding;
    let fontLicenseUrl = ch.fontLicenseUrl || null;

    if (CHUNK_TYPES[tag]?.markup) {
      let content = contentArea?.value || ch.content || "";
      if (contentType === "markdown" && content && !content.startsWith("# ")) {
        content = `# ${title}\n\n${content}`;
      }
      data = new TextEncoder().encode(content);
      content_type_value = contentType === "html" ? 1 : 0;
    } else if (tag === "IMG_" || tag === "COVR") {
      data = ch.fileData;
      if (!data) continue;
      altText = altInput?.value?.trim() || ch.altText || null;
      coverType = coverSelect ? parseInt(coverSelect.value, 10) : coverType;
    } else if (tag === "CSS_") {
      data = new TextEncoder().encode(contentArea?.value || ch.content || "");
    } else if (tag === "FONT") {
      data = ch.fileData;
      if (!data) continue;
      fontEmbedding = embedSelect ? parseInt(embedSelect.value, 10) : (ch.fontEmbedding ?? 0);
      fontLicenseUrl = licenseInput?.value?.trim() || ch.fontLicenseUrl || null;
    } else if (tag === "MATH") {
      let content = contentArea?.value || ch.content || "";
      data = new TextEncoder().encode(content);
      content_type_kind = 2;
      content_type_value = mathSelect ? parseInt(mathSelect.value, 10) : (ch.mathType || 0);
    } else {
      data = new TextEncoder().encode(title);
    }

    result.push({
      tag,
      data,
      compression,
      content_type_kind,
      content_type_value,
      cover_type: coverType,
      alt_text: altText,
      font_embedding: fontEmbedding !== undefined ? fontEmbedding : null,
      font_license_url: fontLicenseUrl,
    });
  }
  return result;
}

function collectPmap() {
  const rows = pmapBody.querySelectorAll("tr");
  return Array.from(rows).map((row) => ({
    printPage: parseInt(row.querySelector(".pmap-print")?.value, 10) || 1,
    chunkId: parseInt(row.querySelector(".pmap-chunk")?.value, 10) || 0,
    byteOffset: parseInt(row.querySelector(".pmap-offset")?.value, 10) || 0,
  }));
}

// Build
async function buildBook() {
  try {
    await ensureWasm();
    const chunks = collectChunks();
    const pmap = collectPmap();
    const { meta, minVer } = collectMeta();
    const hasContent = chunks.length > 0;
    if (!hasContent) {
      showStatus("error", "Add at least one chunk before building");
      return;
    }
    showStatus("loading", "Building Honzo file...");

    const autoSidx = document.getElementById("optSidx")?.checked ?? true;
    const autoCovt = document.getElementById("optCovt")?.checked ?? true;
    let flags = 0;
    if (document.getElementById("flagAnno")?.checked) flags |= 0x40;
    if (document.getElementById("flagSync")?.checked) flags |= 0x80;

    const spec = {
      chunks,
      meta,
      language: meta.language || "en",
      layout: layoutMode,
      flags,
      min_reader_version: minVer,
      auto_sidx: autoSidx,
      auto_covt: autoCovt,
      pmap_entries: pmap.length > 0 ? pmap : undefined,
    };

    // DRM
    if (document.getElementById("flagDrm")?.checked) {
      const pkInput = document.getElementById("drmPublicKey")?.value?.trim();
      const licenseUrl = document.getElementById("drmLicenseUrl")?.value?.trim() || null;
      const expiresVal = document.getElementById("drmExpires")?.value;
      const expiresAt = expiresVal ? Math.floor(new Date(expiresVal + "T23:59:59Z").getTime() / 1000) : null;
      if (pkInput) {
        try {
          const raw = Uint8Array.from(atob(pkInput), (c) => c.charCodeAt(0));
          if (raw.length === 32) {
            spec.drm = {
              encrypt_chunk_ids: chunks.map((_, i) => i),
              public_key_der: raw,
              ...(licenseUrl && { license_url: licenseUrl }),
              ...(expiresAt && { expires_at: expiresAt }),
            };
          } else {
            showStatus("error", "DRM public key must be 32 bytes");
            return;
          }
        } catch {
          showStatus("error", "DRM public key is not valid base64");
          return;
        }
      } else {
        showStatus("error", "DRM requires a public key");
        return;
      }
    }

    // Debug: dump every field type before passing to wasm
    for (const key of Object.keys(spec)) {
      const val = spec[key];
      const typeOf = typeof val;
      const detail = val === null ? "null" :
        val === undefined ? "undefined" :
        Array.isArray(val) ? `array[${val.length}]` :
        val instanceof Uint8Array ? `Uint8Array[${val.length}]` :
        val instanceof Blob ? "Blob" :
        typeOf === "object" ? `Object(${Object.keys(val).join(",")})` :
        typeOf;
      makerLog.debug(`spec.${key} type`, { type: detail });
    }
    const result = honzo_build(spec);
    const title = (meta.title?.[Object.keys(meta.title)[0]] || "untitled")
      .replace(/[^a-zA-Z0-9_-]/g, "_").toLowerCase();
    const filename = `${title}.hzo`;

    download(result, filename);
    const chunkCount = chunks.length;
    const pmapLen = pmap.length;
    buildInfoText.set(
      `${chunkCount} chunk${chunkCount !== 1 ? "s" : ""} · ${formatSize(result.length)} · ${meta.direction === "rtl" ? "RTL " : ""}${["Reflowable", "Fixed", "Scroll"][layoutMode] || "Reflowable"}${pmapLen > 0 ? ` · ${pmapLen} PMAP` : ""}`,
    );
    showStatus("success", `Built: ${filename} (${formatSize(result.length)})`);
  } catch (e) {
    makerLog.error("Build failed", { error: e?.message || String(e), type: typeof e });
    if (e?.message?.includes("invalid type")) {
      makerLog.error("Field type mismatch", { field: e.message.match(/'([^']+)'/)?.at(1) || "unknown", expected: e.message.match(/expected (\w+)/)?.at(1) || "unknown" });
    }
    showStatus("error", `Build failed: ${e?.message || String(e)}`);
  }
}

// WASM
async function ensureWasm() {
  if (!wasmReady) { await init(); wasmReady = true; }
}

// Status
function showStatus(kind, msg) {
  statusVisible.set(true);
  statusKind.set(kind);
  statusMessage.set(msg);
  if (kind === "success" || kind === "loading") {
    setTimeout(() => { if (statusKind.get() === kind) statusVisible.set(false); }, 5000);
  }
}

// Download
function download(bytes, filename) {
  const blob = new Blob([bytes], { type: "application/octet-stream" });
  const a = document.createElement("a");
  a.href = URL.createObjectURL(blob);
  a.download = filename;
  a.click();
  URL.revokeObjectURL(a.href);
}

// ========================
// EVENT WIRING
// ========================

// Add chunk buttons
for (const [type, btnId] of [
  ["CHAP", "addChapBtn"],
  ["IMG_", "addImageBtn"],
  ["CSS_", "addCssBtn"],
  ["FONT", "addFontBtn"],
  ["COVR", "addCoverBtn"],
  ["MATH", "addMathBtn"],
  ["NOTE", "addNoteBtn"],
]) {
  const el = document.getElementById(btnId);
  if (el) bindEvent(el, "click", () => addChunk(type));
}

// Add PMAP
bindEvent(addPmapBtn, "click", addPmapEntry);

// Build
bindEvent(buildBtn, "click", buildBook);

// Save
const saveBtn = document.getElementById("saveBtn");
const saveNameInput = document.getElementById("saveNameInput");
if (saveBtn) {
  bindEvent(saveBtn, "click", () => {
    const name = saveNameInput?.value?.trim();
    if (!name) { showStatus("error", "Enter a save name"); return; }
    saveProject(name);
  });
}
if (saveNameInput) {
  bindEvent(saveNameInput, "keydown", (e) => {
    if (e.key === "Enter") saveBtn?.click();
  });
}

// Save action delegation (restore/delete)
bindEvent(document, "click", (e) => {
  const btn = e.target.closest("[data-save-action]");
  if (!btn) return;
  const action = btn.dataset.saveAction;
  if (action === "restore-autosave") restoreAutosave();
  else if (action === "delete-autosave") deleteAutosave();
  else if (action === "restore" || action === "delete") {
    const name = btn.dataset.saveName;
    if (!name) return;
    if (action === "restore") restoreProject(name);
    else deleteSave(name);
  }
});

// Chunk event delegation
bindEvent(chunksList, "input", (e) => {
  const target = e.target;
  const card = target.closest(".chunk-card");
  if (!card) return;
  const id = parseInt(card.dataset.id, 10);
  const ch = chunks.find((c) => c.id === id);
  if (!ch) return;

  if (target.classList.contains("chunk-title-input")) {
    ch.title = target.value;
  }
  if (target.classList.contains("chunk-content")) {
    ch.content = target.value;
  }
  if (target.classList.contains("chunk-type-select")) {
    const textarea = card?.querySelector(".chunk-content");
    if (textarea) textarea.placeholder = `Write ${target.value} content here...`;
  }
  if (target.classList.contains("chunk-math-select")) {
    const textarea = card?.querySelector(".chunk-content");
    if (textarea) textarea.placeholder = parseInt(target.value, 10) ? "\\sum_{n=1}^{\\infty} ..." : "<math>...</math>";
  }
});

bindEvent(chunksList, "click", (e) => {
  const btn = e.target.closest("[data-action]");
  if (!btn) return;
  const action = btn.dataset.action;
  const id = parseInt(btn.dataset.id, 10);
  if (action === "delete") removeChunk(id);
  if (action === "settings") {
    const panel = document.getElementById(`settings_${id}`);
    if (panel) panel.style.display = panel.style.display === "none" ? "block" : "none";
  }
  if (action === "remove-file") {
    const ch = chunks.find((c) => c.id === id);
    if (ch) { ch.fileData = null; ch.fileName = ""; ch.fileType = ""; }
    renderChunks();
  }
});

// File input for binary chunks
bindEvent(chunksList, "change", (e) => {
  const input = e.target.closest(".chunk-file-input");
  if (!input) return;
  const id = parseInt(input.dataset.id, 10);
  const ch = chunks.find((c) => c.id === id);
  if (!ch || !input.files?.[0]) return;
  const file = input.files[0];
  const reader = new FileReader();
  reader.onload = () => {
    ch.fileData = new Uint8Array(reader.result);
    ch.fileName = file.name;
    ch.fileType = file.type || "";
    renderChunks();
  };
  reader.readAsArrayBuffer(file);
});

// File drop zone for binary chunks
bindEvent(chunksList, "dragover", (e) => {
  const drop = e.target.closest(".chunk-file-drop");
  if (drop) { e.preventDefault(); drop.classList.add("dragover"); }
});
bindEvent(chunksList, "dragleave", (e) => {
  const drop = e.target.closest(".chunk-file-drop");
  if (drop) drop.classList.remove("dragover");
});
bindEvent(chunksList, "drop", (e) => {
  const drop = e.target.closest(".chunk-file-drop");
  if (!drop) return;
  e.preventDefault();
  drop.classList.remove("dragover");
  const id = parseInt(drop.dataset.id, 10);
  const ch = chunks.find((c) => c.id === id);
  if (!ch || !e.dataTransfer?.files?.[0]) return;
  const file = e.dataTransfer.files[0];
  const reader = new FileReader();
  reader.onload = () => {
    ch.fileData = new Uint8Array(reader.result);
    ch.fileName = file.name;
    ch.fileType = file.type || "";
    renderChunks();
  };
  reader.readAsArrayBuffer(file);
});

// Drag reorder
let dragSrcIdx = null;
bindEvent(chunksList, "dragstart", (e) => {
  const card = e.target.closest(".chunk-card");
  if (!card) return;
  dragSrcIdx = parseInt(card.dataset.idx, 10);
  e.dataTransfer.effectAllowed = "move";
  e.dataTransfer.setData("text/plain", String(dragSrcIdx));
  card.classList.add("dragging");
});
bindEvent(chunksList, "dragend", (e) => {
  const card = e.target.closest(".chunk-card");
  if (card) card.classList.remove("dragging");
  document.querySelectorAll(".chunk-card.drag-over").forEach((el) => el.classList.remove("drag-over"));
  dragSrcIdx = null;
});
bindEvent(chunksList, "dragover", (e) => {
  if (e.target.closest(".chunk-file-drop")) return; // let file drops handle
  e.preventDefault();
  e.dataTransfer.dropEffect = "move";
  const target = e.target.closest(".chunk-card");
  if (!target || dragSrcIdx === null) return;
  document.querySelectorAll(".chunk-card.drag-over").forEach((el) => el.classList.remove("drag-over"));
  target.classList.add("drag-over");
});
bindEvent(chunksList, "drop", (e) => {
  if (e.target.closest(".chunk-file-drop")) return;
  e.preventDefault();
  const target = e.target.closest(".chunk-card");
  if (!target || dragSrcIdx === null) return;
  const toIdx = parseInt(target.dataset.idx, 10);
  target.classList.remove("drag-over");
  moveChunk(dragSrcIdx, toIdx);
  dragSrcIdx = null;
});

// Settings selectors
const setupSegmented = (el, cb) => {
  if (!el) return;
  bindEvent(el, "click", (e) => {
    const opt = e.target.closest(".setting-option");
    if (!opt) return;
    el.querySelectorAll(".setting-option").forEach((o) => o.classList.remove("active"));
    opt.classList.add("active");
    cb(parseInt(opt.dataset.value, 10));
  });
};
setupSegmented(layoutOptions, (v) => { layoutMode = v; });
setupSegmented(compressionOptions, (v) => { defaultCompression = v; });

if (directionOptions) {
  bindEvent(directionOptions, "click", (e) => {
    const opt = e.target.closest(".setting-option");
    if (!opt) return;
    directionOptions.querySelectorAll(".setting-option").forEach((o) => o.classList.remove("active"));
    opt.classList.add("active");
    direction = opt.dataset.value;
  });
}

// PMAP events
bindEvent(pmapBody, "input", (e) => {
  const target = e.target;
  if (!target.classList.contains("pmap-input")) return;
  const idx = parseInt(target.dataset.idx, 10);
  const val = parseInt(target.value, 10) || 0;
  if (target.classList.contains("pmap-print")) pmapEntries[idx].printPage = val || 1;
  else if (target.classList.contains("pmap-chunk")) pmapEntries[idx].chunkId = val;
  else if (target.classList.contains("pmap-offset")) pmapEntries[idx].byteOffset = val;
});
bindEvent(pmapBody, "click", (e) => {
  const btn = e.target.closest("[data-action='delete-pmap']");
  if (btn) removePmapEntry(parseInt(btn.dataset.idx, 10));
});

// Identifiers
let idCounter = 0;
function addIdentifierRow(type, value) {
  const container = document.getElementById("idList");
  const row = document.createElement("div");
  row.className = "maker-id-row";
  row.style.cssText = "display:flex;gap:8px;margin-bottom:6px;align-items:end";
  row.innerHTML = `
    <div style="flex:1"><input type="text" class="id-type" value="${esc(type || "")}" placeholder="uuid, isbn, doi..." style="width:100%;padding:6px 10px;border:1px solid var(--border);border-radius:6px;font-size:0.85rem;font-family:inherit" /></div>
    <div style="flex:2"><input type="text" class="id-value" value="${esc(value || "")}" placeholder="Value" style="width:100%;padding:6px 10px;border:1px solid var(--border);border-radius:6px;font-size:0.85rem;font-family:inherit" /></div>
    <button class="btn btn-secondary" data-remove="id" style="padding:4px 10px;font-size:0.8rem;height:auto;flex-shrink:0">×</button>`;
  container.appendChild(row);
}
bindEvent(document.getElementById("addIdBtn"), "click", () => addIdentifierRow());

// Contributors
function addContributorRow(name, role) {
  const container = document.getElementById("contributorList");
  const row = document.createElement("div");
  row.className = "maker-contrib-row";
  row.style.cssText = "display:flex;gap:8px;margin-bottom:6px;align-items:end";
  row.innerHTML = `
    <div style="flex:2"><input type="text" class="contrib-name" value="${esc(name || "")}" placeholder="Name" style="width:100%;padding:6px 10px;border:1px solid var(--border);border-radius:6px;font-size:0.85rem;font-family:inherit" /></div>
    <div style="flex:1"><input type="text" class="contrib-role" value="${esc(role || "")}" placeholder="Role" style="width:100%;padding:6px 10px;border:1px solid var(--border);border-radius:6px;font-size:0.85rem;font-family:inherit" /></div>
    <button class="btn btn-secondary" data-remove="contrib" style="padding:4px 10px;font-size:0.8rem;height:auto;flex-shrink:0">×</button>`;
  container.appendChild(row);
}
bindEvent(document.getElementById("addContribBtn"), "click", () => addContributorRow());

// Tags
function addTag(id) {
  const input = document.getElementById(`new_${id}`);
  const value = input?.value?.trim();
  if (value) {
    const container = document.getElementById(`tags_${id}`);
    if (container) {
      container.insertAdjacentHTML("beforeend",
        `<span class="tag"><span class="tag-text">${esc(value)}</span> <span class="tag-remove" data-tag-id="${id}" data-tag-value="${esc(value)}">×</span></span>`);
    }
    input.value = "";
  }
}

bindEvent(document, "click", (e) => {
  const btn = e.target.closest("[data-tag-add]");
  if (btn) addTag(btn.dataset.tagAdd);
  const rem = e.target.closest(".tag-remove");
  if (rem) rem.parentElement.remove();
});

// Delegate remove buttons for identifier/contributor rows
bindEvent(document, "click", (e) => {
  const btn = e.target.closest("[data-remove]");
  if (btn) btn.closest(".maker-id-row, .maker-contrib-row")?.remove();
});

// Prevent file drop on body from reloading page
bindEvent(document.body, "dragover", (e) => { if (e.dataTransfer?.types.includes("Files")) e.preventDefault(); });
bindEvent(document.body, "drop", (e) => { if (e.dataTransfer?.types.includes("Files")) e.preventDefault(); });

// Utils
function formatSize(bytes) {
  const n = Number(bytes);
  if (n < 1024) return n + " B";
  if (n < 1048576) return (n / 1024).toFixed(1) + " KB";
  if (n < 1073741824) return (n / 1048576).toFixed(1) + " MB";
  return (n / 1073741824).toFixed(1) + " GB";
}
function esc(s) {
  if (!s) return "";
  return String(s).replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;").replace(/'/g, "&#39;");
}

// Start
initMaker();
