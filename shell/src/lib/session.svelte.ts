/**
 * shell/src/lib/session.svelte.ts
 *
 * Svelte 5 runes-based session store.
 * Tracks the full lifecycle of a play session: loading → active → ended.
 * All game events from the WASM module flow through here.
 */

import type {
	PlayableDemo,
	ParamManifest,
	ParamDescriptor,
	GameEvent,
	SessionStart,
	SessionEnd,
	MomentEmit,
	StateChange,
	ParamChange,
	MomentBookmark,
} from './types.js';

// ---------------------------------------------------------------------------
// Session lifecycle
// ---------------------------------------------------------------------------

export type SessionStatus =
	| 'idle'        // No demo loaded
	| 'loading'     // WASM is being fetched/compiled
	| 'ready'       // WASM loaded, waiting for SessionStart event
	| 'active'      // Session in progress
	| 'paused'      // Paused (e.g. bookmark capture in progress)
	| 'ended'       // SessionEnd received
	| 'error';      // WASM load failed or runtime error

export interface SessionError {
	code: 'wasm_load_failed' | 'wasm_no_path' | 'wasm_runtime' | 'event_parse';
	message: string;
}

// ---------------------------------------------------------------------------
// Param state — current values across all params
// ---------------------------------------------------------------------------

export type ParamValues = Record<string, number>;

function defaultParamValues(manifest: ParamManifest): ParamValues {
	const values: ParamValues = {};
	for (const p of manifest.params) {
		values[p.key] = p.default;
	}
	return values;
}

// ---------------------------------------------------------------------------
// Session store — Svelte 5 runes
// ---------------------------------------------------------------------------

function createSessionStore() {
	// Core state
	let status = $state<SessionStatus>('idle');
	let demo = $state<PlayableDemo | null>(null);
	let error = $state<SessionError | null>(null);

	// Session data
	let sessionId = $state<string | null>(null);
	let frameCount = $state<number>(0);
	let durationMs = $state<number>(0);
	let seed = $state<number>(0);
	let branchId = $state<string>('main');

	// Params
	let paramManifest = $state<ParamManifest | null>(null);
	let paramValues = $state<ParamValues>({});

	// State machine
	let currentStates = $state<string[]>([]);
	let lastStateChange = $state<StateChange | null>(null);

	// Bookmarks captured this session
	let pendingMoment = $state<MomentEmit | null>(null);
	let sessionBookmarks = $state<MomentBookmark[]>([]);

	// Event log (capped at last 200)
	let eventLog = $state<GameEvent[]>([]);
	const EVENT_LOG_MAX = 200;

	// ---------------------------------------------------------------------------
	// Internal helpers
	// ---------------------------------------------------------------------------

	function logEvent(event: GameEvent) {
		eventLog = [...eventLog.slice(-(EVENT_LOG_MAX - 1)), event];
	}

	// ---------------------------------------------------------------------------
	// Event handler — called by ludeme-shell.ts when WASM emits
	// ---------------------------------------------------------------------------

	function handleEvent(event: GameEvent) {
		logEvent(event);

		switch (event.type) {
			case 'session_start': {
				const e = event.data as SessionStart;
				sessionId = crypto.randomUUID();
				seed = e.seed;
				branchId = e.branch_id;
				paramManifest = e.param_manifest;
				paramValues = defaultParamValues(e.param_manifest);
				frameCount = 0;
				durationMs = 0;
				currentStates = [];
				status = 'active';
				break;
			}

			case 'session_end': {
				const e = event.data as SessionEnd;
				frameCount = e.frame_count;
				durationMs = e.duration_ms;
				status = 'ended';
				break;
			}

			case 'moment_emit': {
				pendingMoment = event.data as MomentEmit;
				status = 'paused';
				break;
			}

			case 'state_change': {
				const e = event.data as StateChange;
				lastStateChange = e;
				// Update active states list
				currentStates = [...currentStates.filter(s => s !== e.from_state), e.to_state];
				break;
			}

			case 'param_change': {
				const e = event.data as ParamChange;
				paramValues = { ...paramValues, [e.key]: e.value };
				break;
			}

			case 'branch_change': {
				branchId = event.data.to_branch;
				break;
			}

			case 'frame_tick': {
				frameCount = event.data.frame;
				currentStates = event.data.active_states;
				break;
			}
		}
	}

	// ---------------------------------------------------------------------------
	// Public actions
	// ---------------------------------------------------------------------------

	function setDemo(d: PlayableDemo) {
		demo = d;
		status = 'idle';
		error = null;
		sessionId = null;
		paramManifest = null;
		paramValues = {};
		currentStates = [];
		pendingMoment = null;
		eventLog = [];
	}

	function setStatus(s: SessionStatus) { status = s; }

	function setError(e: SessionError) {
		error = e;
		status = 'error';
	}

	function setParam(key: string, value: number) {
		paramValues = { ...paramValues, [key]: value };
	}

	function dismissMoment() {
		pendingMoment = null;
		if (status === 'paused') status = 'active';
	}

	function confirmBookmark(label: string, tags: string[]) {
		if (!pendingMoment || !demo) return;
		const bookmark: MomentBookmark = {
			id: crypto.randomUUID(),
			session_id: sessionId,
			demo_id: demo.id,
			frame: pendingMoment.frame,
			player_label: label || null,
			auto_tags: tags,
			screenshot_url: null, // set after canvas capture
		};
		sessionBookmarks = [...sessionBookmarks, bookmark];
		dismissMoment();
	}

	function reset() {
		status = 'idle';
		demo = null;
		error = null;
		sessionId = null;
		frameCount = 0;
		durationMs = 0;
		seed = 0;
		branchId = 'main';
		paramManifest = null;
		paramValues = {};
		currentStates = [];
		lastStateChange = null;
		pendingMoment = null;
		sessionBookmarks = [];
		eventLog = [];
	}

	return {
		// Readable state
		get status()           { return status; },
		get demo()             { return demo; },
		get error()            { return error; },
		get sessionId()        { return sessionId; },
		get frameCount()       { return frameCount; },
		get durationMs()       { return durationMs; },
		get seed()             { return seed; },
		get branchId()         { return branchId; },
		get paramManifest()    { return paramManifest; },
		get paramValues()      { return paramValues; },
		get currentStates()    { return currentStates; },
		get lastStateChange()  { return lastStateChange; },
		get pendingMoment()    { return pendingMoment; },
		get sessionBookmarks() { return sessionBookmarks; },
		get eventLog()         { return eventLog; },

		// Derived
		get isActive()  { return status === 'active'; },
		get isPaused()  { return status === 'paused'; },
		get isLoading() { return status === 'loading'; },
		get hasError()  { return status === 'error'; },

		// Actions
		handleEvent,
		setDemo,
		setStatus,
		setError,
		setParam,
		dismissMoment,
		confirmBookmark,
		reset,
	};
}

// Singleton — one session at a time
export const session = createSessionStore();
