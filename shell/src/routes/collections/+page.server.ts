import type { PageServerLoad } from './$types';
import { api } from '$lib/api';

export const load: PageServerLoad = async ({ fetch }) => {
	const res = await fetch(api('/collections'));
	if (!res.ok) {
		return { collections: [] };
	}
	const collections = await res.json();
	return { collections };
};
