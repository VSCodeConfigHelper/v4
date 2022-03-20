// @ts-check

import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

// https://vitejs.dev/config/
export default defineConfig({
  server: {
    port: 8000,
    strictPort: true
  },
  plugins: [svelte()]
});
