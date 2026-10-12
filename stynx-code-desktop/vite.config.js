import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

export default defineConfig({
  plugins: [svelte()],
  build: { chunkSizeWarningLimit: 4500 },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
});
