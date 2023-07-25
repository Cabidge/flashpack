import { invoke } from '$lib/commands';
import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load = (async ({ depends, params }) => {
    depends('flashpack:cards');

    // TODO: change accepted id type to a plain string
    const id = parseInt(params.id) as unknown as bigint;
    const card = await invoke('card_by_id', { id });

    if (card === null) {
        throw redirect(307, '/');
    }

    return {
        card
    };
}) satisfies PageLoad;
