import type { Card } from '@bindings/Card';
import type { Pack } from '@bindings/Pack';
import { invoke } from '@lib/commands';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
    const id = params.id;

    const pack = await invoke<Pack>('get_pack', { id });
    const cards = await invoke<Card[]>('list_cards', { id });

    return {
        pack,
        cards
    };
};
