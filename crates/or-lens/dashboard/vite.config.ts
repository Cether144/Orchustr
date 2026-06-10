import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// Builds to fixed file names (no content hashes) so or-lens can embed the
// output with include_str! and serve it from stable routes.
export default defineConfig({
  plugins: [react()],
  base: "/assets/",
  build: {
    outDir: "../assets/dist",
    emptyOutDir: true,
    cssCodeSplit: false,
    rollupOptions: {
      output: {
        entryFileNames: "dashboard.js",
        chunkFileNames: "dashboard-[name].js",
        assetFileNames: "dashboard.[ext]",
      },
    },
  },
  server: {
    // `vite dev` proxies API calls to a locally running or-lens server.
    proxy: { "/api": "http://127.0.0.1:7700" },
  },
});
