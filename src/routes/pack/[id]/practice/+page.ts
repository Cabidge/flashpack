import { invoke } from '$lib/commands';
import type { PageLoad } from './$types';

export const load = (async ({ parent }) => {
    // TODO: change accepted id type to a plain string
    const id = (await parent()).pack.id;

    return {
        questions: invoke('pack_generate_practice', { id })
    };
}) satisfies PageLoad;
