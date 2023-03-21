import { invoke } from '$lib/commands';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
    const id = parseInt(params.cardId);

    const card = await invoke('get_card', { id });

    return {
        id,
        card
    };
};
