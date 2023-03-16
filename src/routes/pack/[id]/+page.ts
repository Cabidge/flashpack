import { invoke } from '$lib/commands';
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
    try {
        const id = parseInt(params.id);
        const pack = await invoke('get_pack', { id });

        return {
            id,
            pack,
        };
    } catch {
        throw error(404, 'Not found');
    }
};
