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
			(window as unknown as Record<string, { (k: string, v: number): void }[]>)
				.ludeme_set_param(key, value);
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

export async function loadDemo(wasmPath: string): Promise<void> {
	session.setStatus('loading');

	try {
		// Dynamically import the wasm-bindgen JS glue module.
		// Each demo's build outputs a <name>.js that exports `default` (init).
		const glueModule = await import(/* @vite-ignore */ wasmPath.replace('.wasm', '.js'));

		if (typeof glueModule.default !== 'function') {
			throw new Error('WASM glue module does not export a default init function.');
		}

		// Register the shell API BEFORE init so the game can call it immediately.
		registerShellAPI();

		await glueModule.default(wasmPath);

		session.setStatus('ready');
	} catch (err) {
		const message = err instanceof Error ? err.message : String(err);
		session.setError({
			code: 'wasm_load_failed',
			message,
		});
		console.error('[ludeme-shell] WASM load failed:', message);
	}
}

// ---------------------------------------------------------------------------
// Set a param value (called by the param tuner UI)
// Updates both the internal registry and the session store
// ---------------------------------------------------------------------------

export function setParam(key: string, value: number): void {
	_params[key] = value;
	session.setParam(key, value);

	if (window.__ludeme_set_param) {
		window.__ludeme_set_param(key, value);
	}
}
