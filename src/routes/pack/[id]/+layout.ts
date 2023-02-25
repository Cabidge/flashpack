import { redirect } from '@sveltejs/kit';
import { invoke } from '@tauri-apps/api';
import type { Card } from '@bindings/Card';
import type { Pack } from '@bindings/Pack';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ params }) => {
    const id = params.id;

    try {
        const pack: Pack = await invoke('get_pack', { id });
        const cards: Card[] = await invoke('list_cards', { id });

        return { pack, cards };
    } catch {
        throw redirect(301, '/');
    }
};
