import { banners } from '$lib/banners';
import { invoke } from '$lib/commands';
import { createContext } from '$lib/context';
import type { Card } from '@bindings/Card';
import { derived, writable, type Readable } from 'svelte/store';

export const createStore = () => {
    const cards = writable<Record<number, Card>>({});

    const { subscribe } = derived(cards, ($cards) => {
        return Object.entries($cards).map(([id, card]) => ({ id: Number(id), ...card }));
    });

    let _packId: number | undefined = undefined;

    const reload = (packId?: number) => {
        if (packId !== undefined) {
            _packId = packId;
        }

        if (_packId === undefined) {
            return;
        }

        invoke('pack_cards', { id: _packId }).then((latest) => cards.set(latest));
    };

    const get = (id: Readable<number>) =>
        derived([cards, id], ([$cards, $id]) => {
            return (
                $cards[$id] ?? {
                    label: 'Deleted Card',
                    script: null,
                    front: '...',
                    back: '...'
                }
            );
        });

    return {
        subscribe,
        reload,
        get
    };
};

type CardsStore = ReturnType<typeof createStore>;

export const cardsContext = createContext<CardsStore>(() => {
    banners.add('Cards Context not found...');
    throw new Error('No cards context found...');
});
