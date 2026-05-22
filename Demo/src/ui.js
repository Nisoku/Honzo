import {
  errorText,
  errorVisible,
  libraryOpen,
  loadingVisible,
  searchOpen,
  tocOpen,
} from "./state.js";

export function showLoading() {
  loadingVisible.set(true);
}

export function hideLoading() {
  loadingVisible.set(false);
}

export function showError(message) {
  errorText.set(message);
  errorVisible.set(true);
}

export function hideError() {
  errorVisible.set(false);
}

export function toggleLibrary(forceOpen) {
  if (forceOpen === true) {
    libraryOpen.set(true);
  } else if (forceOpen === false) {
    libraryOpen.set(false);
  } else {
    libraryOpen.set(!libraryOpen.get());
  }
}

export function toggleToc(forceOpen) {
  if (forceOpen === true) {
    tocOpen.set(true);
  } else if (forceOpen === false) {
    tocOpen.set(false);
  } else {
    tocOpen.set(!tocOpen.get());
  }
}

export function toggleSearch(forceOpen) {
  if (forceOpen === true) {
    searchOpen.set(true);
  } else if (forceOpen === false) {
    searchOpen.set(false);
  } else {
    searchOpen.set(!searchOpen.get());
  }
}
