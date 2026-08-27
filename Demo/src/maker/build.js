import init, { honzo_build } from "../wasm/honzo_wasm.js";
import {
  wasmReady,
  setWasmReady,
  showStatus,
  buildInfoText,
  formatSize,
  layoutMode,
  CHUNK_TYPES,
  chunks,
  pmapBody,
} from "./state.js";
import { collectMeta } from "./meta.js";
import { makerLog } from "../satori.js";
import { download } from "../shared/download.js";

export async function ensureWasm() {
  if (!wasmReady) {
    await init();
    setWasmReady(true);
  }
}

function collectChunks() {
  const cards = document
    .getElementById("chunksList")
    .querySelectorAll(".chunk-card");
  const result = [];
  for (const card of cards) {
    const id = parseInt(card.dataset.id, 10);
    const ch = chunks.find((c) => c.id === id);
    if (!ch) continue;

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
    const compression = compSelect
      ? parseInt(compSelect.value, 10)
      : ch.compression || 0;
    const contentType = typeSelect?.value || ch.contentType || "markdown";

    // eslint-disable-next-line no-useless-assignment
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
      fontEmbedding = embedSelect
        ? parseInt(embedSelect.value, 10)
        : (ch.fontEmbedding ?? 0);
      fontLicenseUrl = licenseInput?.value?.trim() || ch.fontLicenseUrl || null;
    } else if (tag === "MATH") {
      let content = contentArea?.value || ch.content || "";
      data = new TextEncoder().encode(content);
      content_type_kind = 2;
      content_type_value = mathSelect
        ? parseInt(mathSelect.value, 10)
        : ch.mathType || 0;
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
  }));
}

function resolvePmapByteOffsets(pmap, ch) {
  const chunkBreakOffsets = {};
  for (let ci = 0; ci < ch.length; ci++) {
    const c = ch[ci];
    if (c.tag !== "CHAP" && c.tag !== "NOTE") continue;
    const text = new TextDecoder().decode(c.data);
    const offsets = [];
    const re = /<!--\s*pagebreak(?:\s+(\d+))?\s*-->/g;
    let match;
    while ((match = re.exec(text)) !== null) {
      offsets.push({ byteOffset: match.index });
    }
    chunkBreakOffsets[ci] = offsets;
  }

  const usedMarkers = {};
  return pmap.map((entry) => {
    const ci = entry.chunkId;
    if (ci >= ch.length) return { ...entry, byteOffset: 0 };

    const marks = chunkBreakOffsets[ci] || [];
    const key = `${ci}`;
    if (!usedMarkers[key]) usedMarkers[key] = 0;

    const idx = usedMarkers[key];
    usedMarkers[key]++;

    if (idx < marks.length) {
      return { ...entry, byteOffset: marks[idx].byteOffset };
    }
    return { ...entry, byteOffset: 0 };
  });
}

export async function buildBook() {
  try {
    await ensureWasm();
    const ch = collectChunks();
    const pmap = resolvePmapByteOffsets(collectPmap(), ch);
    const { meta, minVer } = collectMeta();
    if (ch.length === 0) {
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
      chunks: ch,
      meta,
      language: meta.language || "en",
      layout: layoutMode,
      flags,
      min_reader_version: minVer,
      auto_sidx: autoSidx,
      auto_covt: autoCovt,
      pmap_entries: pmap.length > 0 ? pmap : undefined,
    };

    if (document.getElementById("flagDrm")?.checked) {
      const pkInput = document.getElementById("drmPublicKey")?.value?.trim();
      const licenseUrl =
        document.getElementById("drmLicenseUrl")?.value?.trim() || null;
      const expiresVal = document.getElementById("drmExpires")?.value;
      const expiresAt = expiresVal
        ? Math.floor(new Date(expiresVal + "T23:59:59Z").getTime() / 1000)
        : null;
      if (pkInput) {
        try {
          const raw = Uint8Array.from(atob(pkInput), (c) => c.charCodeAt(0));
          if (raw.length === 32) {
            spec.drm = {
              encrypt_chunk_ids: ch.map((_, i) => i),
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

    for (const key of Object.keys(spec)) {
      const val = spec[key];
      const typeOf = typeof val;
      const detail =
        val === null
          ? "null"
          : val === undefined
            ? "undefined"
            : Array.isArray(val)
              ? `array[${val.length}]`
              : val instanceof Uint8Array
                ? `Uint8Array[${val.length}]`
                : val instanceof Blob
                  ? "Blob"
                  : typeOf === "object"
                    ? `Object(${Object.keys(val).join(",")})`
                    : typeOf;
      makerLog.debug(`spec.${key} type`, { type: detail });
    }

    const result = honzo_build(spec);
    const slug = (meta.title?.[Object.keys(meta.title)[0]] || "untitled")
      .replace(/[^a-zA-Z0-9_-]/g, "_")
      .toLowerCase();
    download(result, `${slug}.hzo`);

    buildInfoText.set(
      `${ch.length} chunk${ch.length !== 1 ? "s" : ""} \u00b7 ${formatSize(result.length)} \u00b7 ${meta.direction === "rtl" ? "RTL " : ""}${["Reflowable", "Fixed", "Scroll"][layoutMode] || "Reflowable"}${pmap.length > 0 ? ` \u00b7 ${pmap.length} PMAP` : ""}`,
    );
    showStatus("success", `Built: ${slug}.hzo (${formatSize(result.length)})`);
  } catch (e) {
    makerLog.error("Build failed", {
      error: e?.message || String(e),
      type: typeof e,
    });
    if (e?.message?.includes("invalid type")) {
      makerLog.error("Field type mismatch", {
        field: e.message.match(/'([^']+)'/)?.at(1) || "unknown",
        expected: e.message.match(/expected (\w+)/)?.at(1) || "unknown",
      });
    }
    showStatus("error", `Build failed: ${e?.message || String(e)}`);
  }
}
