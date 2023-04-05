import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import path from 'path';

export default defineConfig({
	plugins: [sveltekit()],
  resolve: {
    alias: {
      '$components': path.resolve('./src/components'),
      '$routes': path.resolve('./src/routes'),
      '$utils': path.resolve('./src/utils'),
      '$data': path.resolve('./src/data')
    }
  }
});
