import type { PageServerLoad } from './$types';
import { api } from '$lib/api';

export const load: PageServerLoad = async ({ fetch }) => {
	const [mechRes, famRes] = await Promise.all([
		fetch(api('/mechanics')),
		fetch(api('/taxonomy/families'))
	]);

	const mechanics = mechRes.ok ? await mechRes.json() : [];
	const families = famRes.ok ? await famRes.json() : [];

	return { mechanics, families };
};
