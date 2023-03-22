import { invoke } from '$lib/commands';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
    const id = parseInt(params.id);
    const dealer = await invoke('get_dealer', { id });

    return {
        id,
        dealer
    };
};
