import { banners } from '$lib/banners';
import { invoke } from '$lib/commands';
import { createContext } from '$lib/context';
import type { Card } from '@bindings/Card';
import { listen } from '@tauri-apps/api/event';
import { derived, type Readable } from 'svelte/store';

export const createStore = (packId: Readable<number>) => {
    const cards = derived(
        packId,
        ($packId, set) => {
            const reloadCards = () => invoke('pack_cards', { id: $packId }).then(set);

            reloadCards();

            const unlisten = listen('update:cards', reloadCards);

            return async () => {
                // listen returns a Promise to a function that removes the listener
                (await unlisten)();
            };
        },
        [] as Card[]
    );

    const get = (id: Readable<number>) =>
        derived([cards, id], ([$cards, $id]) => {
            return ($cards.find((card) => card.id === $id) ?? {
                id: -1,
                label: 'Deleted Card',
                script: null,
                front: '...',
                back: '...'
            }) satisfies Card;
        });

    return {
        ...cards,
        get
    };
};

export type CardsStore = ReturnType<typeof createStore>;

export const cardsContext = createContext<CardsStore>(() => {
    banners.add('Cards Context not found...');
    throw new Error('No cards context found...');
});
