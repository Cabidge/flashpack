import { listen } from '@tauri-apps/api/event';
import { invoke } from '$lib/commands';
import type { Pack } from '@bindings/Pack';
import { derived, readable, type Readable } from 'svelte/store';

export type PackWithId = Pack & {
    id: number;
};

export type PacksStore = Readable<PackWithId[]> & {
    get: (id: Readable<number>) => Readable<Pack>;
};

const createStore = (): PacksStore => {
    const packs = readable([] as PackWithId[], (set) => {
        const reload = () => {
            invoke('pack_list').then((packMap) => {
                const packs = Object.entries(packMap).map(([id, pack]) => ({
                    id: Number(id),
                    ...pack
                }));
                set(packs);
            });
        };

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
                    title: 'Deleted Pack'
                }
            );
        });

    return {
        ...packs,
        // Creates a store pointing to the specific pack
        get
    };
};

export const packs = createStore();
