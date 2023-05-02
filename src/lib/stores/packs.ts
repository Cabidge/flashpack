import { invoke } from "$lib/commands";
import type { Pack } from "@bindings/Pack";
import { derived, writable, type Readable } from "svelte/store";

export type PackWithId = Pack & {
    id: number;
};

export type PacksStore = Readable<PackWithId[]> & {
    reload: () => void;
    get: (id: Readable<number>) => Readable<Pack>;
};

const createStore = (): PacksStore => {
    const packs = writable<Record<number, Pack>>({});

    // Turn the packs into an array
    const { subscribe } = derived(packs, ($packs) => {
        return Object.entries($packs)
            .map(([id, pack]) => ({ id: Number(id), ...pack }))
    })

    const reload = () => invoke("pack_list").then((latest) => packs.set(latest));

    const get = (id: Readable<number>) => derived([packs, id], ([$packs, $id]) => {
        return $packs[$id] ?? {
            title: "Deleted Pack"
        };
    })

    reload();

    return {
        subscribe,
        // Refetches the packs from the database
        reload,
        // Creates a store pointing to the specific pack
        get,
    }
};

export const packs = createStore();
