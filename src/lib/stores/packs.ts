import { listen } from '@tauri-apps/api/event';
import { invoke } from '$lib/commands';
import type { Pack } from '@bindings/Pack';
import { derived, writable, type Readable } from 'svelte/store';

export type PackWithId = Pack & {
    id: number;
};

export type PacksStore = Readable<PackWithId[]> & {
    reload: () => void;
    get: (id: Readable<number>) => Readable<Pack>;
};

const createStore = (): PacksStore => {
    const trigger = writable(null);
    const runTrigger = () => trigger.set(null);

    // Turn the packs into an array
    const { subscribe } = derived(
        trigger,
        (_$trigger, set) => {
            const reloadPacks = () =>
                invoke('pack_list').then((packs) =>
                    set(Object.entries(packs).map(([id, pack]) => ({ id: Number(id), ...pack })))
                );

            reloadPacks();

            const unlisten = listen('update:packs', reloadPacks);

            return async () => {
                // listen returns a Promise to a function that removes the listener
                (await unlisten)();
            };
        },
        [] as PackWithId[]
    );

    const reload = runTrigger;

    const get = (id: Readable<number>) =>
        derived([packs, id], ([$packs, $id]) => {
            return (
                $packs.find((pack) => pack.id === $id) ?? {
                    title: 'Deleted Pack'
                }
            );
        });

    return {
        subscribe,
        // Refetches the packs from the database
        reload,
        // Creates a store pointing to the specific pack
        get
    };
};

export const packs = createStore();
