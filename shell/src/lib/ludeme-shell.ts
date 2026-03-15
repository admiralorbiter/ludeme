/**
 * shell/src/lib/ludeme-shell.ts
 *
 * Sets up window.__ludeme (the shell API contract) and handles WASM loading.
 * This module is the ONLY place that touches window.__ludeme.
 * See docs/04-shell-api-contract.md for the full protocol.
 */

import { session } from './session.svelte.js';
import type { GameEvent, LudemeShellAPI } from './types.js';

// ---------------------------------------------------------------------------
// Internal param registry — the bridge from shell → WASM
// ---------------------------------------------------------------------------

// Current param values the WASM can read on each frame.
// Kept in sync with session.paramValues by setParam().
const _params: Record<string, number> = {};

// ---------------------------------------------------------------------------
// Parse and dispatch a raw event JSON string from the WASM module
// ---------------------------------------------------------------------------

function dispatchEvent(eventJson: string): void {
	let event: GameEvent;
	try {
		event = JSON.parse(eventJson) as GameEvent;
	} catch {
		session.setError({
			code: 'event_parse',
			message: `Failed to parse GameEvent: ${eventJson.slice(0, 120)}`,
		});
		console.error('[ludeme-shell] Failed to parse event JSON:', eventJson);
		return;
	}

	session.handleEvent(event);
}

// ---------------------------------------------------------------------------
// Register window.__ludeme — must be called before WASM init
// ---------------------------------------------------------------------------

export function registerShellAPI(): void {
	const api: LudemeShellAPI = {
		onEvent: dispatchEvent,
		getParam: (key: string) => _params[key] ?? null,
	};

	window.__ludeme = api;

	// Also expose the set-param bridge so the shell can push values into WASM
	window.__ludeme_set_param = (key: string, value: number) => {
		_params[key] = value;
		// If the WASM module exposes a setter, call it
		// (wasm-bindgen generates ludeme_set_param on the module instance)
		if (typeof (window as unknown as Record<string, unknown>).ludeme_set_param === 'function') {
			const fn = (window as unknown as Record<string, (k: string, v: number) => void>).ludeme_set_param;
			fn(key, value);
		}
	};
}

// ---------------------------------------------------------------------------
// Sync param values from the session store → the internal registry
// This keeps _params up to date when sliders or defaults change
// ---------------------------------------------------------------------------

export function syncParams(values: Record<string, number>): void {
	for (const [key, value] of Object.entries(values)) {
		_params[key] = value;
	}
}

// ---------------------------------------------------------------------------
// Load and initialise a WASM demo module
// ---------------------------------------------------------------------------

export async function loadDemo(wasmJsPath: string): Promise<void> {
	session.setStatus('loading');

	try {
		// wasmJsPath points to the wasm-bindgen JS glue (e.g. /demos/pong-76/pong_76.js)
		// The .wasm binary is the sibling _bg.wasm file (wasm-bindgen naming convention)
		const wasmBinaryPath = wasmJsPath.replace(/\.js$/, '_bg.wasm');

		// Dynamically import the JS glue.
		// Add a cache-busting query param so the module is freshly imported on each
		// client-side navigation (otherwise the browser reuses the cached module
		// whose WASM instance is already dead after the previous page teardown).
		const cacheBust = `?t=${Date.now()}`;
		const glueModule = await import(/* @vite-ignore */ `${wasmJsPath}${cacheBust}`);

		// Find the init function — wasm-bindgen names it differently across versions
		const initFn = glueModule.default ?? glueModule.__wbg_init ?? glueModule.init;

		if (typeof initFn !== 'function') {
			throw new Error(
				'WASM glue module does not export an init function. ' +
				`Found exports: [${Object.keys(glueModule).join(', ')}]`
			);
		}

		// Pass the explicit .wasm path as an object (new wasm-bindgen API).
		// This avoids the "using deprecated parameters" warning.
		await initFn({ module_or_path: wasmBinaryPath });

		// The WASM module's #[wasm_bindgen(start)] fn fires during init().
		// It emits SessionStart which drives the session store into 'active'.
	} catch (err) {
		const message = err instanceof Error ? err.message : String(err);
		session.setError({
			code: 'wasm_load_failed',
			message,
		});
		console.error('[ludeme-shell] WASM load failed:', message);
	}
}

import { api } from './api.js';

// ---------------------------------------------------------------------------
// Set a param value (called by the param tuner UI)
// Updates both the internal registry and the session store
// ---------------------------------------------------------------------------

export function setParam(key: string, value: number): void {
	const oldValue = _params[key] ?? null;
	_params[key] = value;
	session.setParam(key, value);

	if (window.__ludeme_set_param) {
		window.__ludeme_set_param(key, value);
	}

	// Log param change to API (fire-and-forget)
	const demo = session.demo;
	if (demo) {
		fetch(api('/param-changes'), {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				demo_id:   demo.id,
				session_id: session.sessionId ?? null,
				frame:     session.frameCount,
				param_key: key,
				old_value: oldValue,
				new_value: value,
			}),
		}).catch(() => { /* ignore network errors */ });
	}
}
