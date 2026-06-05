import { fontFamily, fontSize, layoutMode, textAlign, theme } from "./state.js";

// Theme engine
const THEMES = {
  light: {
    "--bg": "#ffffff",
    "--bg-secondary": "#f5f5f7",
    "--text": "#1d1d1f",
    "--text-secondary": "#86868b",
    "--text-tertiary": "#aeaeb2",
    "--border": "#d2d2d7",
    "--accent": "#0071e3",
    "--accent-hover": "#0077ed",
    "--accent-soft": "rgba(0, 113, 227, 0.08)",
    "--header-bg": "rgba(255, 255, 255, 0.85)",
    "--sidebar-bg": "#ffffff",
    "--overlay": "rgba(0, 0, 0, 0.3)",
    "--shadow": "0 2px 12px rgba(0, 0, 0, 0.08)",
    "--shadow-lg": "0 8px 30px rgba(0, 0, 0, 0.12)",
    "--progress-fill": "#0071e3",
    "--progress-track": "#e8e8ed",
    "--chapter-title": "#1d1d1f",
    "--chapter-text": "#1d1d1f",
    "--code-bg": "#f5f5f7",
    "--code-border": "#e8e8ed",
    "--reader-width": "42rem",
  },
  sepia: {
    "--bg": "#fbf7f0",
    "--bg-secondary": "#f5efe6",
    "--text": "#3e3026",
    "--text-secondary": "#7a6a5e",
    "--text-tertiary": "#a09284",
    "--border": "#d9cdbc",
    "--accent": "#8b5e3c",
    "--accent-hover": "#7a4f30",
    "--accent-soft": "rgba(139, 94, 60, 0.1)",
    "--header-bg": "rgba(251, 247, 240, 0.85)",
    "--sidebar-bg": "#fbf7f0",
    "--overlay": "rgba(0, 0, 0, 0.35)",
    "--shadow": "0 2px 12px rgba(0, 0, 0, 0.06)",
    "--shadow-lg": "0 8px 30px rgba(0, 0, 0, 0.1)",
    "--progress-fill": "#8b5e3c",
    "--progress-track": "#e8ddd0",
    "--chapter-title": "#3e3026",
    "--chapter-text": "#3e3026",
    "--code-bg": "#f5efe6",
    "--code-border": "#e8ddd0",
    "--reader-width": "42rem",
  },
  dark: {
    "--bg": "#1c1c1e",
    "--bg-secondary": "#2c2c2e",
    "--text": "#f5f5f7",
    "--text-secondary": "#98989d",
    "--text-tertiary": "#636366",
    "--border": "#38383a",
    "--accent": "#0a84ff",
    "--accent-hover": "#409cff",
    "--accent-soft": "rgba(10, 132, 255, 0.15)",
    "--header-bg": "rgba(28, 28, 30, 0.9)",
    "--sidebar-bg": "#1c1c1e",
    "--overlay": "rgba(0, 0, 0, 0.6)",
    "--shadow": "0 2px 12px rgba(0, 0, 0, 0.3)",
    "--shadow-lg": "0 8px 30px rgba(0, 0, 0, 0.4)",
    "--progress-fill": "#0a84ff",
    "--progress-track": "#38383a",
    "--chapter-title": "#f5f5f7",
    "--chapter-text": "#f5f5f7",
    "--code-bg": "#2c2c2e",
    "--code-border": "#38383a",
    "--reader-width": "42rem",
  },
};

const FONTS = {
  sans: "'Inter', -apple-system, BlinkMacSystemFont, sans-serif",
  serif: "'Merriweather', 'Georgia', serif",
  "serif-alt": "'IBM Plex Serif', 'Georgia', serif",
};

const FONT_SIZES = {
  small: { html: "14px", reader: "0.95rem", lineHeight: 1.65 },
  medium: { html: "15px", reader: "1.05rem", lineHeight: 1.75 },
  large: { html: "16px", reader: "1.2rem", lineHeight: 1.8 },
};

export function applyTheme() {
  const t = THEMES[theme.get()] || THEMES.light;
  const root = document.documentElement;
  for (const [key, val] of Object.entries(t)) {
    root.style.setProperty(key, val);
  }

  const fs = FONT_SIZES[fontSize.get()] || FONT_SIZES.medium;
  root.style.fontSize = fs.html;
  root.style.setProperty("--reader-font-size", fs.reader);
  root.style.setProperty("--reader-line-height", String(fs.lineHeight));

  const ff = FONTS[fontFamily.get()] || FONTS.sans;
  root.style.setProperty("--reader-font-family", ff);

  const html = document.documentElement;
  html.setAttribute("data-theme", theme.get());
  html.setAttribute("data-font-size", fontSize.get());
  html.setAttribute("data-font-family", fontFamily.get());
}

// Apply on changes
theme.subscribe(applyTheme);
fontSize.subscribe(applyTheme);
fontFamily.subscribe(applyTheme);

export function renderSettings(container) {
  container.innerHTML = `
    <div class="settings-group">
      <label class="settings-label">Theme</label>
      <div class="settings-options" data-setting="theme">
        <button class="setting-option${theme.get() === "light" ? " active" : ""}" data-value="light">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/><line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/></svg>
          Light
        </button>
        <button class="setting-option${theme.get() === "sepia" ? " active" : ""}" data-value="sepia">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M12 2a10 10 0 0 1 0 20"/></svg>
          Sepia
        </button>
        <button class="setting-option${theme.get() === "dark" ? " active" : ""}" data-value="dark">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg>
          Dark
        </button>
      </div>
    </div>

    <div class="settings-group">
      <label class="settings-label">Font</label>
      <div class="settings-options" data-setting="fontFamily">
        <button class="setting-option${fontFamily.get() === "sans" ? " active" : ""}" data-value="sans">Sans</button>
        <button class="setting-option${fontFamily.get() === "serif" ? " active" : ""}" data-value="serif">Serif</button>
        <button class="setting-option${fontFamily.get() === "serif-alt" ? " active" : ""}" data-value="serif-alt">Serif Alt</button>
      </div>
    </div>

    <div class="settings-group">
      <label class="settings-label">Text Size</label>
      <div class="settings-options" data-setting="fontSize">
        <button class="setting-option${fontSize.get() === "small" ? " active" : ""}" data-value="small">A</button>
        <button class="setting-option${fontSize.get() === "medium" ? " active" : ""}" data-value="medium">A</button>
        <button class="setting-option${fontSize.get() === "large" ? " active" : ""}" data-value="large">A</button>
      </div>
    </div>

    <div class="settings-group">
      <label class="settings-label">Text Alignment</label>
      <div class="settings-options" data-setting="textAlign">
        <button class="setting-option${textAlign.get() === "ltr" ? " active" : ""}" data-value="ltr">LTR</button>
        <button class="setting-option${textAlign.get() === "rtl" ? " active" : ""}" data-value="rtl">RTL</button>
        <button class="setting-option${textAlign.get() === "justify" ? " active" : ""}" data-value="justify">Justify</button>
      </div>
    </div>

    <div class="settings-group">
      <label class="settings-label">Layout</label>
      <div class="settings-options" data-setting="layoutMode">
        <button class="setting-option${layoutMode.get() === "scroll" ? " active" : ""}" data-value="scroll">Scroll</button>
        <button class="setting-option${layoutMode.get() === "paginated" ? " active" : ""}" data-value="paginated" disabled>Paginated</button>
      </div>
    </div>
  `;

  container.querySelectorAll(".settings-options").forEach((group) => {
    group.addEventListener("click", (e) => {
      const btn = e.target.closest(".setting-option");
      if (!btn || btn.disabled) return;
      const setting = group.dataset.setting;
      const value = btn.dataset.value;

      // Update active state
      group
        .querySelectorAll(".setting-option")
        .forEach((b) => b.classList.remove("active"));
      btn.classList.add("active");

      // Apply
      switch (setting) {
        case "theme":
          theme.set(value);
          break;
        case "fontSize":
          fontSize.set(value);
          break;
        case "fontFamily":
          fontFamily.set(value);
          break;
        case "layoutMode":
          layoutMode.set(value);
          break;
        case "textAlign":
          textAlign.set(value);
          break;
      }
    });
  });
}
