import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	const res = await fetch('http://localhost:3000/api/collections');
	if (!res.ok) {
		return { collections: [] };
	}
	const collections = await res.json();
	return { collections };
};
