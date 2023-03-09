import { invoke } from '$lib/commands';
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
    const id = params.id;

    try {
        const pack = await invoke('getPack', { id });
        const cards = await invoke('listCards', { id });

        return {
            pack,
            cards
        };
    } catch {
        throw error(404, 'Not found');
    }
};
