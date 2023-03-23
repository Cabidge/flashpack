import { invoke } from '$lib/commands';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
    const id = parseInt(params.filterId);
    const filter = await invoke('get_filter', { id });

    return {
        id,
        filter
    };
};
