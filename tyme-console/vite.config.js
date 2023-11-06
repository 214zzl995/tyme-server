import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import CompressionPlugin from 'vite-plugin-compression';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte(), CompressionPlugin()],
  server: {
    host: '0.0.0.0'
  },
})
