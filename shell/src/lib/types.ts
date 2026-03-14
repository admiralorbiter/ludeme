/**
 * shell/src/lib/types.ts
 *
 * TypeScript types that mirror ludeme-core Rust types.
 * These will eventually be auto-generated via `ts-rs` from the Rust source.
 * Until then, keep these in sync with crates/ludeme-core/src/
 */

// ---------------------------------------------------------------------------
// Taxonomy
// ---------------------------------------------------------------------------

export type FidelityLevel = 'faithful' | 'interpreted' | 'experimental';
export type Confidence = 'speculative' | 'tentative' | 'supported' | 'established';
export type PublishState = 'draft' | 'review' | 'public';

// ---------------------------------------------------------------------------
// Entities
// ---------------------------------------------------------------------------

export interface Mechanic {
	id: string;
	name: string;
	family: string;
	short_definition: string | null;
	verbs: string[];
	failure_pattern: string | null;
	mastery_pattern: string | null;
	publish_state: PublishState;
}

export interface Work {
	id: string;
	title: string;
	year: number | null;
	platform: string | null;
	genre: string | null;
	significance: string | null;
	notable_constraints: string | null;
	publish_state: PublishState;
}

export interface PlayableDemo {
	id: string;
	title: string;
	linked_work: string | null;
	mechanic_tags: string[];
	fidelity_level: FidelityLevel;
	branch_id: string;
	wasm_path: string | null;
	publish_state: PublishState;
	/** Optional: human-readable description for the shell UI */
	description?: string;
	/** Optional: era string e.g. "1976" */
	era?: string;
	/** Optional: platform label e.g. "Arcade" */
	platform?: string;
	/** Optional: static state graph for the overlay panel */
	state_graph?: StateGraph | null;
	/** Optional: notable_interpretations for interpreted/experimental demos */
	notable_interpretations?: string[];
	/** Optional: hypothesis for experimental demos */
	hypothesis?: string;
}

export interface RelationshipEdge {
	id: string;
	from_id: string;
	from_type: string;
	to_id: string;
	to_type: string;
	relation_type: string;
	confidence: Confidence;
	note: string | null;
}

export interface MomentBookmark {
	id: string;
	session_id: string | null;
	demo_id: string;
	frame: number;
	player_label: string | null;
	auto_tags: string[];
	screenshot_url: string | null;
}

// ---------------------------------------------------------------------------
// State graph (for the state machine overlay)
// ---------------------------------------------------------------------------

export interface StateGraph {
	states: StateNode[];
	transitions: StateTransition[];
}

export interface StateNode {
	id: string;
	label: string;
}

export interface StateTransition {
	from: string;
	to: string;
	trigger: string;
}

// ---------------------------------------------------------------------------
// Shell API Contract — GameEvent types
// See docs/04-shell-api-contract.md
// ---------------------------------------------------------------------------

export type ParamKind = 'float' | 'integer' | 'toggle';

export interface ParamDescriptor {
	key: string;
	label: string;
	kind: ParamKind;
	default: number;
	min: number;
	max: number;
	step: number;
	group: string | null;
}

export interface ParamManifest {
	params: ParamDescriptor[];
}

export interface SessionStart {
	demo_id: string;
	branch_id: string;
	seed: number;
	param_manifest: ParamManifest;
}

export interface SessionEnd {
	frame_count: number;
	duration_ms: number;
	input_log_available: boolean;
}

export interface MomentEmit {
	scene_id: string;
	frame: number;
	state_blob: number[] | null;
	player_label: string | null;
	auto_tags: string[];
}

export interface StateChange {
	from_state: string;
	to_state: string;
	frame: number;
}

export interface ParamChange {
	key: string;
	value: number;
	frame: number;
}

export interface BranchChange {
	from_branch: string;
	to_branch: string;
}

export interface FrameTick {
	frame: number;
	position: [number, number] | null;
	active_states: string[];
}

export type GameEvent =
	| { type: 'session_start'; data: SessionStart }
	| { type: 'session_end'; data: SessionEnd }
	| { type: 'moment_emit'; data: MomentEmit }
	| { type: 'state_change'; data: StateChange }
	| { type: 'param_change'; data: ParamChange }
	| { type: 'branch_change'; data: BranchChange }
	| { type: 'frame_tick'; data: FrameTick };

// ---------------------------------------------------------------------------
// The window.__ludeme contract
// ---------------------------------------------------------------------------

export interface LudemeShellAPI {
	/** Called by the WASM module when it wants to emit an event */
	onEvent: (eventJson: string) => void;
	/** Called by the WASM module to read a param value */
	getParam: (key: string) => number | null;
}

declare global {
	interface Window {
		__ludeme?: LudemeShellAPI;
		__ludeme_set_param?: (key: string, value: number) => void;
	}
}
