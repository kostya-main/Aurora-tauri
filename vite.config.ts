import { defineConfig } from "vite";
import preact from '@preact/preset-vite';
import minipic from 'vite-plugin-minipic'

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [preact(), minipic({
    sharpOptions: {
      avif: {
        quality: 75
      },
      jpeg: {
        quality: 75
      },
      jpg: {
        quality: 75
      },
      png: {
        quality: 75
      },
      webp: {
        quality: 100
      },
      gif:{}
    },
    convert: [
      {from: "png", to: "webp"}
    ]
  })],

  root: "frontend",

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
  build: {
    target: 'esnext',
    chunkSizeWarningLimit: 1000,
  }
}));
