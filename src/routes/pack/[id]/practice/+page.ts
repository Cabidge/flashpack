import { invoke } from '$lib/commands';
import type { PageLoad } from './$types';

export const load = (async ({ params }) => {
    // TODO: change accepted id type to a plain string
    const id = parseInt(params.id) as unknown as bigint;

    return {
        id,
        questions: invoke('pack_generate_practice', { id })
    };
}) satisfies PageLoad;
