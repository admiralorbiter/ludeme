import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { api } from '$lib/api';

export const load: PageServerLoad = async ({ params, fetch }) => {
	const [mechRes, edgesRes] = await Promise.all([
		fetch(api(`/mechanics/${params.id}`)),
		fetch(api(`/edges?to_id=${params.id}`))
	]);

	if (!mechRes.ok) {
		throw error(404, 'Mechanic not found');
	}

	const mechanic = await mechRes.json();

	// Get edges pointing TO this mechanic (e.g. demos that demonstrate it)
	// and edges FROM this mechanic
	const edgesToThis = edgesRes.ok ? await edgesRes.json() : [];

	// Also get edges FROM this mechanic
	const edgesFromRes = await fetch(api(`/edges?from_id=${params.id}`));
	const edgesFromThis = edgesFromRes.ok ? await edgesFromRes.json() : [];

	// Fetch linked demos — edges where this mechanic is demonstrated
	// Edge pattern: from=mechanic, to=demo, relation_type=demonstrated-in
	const demoEdges = edgesFromThis.filter((e: any) => e.to_type === 'demo');
	const linkedDemos = [];

	for (const edge of demoEdges) {
		const demoRes = await fetch(api(`/demos/${edge.to_id}`));
		if (demoRes.ok) {
			linkedDemos.push(await demoRes.json());
		}
	}

	return { mechanic, linkedDemos, edgesToThis, edgesFromThis };
};
