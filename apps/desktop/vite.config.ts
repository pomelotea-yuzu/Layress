import { defineConfig } from "vite";

export default defineConfig({
  // strictPort：devUrl 固定为 5173，端口被占时直接失败，避免 Tauri devUrl 与实际不符
  server: {
    strictPort: true,
    port: 5173,
  },
  build: {
    outDir: "dist",
    emptyOutDir: true,
    sourcemap: true,
  },
});
