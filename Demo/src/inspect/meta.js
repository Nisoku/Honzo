import { esc } from "../shared/esc.js";

// Return [locale, value] pairs for a localized meta value, which may be a `Map`
// (as produced by the wasm reader) or a plain object. Non-localized values yield [].
export function localizedEntries(value) {
  if (value instanceof Map) return [...value.entries()];
  if (value && typeof value === "object" && !Array.isArray(value)) {
    return Object.entries(value);
  }
  return [];
}

function inputMarkup(id, value, type) {
  const display = value !== null && value !== undefined ? String(value) : "";
  if (type === "textarea") {
    return `<textarea id="${id}">${esc(display)}</textarea>`;
  }
  return `<input type="${type}" id="${id}" value="${esc(display)}" />`;
}

// Render an editable field for a metadata value.
export function field(label, id, value, optional, type) {
  const entries = localizedEntries(value);
  if (entries.length > 1) {
    const inputs = entries
      .map(([locale, val], i) => {
        const inputId = i === 0 ? `mf_${id}` : `mf_${id}__${locale}`;
        const suffix = i === 0 ? "" : ` (${esc(locale)})`;
        return `<label>${esc(label)}${suffix}</label>${inputMarkup(inputId, val, type)}`;
      })
      .join("");
    return `<div class="field">${inputs}</div>`;
  }

  const single =
    value instanceof Map
      ? value.values().next().value ?? ""
      : value !== null && value !== undefined
        ? value
        : "";
  const displayVal =
    typeof single === "object" && single !== null
      ? Object.values(single)[0] ?? ""
      : String(single);

  const input =
    type === "csv"
      ? `<input type="text" id="mf_${id}" value="${esc(Array.isArray(single) ? single.join(", ") : displayVal)}" />`
      : inputMarkup(id, displayVal, type);

  const mark = optional ? "" : " *";
  return `<div class="field"><label for="mf_${id}">${esc(label)}${mark}</label>${input}</div>`;
}

// Apply an edited string value to a metadata field, preserving its shape.
export function setStr(obj, fieldName, val) {
  if (!val || !val.trim()) {
    delete obj[fieldName];
    return;
  }
  const v = val.trim();
  if (obj[fieldName] instanceof Map) {
    const updated = new Map(obj[fieldName]);
    const firstKey = updated.keys().next().value;
    if (firstKey !== undefined) updated.set(firstKey, v);
    else updated.set("en", v);
    obj[fieldName] = updated;
    return;
  }
  if (
    obj[fieldName] &&
    typeof obj[fieldName] === "object" &&
    !Array.isArray(obj[fieldName])
  ) {
    obj[fieldName] = { ...obj[fieldName] };
    const keys = Object.keys(obj[fieldName]);
    if (keys.length > 0) obj[fieldName][keys[0]] = v;
    else obj[fieldName] = { en: v };
  } else {
    obj[fieldName] = v;
  }
}
