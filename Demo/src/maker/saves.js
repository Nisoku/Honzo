import { esc, showStatus, chunks, setChunks, chunkIdCounter, setChunkIdCounter, pmapEntries, setPmapEntries, layoutMode, setLayoutMode, defaultCompression, setDefaultCompression, direction, setDirection } from "./state.js";
import { renderChunks } from "./chunks.js";
import { renderPmap } from "./pmap.js";
import { collectTags, addIdentifierRow, addContributorRow } from "./meta.js";
import { icon, icons } from "../icons.js";
import { createIcons } from "lucide";

const SAVES_KEY = "honzo_maker_saves";
const AUTOSAVE_KEY = "honzo_maker_autosave";
let _autoSaveDirty = false;

export function markDirty() {
  _autoSaveDirty = true;
}

setInterval(() => {
  if (_autoSaveDirty) {
    _autoSaveDirty = false;
    try {
      localStorage.setItem(AUTOSAVE_KEY, JSON.stringify(serializeState()));
    } catch (e) {}
  }
}, 2000);

function getSaves() {
  try { return JSON.parse(localStorage.getItem(SAVES_KEY) || "{}"); }
  catch { return {}; }
}

function putSaves(saves) {
  localStorage.setItem(SAVES_KEY, JSON.stringify(saves));
}

function getVal(id) { return document.getElementById(id)?.value ?? ""; }
function setVal(id, val) { const el = document.getElementById(id); if (el) el.value = val ?? ""; }
function getChecked(id) { return document.getElementById(id)?.checked ?? false; }
function setChecked(id, val) { const el = document.getElementById(id); if (el) el.checked = !!val; }

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
  for (const [flagId, panelId] of [["flagDrm", "drmPanel"], ["flagAnno", "annoPanel"], ["flagSync", "syncPanel"]]) {
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
    mf_title: getVal("mf_title"), mf_authors: getVal("mf_authors"),
    mf_language: getVal("mf_language"), mf_publisher: getVal("mf_publisher"),
    mf_description: getVal("mf_description"), mf_subtitle: getVal("mf_subtitle"),
    mf_edition: getVal("mf_edition"), mf_word_count: getVal("mf_word_count"),
    mf_reading_time: getVal("mf_reading_time"), mf_source_url: getVal("mf_source_url"),
    mf_license: getVal("mf_license"), mf_original_title: getVal("mf_original_title"),
    mf_original_lang: getVal("mf_original_lang"), mf_original_authors: getVal("mf_original_authors"),
    mf_series_title: getVal("mf_series_title"), mf_series_pos: getVal("mf_series_pos"),
    mf_series_arc: getVal("mf_series_arc"), mf_minVer: getVal("mf_minVer"),
    identifiers, contributors,
    genres: collectTags("genres"), tags: collectTags("tags"),
    flagDrm: getChecked("flagDrm"), flagAnno: getChecked("flagAnno"),
    flagSync: getChecked("flagSync"), optSidx: getChecked("optSidx"),
    optCovt: getChecked("optCovt"),
    drmPublicKey: getVal("drmPublicKey"), drmLicenseUrl: getVal("drmLicenseUrl"),
    drmExpires: getVal("drmExpires"), annoStyle: getVal("annoStyle"),
    annoColor: getVal("annoColor"), annoUserAnno: getChecked("annoUserAnno"),
    syncUrl: getVal("syncUrl"), syncAuto: getChecked("syncAuto"),
    syncInterval: getVal("syncInterval"),
  };
}

function restoreTags(containerId, values) {
  const container = document.getElementById("tags_" + containerId);
  if (!container) return;
  container.innerHTML = "";
  for (const v of values || []) {
    container.insertAdjacentHTML("beforeend",
      `<span class="tag"><span class="tag-text">${esc(v)}</span> <span class="tag-remove" data-tag-id="${containerId}" data-tag-value="${esc(v)}">\u00d7</span></span>`);
  }
}

export function deserializeState(state) {
  setLayoutMode(state.layoutMode ?? 0);
  setDefaultCompression(state.defaultCompression ?? 0);
  setDirection(state.direction ?? "ltr");
  setChunkIdCounter(state.chunkIdCounter ?? 0);

  setChunks((state.chunks || []).map((ch) => ({
    ...ch,
    fileData: ch.fileData && typeof ch.fileData === "string" ? base64ToUint8Array(ch.fileData) : null,
  })));
  setPmapEntries((state.pmapEntries || []).map((e) => ({ ...e })));

  setVal("mf_title", state.mf_title); setVal("mf_authors", state.mf_authors);
  setVal("mf_language", state.mf_language); setVal("mf_publisher", state.mf_publisher);
  setVal("mf_description", state.mf_description); setVal("mf_subtitle", state.mf_subtitle);
  setVal("mf_edition", state.mf_edition); setVal("mf_word_count", state.mf_word_count);
  setVal("mf_reading_time", state.mf_reading_time); setVal("mf_source_url", state.mf_source_url);
  setVal("mf_license", state.mf_license); setVal("mf_original_title", state.mf_original_title);
  setVal("mf_original_lang", state.mf_original_lang); setVal("mf_original_authors", state.mf_original_authors);
  setVal("mf_series_title", state.mf_series_title); setVal("mf_series_pos", state.mf_series_pos);
  setVal("mf_series_arc", state.mf_series_arc); setVal("mf_minVer", state.mf_minVer);

  document.getElementById("idList").innerHTML = "";
  for (const id of state.identifiers || []) addIdentifierRow(id.type, id.value);
  document.getElementById("contributorList").innerHTML = "";
  for (const c of state.contributors || []) addContributorRow(c.name, c.role);
  restoreTags("genres", state.genres);
  restoreTags("tags", state.tags);

  setChecked("flagDrm", state.flagDrm); setChecked("flagAnno", state.flagAnno);
  setChecked("flagSync", state.flagSync); setChecked("optSidx", state.optSidx);
  setChecked("optCovt", state.optCovt);
  setVal("drmPublicKey", state.drmPublicKey); setVal("drmLicenseUrl", state.drmLicenseUrl);
  setVal("drmExpires", state.drmExpires); setVal("annoStyle", state.annoStyle);
  setVal("annoColor", state.annoColor); setChecked("annoUserAnno", state.annoUserAnno);
  setVal("syncUrl", state.syncUrl); setChecked("syncAuto", state.syncAuto);
  setVal("syncInterval", state.syncInterval);

  setActiveOption("layoutOptions", String(state.layoutMode ?? 0));
  setActiveOption("compressionOptions", String(state.defaultCompression ?? 0));
  setActiveOption("directionOptions", state.direction ?? "ltr");
  syncFeaturePanels();

  renderChunks();
  renderPmap();
  renderSaves();
}

export function saveProject(name) {
  const saves = getSaves();
  if (saves[name] && !confirm(`"${name}" already exists. Overwrite?`)) return;
  try {
    const state = serializeState();
    saves[name] = state;
    putSaves(saves);
    localStorage.setItem(AUTOSAVE_KEY, JSON.stringify(state));
    showStatus("success", `Saved "${name}"`);
    renderSaves();
  } catch (e) {
    showStatus("error", `Save failed: ${e?.message || String(e)}`);
  }
}

export function restoreProject(name) {
  const saves = getSaves();
  const state = saves[name];
  if (!state) { showStatus("error", `Save "${name}" not found`); return; }
  try {
    deserializeState(state);
    showStatus("success", `Restored "${name}"`);
  } catch (e) {
    showStatus("error", `Restore failed: ${e?.message || String(e)}`);
  }
}

export function deleteSave(name) {
  if (!confirm(`Delete "${name}"?`)) return;
  const saves = getSaves();
  delete saves[name];
  putSaves(saves);
  showStatus("success", `Deleted "${name}"`);
  renderSaves();
}

export function restoreAutosave() {
  try {
    const raw = localStorage.getItem(AUTOSAVE_KEY);
    if (!raw) { showStatus("error", "No auto-save found"); return; }
    deserializeState(JSON.parse(raw));
    showStatus("success", "Restored auto-saved project");
  } catch (e) {
    showStatus("error", `Auto-save restore failed: ${e?.message || String(e)}`);
  }
}

export function deleteAutosave() {
  if (!confirm("Delete auto-save?")) return;
  localStorage.removeItem(AUTOSAVE_KEY);
  showStatus("success", "Auto-save deleted");
  renderSaves();
}

export function renderSaves() {
  const list = document.getElementById("savesList");
  if (!list) return;

  let autoHtml = "";
  try {
    const raw = localStorage.getItem(AUTOSAVE_KEY);
    if (raw) {
      const auto = JSON.parse(raw);
      autoHtml = `<div class="maker-save-entry">
        <span class="maker-save-name"><i data-lucide="clock" width="14" height="14"></i> Auto-save (recent)</span>
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
    .map((name) => `<div class="maker-save-entry">
      <span class="maker-save-name"><i data-lucide="file-text" width="14" height="14"></i> ${esc(name)}</span>
      <span class="maker-save-date">${saves[name].savedAt ? new Date(saves[name].savedAt).toLocaleDateString() : ""}</span>
      <button class="btn btn-secondary maker-save-restore-btn" data-save-action="restore" data-save-name="${esc(name)}">Restore</button>
      <button class="btn btn-secondary maker-save-delete-btn" data-save-action="delete" data-save-name="${esc(name)}">Delete</button>
    </div>`)
    .join("");
  createIcons({ icons });
}
