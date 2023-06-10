import { listen } from '@tauri-apps/api/event';
import { invoke } from '$lib/commands';
import type { Pack } from '@bindings/Pack';
import { derived, readable, type Readable } from 'svelte/store';

export type PacksStore = Readable<Pack[]> & {
    get: (id: Readable<number>) => Readable<Pack>;
};

const createStore = (): PacksStore => {
    const packs = readable([] as Pack[], (set) => {
        const reload = () => invoke('pack_list').then(set);

        reload();

        const unlisten = listen('update:packs', reload);

        return async () => {
            // listen returns a Promise to a function that removes the listener
            (await unlisten)();
        };
    });

    const get = (id: Readable<number>) =>
        derived([packs, id], ([$packs, $id]) => {
            return (
                $packs.find((pack) => pack.id === $id) ?? {
                    id: -1,
                    title: 'Deleted Pack',
                }
            ) satisfies Pack;
        });

    return {
        ...packs,
        // Creates a store pointing to the specific pack
        get
    };
};

export const packs = createStore();
