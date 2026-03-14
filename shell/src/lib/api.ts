// Shared API configuration for data loading.
//
// - Server-side (SSR loaders): uses full URL (http://localhost:3000/api/...)
// - Client-side (browser):     uses relative path (/api/...) so Vite's proxy handles it
//
// For production, set PUBLIC_API_BASE in .env to point to your API host.

import { browser } from '$app/environment';

const DEFAULT_API_BASE = 'http://localhost:3000';

/**
 * Returns the API base URL (server-side only).
 * On the client, returns '' so paths are relative and go through the Vite proxy.
 */
function apiBase(): string {
	if (browser) return '';
	return (
		(typeof import.meta !== 'undefined' && import.meta.env?.PUBLIC_API_BASE) ||
		DEFAULT_API_BASE
	);
}

/**
 * Build an API URL from a path.
 *
 * @example api('/demos')
 *   // Server-side → 'http://localhost:3000/api/demos'
 *   // Client-side → '/api/demos'  (proxied by Vite in dev)
 */
export function api(path: string): string {
	const base = apiBase();
	const normalized = path.startsWith('/') ? path : `/${path}`;
	return `${base}/api${normalized}`;
}
