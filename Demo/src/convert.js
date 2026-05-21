import initLexepub, { WasmEpubExtractor } from "lexepub";
import lexepubWasmUrl from "lexepub/lexepub_bg.wasm?url";
import initHonzo, { honzo_build } from "./wasm/honzo_wasm.js";
import { bindClass, bindEvent, bindStyle, bindText } from "@nisoku/sairin";
import { derived, path, signal } from "@nisoku/sairin";

let lexReady = false;
let honzoReady = false;

const dropZone = document.getElementById("dropZone");
const fileInput = document.getElementById("fileInput");
const statusEl = document.getElementById("status");
const statusText = document.getElementById("statusText");
const progressFill = document.getElementById("progressFill");
const historyList = document.getElementById("historyList");

const statusVisible = signal(path("convert", "statusVisible"), false);
const statusKind = signal(path("convert", "statusKind"), "");
const statusMessage = signal(path("convert", "statusMessage"), "");
const progressWidth = signal(path("convert", "progressWidth"), "0%");

const statusClass = derived(path("convert", "statusClass"), () => {
  return `status${statusVisible.get() ? ` active ${statusKind.get()}` : ""}`;
});

bindClass(statusEl, statusClass);
bindText(statusText, statusMessage);
bindStyle(progressFill, "width", progressWidth);

bindEvent(dropZone, "click", () => fileInput.click());
bindEvent(dropZone, "dragover", (event) => {
  event.preventDefault();
  dropZone.classList.add("dragover");
});
bindEvent(dropZone, "dragleave", () => dropZone.classList.remove("dragover"));
bindEvent(dropZone, "drop", (event) => {
  event.preventDefault();
  dropZone.classList.remove("dragover");
  handleFile(event.dataTransfer?.files?.[0]);
});
bindEvent(fileInput, "change", (event) => {
  handleFile(event.target.files?.[0]);
});

async function ensureLexepub() {
  if (!lexReady) {
    await initLexepub(lexepubWasmUrl);
    lexReady = true;
  }
}

async function ensureHonzo() {
  if (!honzoReady) {
    await initHonzo();
    honzoReady = true;
  }
}

async function handleFile(file) {
  if (!file) {
    return;
  }

  showStatus("loading", `Reading ${file.name}...`, 10);

  try {
    const buf = await file.arrayBuffer();
    showStatus("loading", "Parsing EPUB with lexepub...", 30);

    await ensureLexepub();
    const extractor = new WasmEpubExtractor();
    await extractor.load_from_bytes(new Uint8Array(buf));

    showStatus("loading", "Extracting metadata...", 50);
    const meta = await extractor.get_metadata();
    const toc = await extractor.get_toc();

    if (!Array.isArray(toc) || toc.length === 0) {
      throw new Error("No chapters found in this EPUB");
    }

    const firstVal = (v) => v instanceof Map ? v.values().next().value : (typeof v === 'object' ? Object.values(v)[0] : v);
    const title = firstVal(meta?.title) || file.name.replace(/\.epub$/i, "");
    const author = Array.isArray(meta?.authors) ? meta.authors[0] : meta?.creator || "Unknown";
    const lang = Array.isArray(meta?.languages) ? meta.languages[0] : "en";

    const chapters = [];
    for (const entry of toc) {
      const href = entry.chapter_href;
      if (!href) {
        continue;
      }

      try {
        const raw = await extractor.get_resource(href);
        const html = new TextDecoder().decode(raw);
        const cleaned = html
          .replace(/<script[\s\S]*?<\/script>/gi, "")
          .replace(/<style[\s\S]*?<\/style>/gi, "");
        chapters.push(cleaned);
      } catch {
        try {
          const text = await extractor.get_chapter_text(entry.chapter_index ?? chapters.length);
          if (text) {
            chapters.push(`<p>${escapeHtml(text)}</p>`);
          }
        } catch {
          // ignore unreadable chapters
        }
      }
    }

    if (chapters.length === 0) {
      throw new Error("No readable chapters found in this EPUB");
    }

    showStatus("loading", `Extracting images and assets...`, 60);

    const chunks = chapters.map((text) => ({
      tag: "CHAP",
      data: new TextEncoder().encode(text),
      compression: 0,
      content_type_kind: 1,
      content_type_value: 1,
    }));

    // Read container.xml and OPF to find images, CSS, fonts
    try {
      const containerXml = await extractor.get_resource("META-INF/container.xml");
      const containerDoc = new DOMParser().parseFromString(new TextDecoder().decode(containerXml), "text/xml");
      const opfPath = containerDoc.querySelector("rootfile")?.getAttribute("full-path");
      if (opfPath) {
        const opfXml = await extractor.get_resource(opfPath);
        const opf = new DOMParser().parseFromString(new TextDecoder().decode(opfXml), "text/xml");
        const dir = opfPath.includes("/") ? opfPath.slice(0, opfPath.lastIndexOf("/") + 1) : "";
        const resolve = (href) => href.startsWith("/") ? href.slice(1) : dir + href;

        let coverId = null;
        for (const el of opf.querySelectorAll("meta")) {
          if (el.getAttribute("name") === "cover") coverId = el.getAttribute("content");
        }
        for (const el of opf.querySelectorAll("item")) {
          if (el.getAttribute("properties")?.includes("cover-image")) coverId = el.getAttribute("id");
        }

        for (const el of opf.querySelectorAll("item")) {
          const id = el.getAttribute("id");
          const href = el.getAttribute("href");
          const mt = el.getAttribute("media-type") || "";
          if (!id || !href) continue;

          try {
            const path = resolve(href);
            const data = await extractor.get_resource(path);

              if (mt.startsWith("image/")) {
              chunks.push({
                tag: id === coverId ? "COVR" : "IMG_",
                data: new Uint8Array(data),
                compression: 0,
                  content_type_kind: 1,
                  content_type_value: 0,
                alt_text: null,
              });
              } else if (mt === "text/css") {
              chunks.push({
                tag: "CSS_",
                data: new Uint8Array(data),
                compression: 0,
                  content_type_kind: 1,
                  content_type_value: 0,
                alt_text: null,
              });
            } else if (mt.startsWith("font/") || mt.includes("font")) {
              chunks.push({
                tag: "FONT",
                data: new Uint8Array(data),
                compression: 0,
                  content_type_kind: 1,
                  content_type_value: 0,
                alt_text: null,
              });
            }
          } catch {}
        }

        // Generate COVT from COVR
        const covr = chunks.find(c => c.tag === "COVR");
        if (covr) {
          try {
            const { honzo_std } = await import("./wasm/honzo_wasm.js");
            // COVT generation not available in WASM, skip
          } catch {}
        }
      }
    } catch {}

    showStatus("loading", `Building .hzo (${chunks.length} chunks)...`, 70);

    await ensureHonzo();
    const spec = {
      chunks: chunks.map((c) => ({
        tag: c.tag,
        data: c.data,
        compression: c.compression,
        content_type_kind: c.content_type_kind,
        content_type_value: c.content_type_value,
      })),
      meta: {
        title: { [lang]: title },
        authors: [author],
        language: lang,
        source_format: "epub",
      },
    };

    const hzo = await honzo_build(spec);
    showStatus("success", `Converted: ${hzo.length.toLocaleString()} bytes`, 100);
    addToHistory(title, hzo);
  } catch (error) {
    showStatus("error", `Error: ${error?.message || error}`, 0);
  }
}

function showStatus(kind, message, progress) {
  statusVisible.set(true);
  statusKind.set(kind);
  statusMessage.set(message);
  progressWidth.set(`${progress}%`);
}

function addToHistory(name, data) {
  const item = document.createElement("div");
  item.className = "history-item";

  const filename = name.replace(/[^a-zA-Z0-9_-]/g, "_") + ".hzo";
  const blob = new Blob([data], { type: "application/octet-stream" });
  const url = URL.createObjectURL(blob);

  item.innerHTML = `
    <div>
      <div class="name">${filename}</div>
      <div class="size">${data.length.toLocaleString()} bytes</div>
    </div>
    <a class="dl-btn" href="${url}" download="${filename}">Download</a>
  `;

  historyList.prepend(item);
}

function escapeHtml(value) {
  return value
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}
