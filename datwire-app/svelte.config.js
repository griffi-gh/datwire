import adapterAuto from '@sveltejs/adapter-auto';
import adapterStatic from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/kit/vite';
import path from 'path';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	kit: {
		adapter: adapterAuto(),
	},
  alias: {
    '$components': path.resolve('./src/components'),
    '$routes': path.resolve('./src/routes'),
    '$utils': path.resolve('./src/utils'),
    '$data': path.resolve('./src/data')
  }
};

export default config;
