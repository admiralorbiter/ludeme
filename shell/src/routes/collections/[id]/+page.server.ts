import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { api } from '$lib/api';

export const load: PageServerLoad = async ({ params, fetch }) => {
	const res = await fetch(api(`/collections/${params.id}`));
	if (!res.ok) {
		throw error(404, 'Collection not found');
	}
	const collection = await res.json();
	return { collection };
};
