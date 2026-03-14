/**
 * shell/src/routes/+page.server.ts
 *
 * Server-side data loader for the Discover / home page.
 * Fetches demos, mechanics, and taxonomy families from the Ludeme API.
 */

import type { PageServerLoad } from './$types.js';

// Icons for mechanic families — not stored in the database, defined here.
const FAMILY_ICONS: Record<string, string> = {
	'movement':           '↑',
	'collision-response': '◈',
	'scoring-pressure':   '▲',
	'state-transitions':  '⬡',
	'economy':            '◎',
	'timing-windows':     '◷',
	'spatial-rules':      '⬛',
	'ai-behavior':        '◉',
	'progression':        '▶',
	'information':        '◌',
};

interface ApiDemo {
	id: string;
	title: string;
	era: string | null;
	platform: string | null;
	mechanic_tags: string;      // JSON-encoded array from SQLite
	fidelity_level: string;
	description: string | null;
	publish_state: string;
	[key: string]: unknown;
}

interface ApiFamily {
	slug: string;
	label: string;
	description: string | null;
}

export const load: PageServerLoad = async ({ fetch }) => {
	const [demosRes, familiesRes] = await Promise.all([
		fetch('/api/demos'),
		fetch('/api/taxonomy/families'),
	]);

	const rawDemos: ApiDemo[] = demosRes.ok ? await demosRes.json() : [];
	const rawFamilies: ApiFamily[] = familiesRes.ok ? await familiesRes.json() : [];

	// Transform API rows into the shape the template expects
	const demos = rawDemos.map(d => ({
		id:             d.id,
		title:          d.title,
		era:            d.era ?? '',
		platform:       d.platform ?? '',
		mechanic_tags:  parseTags(d.mechanic_tags),
		fidelity:       d.fidelity_level,
		description:    d.description ?? '',
	}));

	const families = rawFamilies.map(f => ({
		slug:  f.slug,
		label: f.label,
		icon:  FAMILY_ICONS[f.slug] ?? '●',
	}));

	return { demos, families };
};

/** Parse mechanic_tags — could be a JSON string or already an array */
function parseTags(tags: unknown): string[] {
	if (Array.isArray(tags)) return tags;
	if (typeof tags === 'string') {
		try { return JSON.parse(tags); }
		catch { return []; }
	}
	return [];
}
