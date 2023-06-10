import { banners } from '$lib/banners';
import { invoke } from '$lib/commands';
import { createContext } from '$lib/context';
import type { Card } from '@bindings/Card';
import { listen } from '@tauri-apps/api/event';
import { derived, writable, type Readable } from 'svelte/store';

export type CardWithId = Card & {
    id: number;
};

export const createStore = (packId: Readable<number>) => {
    const trigger = writable(null);
    const runTrigger = () => trigger.set(null);

    const cards = derived(
        [packId, trigger],
        ([$packId], set) => {
            const reloadCards = () => {
                invoke('pack_cards', { id: $packId }).then((cardMap) => {
                    const cards = Object.entries(cardMap).map(([id, card]) => ({
                        id: Number(id),
                        ...card
                    }));

                    set(cards);
                });
            };

            reloadCards();

            const unlisten = listen('update:cards', reloadCards);

            return async () => {
                // listen returns a Promise to a function that removes the listener
                (await unlisten)();
            };
        },
        [] as CardWithId[]
    );

    const reload = runTrigger;

    const get = (id: Readable<number>) =>
        derived([cards, id], ([$cards, $id]) => {
            return (
                $cards.find((card) => card.id === $id) ?? {
                    label: 'Deleted Card',
                    script: null,
                    front: '...',
                    back: '...'
                }
            );
        });

    return {
        ...cards,
        reload,
        get
    };
};

export type CardsStore = ReturnType<typeof createStore>;

export const cardsContext = createContext<CardsStore>(() => {
    banners.add('Cards Context not found...');
    throw new Error('No cards context found...');
});
