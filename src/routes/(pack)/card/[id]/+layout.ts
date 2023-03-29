import { invoke } from '$lib/commands';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ params }) => {
    const id = parseInt(params.id);
    const card = await invoke('get_card', { id });

    return {
        id,
        card
    };
};
