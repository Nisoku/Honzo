import { derived, path, signal } from "@nisoku/sairin";
import { bindClass, bindText } from "@nisoku/sairin";
export { esc } from "../shared/esc.js";
export { formatSize } from "../shared/format.js";

export let wasmReady = false;
export function setWasmReady(v) { wasmReady = v; }
export const statusVisible = signal(path("maker", "statusVisible"), false);
export const statusKind = signal(path("maker", "statusKind"), "");
export const statusMessage = signal(path("maker", "statusMessage"), "");
export const statusClass = derived(
  path("maker", "statusClass"),
  () => `status${statusVisible.get() ? ` active ${statusKind.get()}` : ""}`,
);
export const buildInfoText = signal(path("maker", "buildInfoText"), "");

export let layoutMode = 0;
export let defaultCompression = 0;
export let direction = "ltr";
export let chunks = [];
export let chunkIdCounter = 0;
export let pmapEntries = [];
export let dragSrcIdx = null;
export let idCounter = 0;

export function setLayoutMode(v) { layoutMode = v; }
export function setDefaultCompression(v) { defaultCompression = v; }
export function setDirection(v) { direction = v; }
export function setChunks(v) { chunks = v; }
export function setChunkIdCounter(v) { chunkIdCounter = v; }
export function setPmapEntries(v) { pmapEntries = v; }
export function setDragSrcIdx(v) { dragSrcIdx = v; }
export function setIdCounter(v) { idCounter = v; }

export const CHUNK_TYPES = {
  CHAP: { label: "Chapter", icon: "\u{1F4C4}", markup: true },
  NOTE: { label: "Note", icon: "\u{1F4DD}", markup: true },
  IMG_: { label: "Image", icon: "\u{1F5BC}", binary: true },
  CSS_: { label: "CSS", icon: "\u{1F3A8}", text: true },
  FONT: { label: "Font", icon: "\u{1F524}", binary: true },
  COVR: { label: "Cover", icon: "\u{1F5BC}", binary: true },
  MATH: { label: "Math", icon: "\u2211", math: true },
};

export const statusEl = document.getElementById("status");
export const statusTextEl = document.getElementById("statusText");
export const buildInfoEl = document.getElementById("buildInfo");
export const buildBtn = document.getElementById("buildBtn");
export const chunksList = document.getElementById("chunksList");
export const pmapBody = document.getElementById("pmapBody");
export const pmapEmpty = document.getElementById("pmapEmpty");
export const pmapTableWrap = document.getElementById("pmapTableWrap");
export const pmapCount = document.getElementById("pmapCount");
export const addPmapBtn = document.getElementById("addPmapBtn");
export const layoutOptions = document.getElementById("layoutOptions");
export const compressionOptions = document.getElementById("compressionOptions");
export const directionOptions = document.getElementById("directionOptions");

bindClass(statusEl, statusClass);
bindText(statusTextEl, statusMessage);
bindText(buildInfoEl, buildInfoText);

export function showStatus(kind, msg) {
  statusVisible.set(true);
  statusKind.set(kind);
  statusMessage.set(msg);
  if (kind === "success" || kind === "loading") {
    setTimeout(() => { if (statusKind.get() === kind) statusVisible.set(false); }, 5000);
  }
}
