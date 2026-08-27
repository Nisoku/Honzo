export function showLoading() {
  const el = document.getElementById("loading-message");
  if (el) el.classList.add("show");
}

export function hideLoading() {
  const el = document.getElementById("loading-message");
  if (el) el.classList.remove("show");
}

export function showError(message) {
  const el = document.getElementById("error-message");
  const text = document.getElementById("error-text");
  if (text) text.textContent = message;
  if (el) el.classList.add("show");
}

export function hideError() {
  const el = document.getElementById("error-message");
  if (el) el.classList.remove("show");
}

export function showToast(message, duration = 1500) {
  const el = document.getElementById("toast");
  if (!el) return;
  el.textContent = message;
  el.classList.add("show");
  clearTimeout(el._timer);
  el._timer = setTimeout(() => el.classList.remove("show"), duration);
}
