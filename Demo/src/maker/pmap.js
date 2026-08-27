import {
  pmapEntries,
  setPmapEntries,
  pmapEmpty,
  pmapTableWrap,
  pmapCount,
  pmapBody,
} from "./state.js";
import { markDirty } from "./saves.js";
import { esc } from "../shared/esc.js";
import { icon } from "../icons.js";

export function addPmapEntry() {
  setPmapEntries([
    ...pmapEntries,
    { printPage: pmapEntries.length + 1, chunkId: 0 },
  ]);
  renderPmap();
}

export function removePmapEntry(idx) {
  const arr = [...pmapEntries];
  arr.splice(idx, 1);
  setPmapEntries(arr);
  renderPmap();
}

export function renderPmap() {
  const hasEntries = pmapEntries.length > 0;
  pmapEmpty.style.display = hasEntries ? "none" : "block";
  pmapTableWrap.style.display = hasEntries ? "" : "none";
  pmapCount.textContent = `${pmapEntries.length} entry${pmapEntries.length !== 1 ? "s" : ""}`;
  pmapBody.innerHTML = pmapEntries
    .map(
      (e, i) => `<tr>
      <td><input type="number" class="pmap-input pmap-print" value="${esc(e.printPage)}" min="1" data-idx="${esc(i)}" /></td>
      <td><input type="number" class="pmap-input pmap-chunk" value="${esc(e.chunkId)}" min="0" data-idx="${esc(i)}" /></td>
      <td><button class="pmap-delete-btn icon-btn" data-action="delete-pmap" data-idx="${esc(i)}" title="Remove">${icon("X", 14)}</button></td>
    </tr>`,
    )
    .join("");
  markDirty();
}
