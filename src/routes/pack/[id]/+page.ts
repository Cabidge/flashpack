import { invoke } from '$lib/commands';
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
    const id = params.id;

    try {
        const pack = await invoke('get_pack', { id });
        const cards = await invoke('list_cards', { id });

        return {
            pack,
            cards
        };
    } catch {
        throw error(404, 'Not found');
    }
};
