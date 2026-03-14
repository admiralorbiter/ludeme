import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { api } from '$lib/api';

export const load: PageServerLoad = async ({ params, fetch }) => {
	const [workRes, edgesFromRes] = await Promise.all([
		fetch(api(`/works/${params.id}`)),
		fetch(api(`/edges?from_id=${params.id}`))
	]);

	if (!workRes.ok) {
		throw error(404, 'Work not found');
	}

	const work = await workRes.json();
	const edgesFrom = edgesFromRes.ok ? await edgesFromRes.json() : [];

	// Fetch linked demos (edges from work to demo: relation_type=demonstrates)
	const demoEdges = edgesFrom.filter((e: any) => e.to_type === 'demo');
	const linkedDemos = [];
	for (const edge of demoEdges) {
		const demoRes = await fetch(api(`/demos/${edge.to_id}`));
		if (demoRes.ok) {
			linkedDemos.push(await demoRes.json());
		}
	}

	// Fetch linked mechanics via demos' mechanic_tags
	const mechanicTags = new Set<string>();
	for (const demo of linkedDemos) {
		if (demo.mechanic_tags) {
			for (const tag of JSON.parse(demo.mechanic_tags)) {
				mechanicTags.add(tag);
			}
		}
	}

	// Fetch mechanics list and filter to linked ones
	const mechRes = await fetch(api('/mechanics'));
	const allMechanics = mechRes.ok ? await mechRes.json() : [];
	const linkedMechanics = allMechanics.filter((m: any) => mechanicTags.has(m.family));

	return { work, linkedDemos, linkedMechanics };
};
