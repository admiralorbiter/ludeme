/**
 * shell/src/routes/demo/[id]/+page.server.ts
 *
 * Server-side data loader for the demo play shell.
 * Fetches the demo from GET /api/demos/:id on ludeme-server.
 */

import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types.js';
import type { PlayableDemo } from '$lib/types.js';
import { api } from '$lib/api';

export const load: PageServerLoad = async ({ params, fetch }) => {
	const { id } = params;

	const res = await fetch(api(`/demos/${id}`));
	if (!res.ok) {
		error(res.status, `Demo "${id}" not found`);
	}

	const raw = await res.json();

	// Transform API row into the PlayableDemo shape the template expects
	const demo: PlayableDemo = {
		id:             raw.id,
		title:          raw.title,
		linked_work:    raw.linked_work ?? null,
		mechanic_tags:  parseJson(raw.mechanic_tags, []),
		fidelity_level: raw.fidelity_level ?? 'interpreted',
		branch_id:      raw.branch_id ?? 'main',
		wasm_path:      raw.wasm_path ?? null,
		publish_state:  raw.publish_state ?? 'draft',
		description:    raw.description ?? undefined,
		era:            raw.era ?? undefined,
		platform:       raw.platform ?? undefined,
		state_graph:    parseJson(raw.state_graph, null),
		notable_interpretations: parseJson(raw.notable_interpretations, undefined),
		hypothesis:     raw.hypothesis ?? undefined,
	};

	// Fetch linked work (for breadcrumb)
	let linkedWork: { id: string; title: string } | null = null;
	if (demo.linked_work) {
		const wRes = await fetch(api(`/works/${demo.linked_work}`));
		if (wRes.ok) {
			const w = await wRes.json();
			linkedWork = { id: w.id, title: w.title };
		}
	}

	// Fetch mechanics matching demo's tags (for clickable chips)
	const mechRes = await fetch(api('/mechanics'));
	const allMechanics = mechRes.ok ? await mechRes.json() : [];
	const linkedMechanics = allMechanics
		.filter((m: any) => demo.mechanic_tags.includes(m.family))
		.map((m: any) => ({ id: m.id, name: m.name, family: m.family }));

	return { demo, linkedWork, linkedMechanics };
};

/** Safely parse a JSON string, returning the fallback if it fails */
function parseJson<T>(value: unknown, fallback: T): T {
	if (value === null || value === undefined) return fallback;
	if (typeof value !== 'string') return value as T;
	try { return JSON.parse(value); }
	catch { return fallback; }
}
