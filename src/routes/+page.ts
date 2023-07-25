import { invoke } from '$lib/commands';
import type { PageLoad } from './$types';

export const load = (async ({ depends }) => {
    depends('flashpack:packs');

    return {
        packs: invoke('pack_list')
    };
}) satisfies PageLoad;
