import { invoke } from '$lib/commands';
import { redirect } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';

export const load = (async ({ depends, params }) => {
    depends('flashpack:packs', 'flashpack:cards');

    // TODO: change accepted id type to a plain string
    const id = parseInt(params.id) as unknown as bigint;
    const pack = await invoke('pack_by_id', { id });

    if (pack === null) throw redirect(307, '/');

    return {
        pack,
        cards: invoke('card_by_pack', { id })
    };
}) satisfies LayoutLoad;
