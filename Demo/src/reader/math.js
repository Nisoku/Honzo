import { render_math } from "../wasm/honzo_wasm.js";

export function renderMath(container, data, contentType) {
  const raw = new TextDecoder().decode(data);
  const block = document.createElement("div");
  block.className = "math-block";

  try {
    const mathml = render_math(data, contentType);
    const el = parseMathML(mathml);
    if (el) {
      block.appendChild(el);
    } else {
      fallback(block, raw, "Parsed MathML was empty");
    }
  } catch (err) {
    fallback(block, raw, err);
  }

  container.appendChild(block);
}

function parseMathML(s) {
  const parser = new DOMParser();
  const doc = parser.parseFromString(
    `<html xmlns="http://www.w3.org/1999/xhtml"><body><div>${s}</div></body></html>`,
    "application/xhtml+xml",
  );
  if (doc.querySelector("parsererror")) return null;
  const div = doc.body.firstChild;
  return div ? document.importNode(div.firstChild, true) : null;
}

function fallback(block, raw, _err) {
  const pre = document.createElement("pre");
  pre.className = "math-latex";
  pre.textContent = raw;
  block.appendChild(pre);
}
