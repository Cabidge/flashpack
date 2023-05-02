import { invoke } from "$lib/commands";
import type { Card } from "@bindings/Card";
import { derived, writable } from "svelte/store";

export const createStore = (packId: number) => {
    const cards = writable<Record<number, Card>>({});

    const { subscribe } = derived(cards, ($cards) => {
        return Object.entries($cards)
            .map(([id, card]) => ({ id: Number(id), ...card }))
    })

    const reload = () => invoke("pack_cards", { id: packId }).then((latest) => cards.set(latest));

    const get = (id: number) => derived(cards, ($cards) => {
        return $cards[id] ?? {
            label: "Deleted Card",
            script: null,
            front: "...",
            back: "...",
        };
    });

    reload();

    return {
        subscribe,
        reload,
        get,
    }
};
