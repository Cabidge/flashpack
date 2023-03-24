import { invoke } from '$lib/commands';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ params }) => {
    const id = parseInt(params.id);
    const dealer = await invoke('get_dealer', { id });

    return {
        id,
        dealer
    };
};
