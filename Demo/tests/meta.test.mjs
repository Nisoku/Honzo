import test from "node:test";
import assert from "node:assert/strict";
import { field, setStr, localizedEntries } from "../src/inspect/meta.js";

test("localizedEntries returns Map entries in insertion order", () => {
  const m = new Map([
    ["en", "Hello"],
    ["fr", "Bonjour"],
  ]);
  assert.deepEqual(localizedEntries(m), [
    ["en", "Hello"],
    ["fr", "Bonjour"],
  ]);
});

test("setStr preserves all locales of a Map instead of collapsing to en", () => {
  const meta = { title: new Map([["en", "Hello"], ["fr", "Bonjour"]]) };
  setStr(meta, "title", "Hi");
  assert.ok(meta.title instanceof Map, "title should remain a Map");
  assert.equal(meta.title.get("en"), "Hi");
  assert.equal(meta.title.get("fr"), "Bonjour");
});

test("setStr keeps the other object locales when updating the primary", () => {
  const meta = { title: { en: "Hello", fr: "Bonjour" } };
  setStr(meta, "title", "Hi");
  assert.equal(meta.title.en, "Hi");
  assert.equal(meta.title.fr, "Bonjour");
});

test("field renders one input per locale for a two-locale Map", () => {
  const html = field(
    "Title",
    "title",
    new Map([
      ["en", "A"],
      ["fr", "B"],
    ]),
    true,
    "text",
  );
  assert.match(html, /id="mf_title"/);
  assert.match(html, /id="mf_title__fr"/);
});
