import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/kit/vite';
import path from 'path';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	kit: {
		adapter: adapter(),
	},
  alias: {
    '$components': path.resolve('./src/components'),
    '$routes': path.resolve('./src/routes'),
    '$utils': path.resolve('./src/utils'),
    '$data': path.resolve('./src/data')
  }
};

export default config;
