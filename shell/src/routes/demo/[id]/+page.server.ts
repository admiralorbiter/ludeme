/**
 * shell/src/routes/demo/[id]/+page.server.ts
 *
 * Server-side data loader for the demo play shell.
 * Returns a typed PlayableDemo object for the given ID.
 *
 * Phase 0: returns mock data.
 * Phase 1: fetches from GET /api/demos/:id on ludeme-server.
 */

import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';
import type { PlayableDemo } from '$lib/types.js';

// ---------------------------------------------------------------------------
// Mock demo registry — replaced by API call in Phase 1
// ---------------------------------------------------------------------------

const MOCK_DEMOS: Record<string, PlayableDemo> = {
	'pong-76': {
		id: 'pong-76',
		title: 'Pong',
		era: '1972',
		platform: 'Arcade',
		linked_work: null,
		mechanic_tags: ['collision-response', 'scoring-pressure'],
		fidelity_level: 'faithful',
		branch_id: 'main',
		// Points to the wasm-bindgen JS glue module — it loads the .wasm internally
		wasm_path: '/demos/pong-76/pong_76.js',
		publish_state: 'public',
		description:
			'Ball and paddle — the mechanic that started everything. No friction, no gravity ' +
			'beyond deflection angle and speed. The simplest complete mechanic system in the canon.',
		state_graph: {
			states: [
				{ id: 'serve',     label: 'Serving' },
				{ id: 'rally',     label: 'Rally' },
				{ id: 'scored',    label: 'Point Scored' },
				{ id: 'game_over', label: 'Game Over' },
			],
			transitions: [
				{ from: 'serve',  to: 'rally',     trigger: 'ball launched' },
				{ from: 'rally',  to: 'scored',    trigger: 'ball missed' },
				{ from: 'scored', to: 'serve',     trigger: 'reset' },
				{ from: 'scored', to: 'game_over', trigger: '7 points reached' },
			],
		},
	},
	'maze-80': {
		id: 'maze-80',
		title: 'Maze Chase',
		era: '1980',
		platform: 'Arcade',
		linked_work: null,
		mechanic_tags: ['ai-behavior', 'state-transitions'],
		fidelity_level: 'interpreted',
		branch_id: 'main',
		wasm_path: null,
		publish_state: 'public',
		description:
			'Four distinct AI patterns operating simultaneously. Each entity uses a different ' +
			'spatial algorithm — the combined pressure of their interaction is the mechanic.',
		notable_interpretations: [
			'Controls mapped to WASD instead of joystick',
			'Scatter/chase timing preserved from original; respawn delay adjusted for modern play',
		],
		state_graph: {
			states: [
				{ id: 'scatter', label: 'Scatter' },
				{ id: 'chase', label: 'Chase' },
				{ id: 'frightened', label: 'Frightened' },
				{ id: 'eaten', label: 'Eaten' },
			],
			transitions: [
				{ from: 'scatter', to: 'chase', trigger: 'mode timer' },
				{ from: 'chase', to: 'scatter', trigger: 'mode timer' },
				{ from: 'chase', to: 'frightened', trigger: 'power-up' },
				{ from: 'scatter', to: 'frightened', trigger: 'power-up' },
				{ from: 'frightened', to: 'eaten', trigger: 'collision' },
				{ from: 'eaten', to: 'scatter', trigger: 'respawn' },
			],
		},
	},
	'jump-feel': {
		id: 'jump-feel',
		title: 'Jump Arc Study',
		era: '1985',
		platform: 'NES',
		linked_work: null,
		mechanic_tags: ['movement', 'timing-windows'],
		fidelity_level: 'experimental',
		branch_id: 'main',
		wasm_path: null,
		publish_state: 'public',
		description:
			'Variable jump height controlled by hold duration. The feel that defined a decade of ' +
			'platformers — exposed, tunable, and comparable across parameter configurations.',
		hypothesis:
			'Increasing jump gravity on the descent arc (asymmetric gravity) produces a ' +
			'more satisfying jump feel than symmetric gravity, measurable by voluntary re-jump rate.',
		state_graph: {
			states: [
				{ id: 'grounded', label: 'Grounded' },
				{ id: 'rising', label: 'Rising' },
				{ id: 'peak', label: 'Peak' },
				{ id: 'falling', label: 'Falling' },
				{ id: 'landing', label: 'Landing' },
				{ id: 'coyote', label: 'Coyote Time' },
			],
			transitions: [
				{ from: 'grounded', to: 'rising', trigger: 'jump pressed' },
				{ from: 'grounded', to: 'coyote', trigger: 'walk off edge' },
				{ from: 'coyote', to: 'rising', trigger: 'jump pressed' },
				{ from: 'coyote', to: 'falling', trigger: 'timer expires' },
				{ from: 'rising', to: 'peak', trigger: 'velocity = 0' },
				{ from: 'peak', to: 'falling', trigger: 'gravity' },
				{ from: 'falling', to: 'landing', trigger: 'ground contact' },
				{ from: 'landing', to: 'grounded', trigger: 'animation end' },
			],
		},
	},
};

// ---------------------------------------------------------------------------
// Load function
// ---------------------------------------------------------------------------

export const load: PageServerLoad = async ({ params, fetch }) => {
	const { id } = params;

	// Phase 1: replace this block with a real API call:
	// const res = await fetch(`/api/demos/${id}`);
	// if (!res.ok) error(res.status, res.statusText);
	// const demo: PlayableDemo = await res.json();

	const demo = MOCK_DEMOS[id];
	if (!demo) {
		error(404, `Demo "${id}" not found`);
	}

	return { demo };
};
