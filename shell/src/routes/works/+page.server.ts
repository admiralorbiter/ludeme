import type { PageServerLoad } from './$types';
import { api } from '$lib/api';

export const load: PageServerLoad = async ({ fetch }) => {
	const res = await fetch(api('/works'));
	const works = res.ok ? await res.json() : [];
	return { works };
};
