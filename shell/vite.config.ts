import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],

	// Serve .wasm files as static assets with the correct MIME type.
	// Demo crates are built to shell/static/demos/<id>/<id>.wasm
	assetsInclude: ['**/*.wasm'],

	server: {
		// Proxy /api/* to the Axum backend so the shell doesn't need absolute URLs.
		// Usage in the shell: fetch('/api/demos/pong-76') → http://localhost:3000/api/demos/pong-76
		proxy: {
			'/api': {
				target: 'http://localhost:3000',
				changeOrigin: true,
			},
		},

		// WASM files must be served with application/wasm MIME type.
		// Browsers will refuse to compile WASM served as text/plain or octet-stream.
		headers: {
			'Cross-Origin-Opener-Policy': 'same-origin',
			'Cross-Origin-Embedder-Policy': 'require-corp',
		},
	},

	// Prevent Vite from mangling WASM binary imports in the build.
	optimizeDeps: {
		exclude: ['ludeme-demos'],
	},
});
