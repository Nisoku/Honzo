import { resolve } from "path";
import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import { VitePWA } from "vite-plugin-pwa";

export default defineConfig({
  root: "./",
  base: "/Honzo/demo/",
  plugins: [
    {
      name: "wasm-mime",
      configureServer(server) {
        server.middlewares.use((req, res, next) => {
          const pathname = req.url?.split("?")[0] ?? "";
          if (pathname.endsWith(".wasm")) {
            res.setHeader("Content-Type", "application/wasm");
          }
          if (pathname.endsWith(".hzo")) {
            res.setHeader("Content-Type", "application/x-hzo");
          }
          next();
        });
      },
      configurePreviewServer(server) {
        server.middlewares.use((req, res, next) => {
          const pathname = req.url?.split("?")[0] ?? "";
          if (pathname.endsWith(".wasm")) {
            res.setHeader("Content-Type", "application/wasm");
          }
          if (pathname.endsWith(".hzo")) {
            res.setHeader("Content-Type", "application/x-hzo");
          }
          next();
        });
      },
    },
    wasm(),
    topLevelAwait(),
    VitePWA({
      registerType: "autoUpdate",
      manifest: {
        name: "HonzoReader",
        short_name: "HonzoReader",
        start_url: "/Honzo/demo/",
        display: "standalone",
        background_color: "#f5f5f5",
        theme_color: "#1a1a2e",
      },
    }),
  ],
  server: {
    open: true,
    allowedHosts: true,
  },
  build: {
    sourcemap: true,
    outDir: "./dist",
    emptyOutDir: true,
    target: "esnext",
    rollupOptions: {
      input: {
        main: resolve(__dirname, "index.html"),
        convert: resolve(__dirname, "convert.html"),
      },
    },
  },
});
