import type { Pack } from "@bindings/Pack";
import { invoke } from "@tauri-apps/api";
import { writable } from "svelte/store";

const loadPacks = () => invoke("list_packs") as Promise<Pack[]>;

export type Packs = {
    list: Pack[],
    loading: boolean,
};

const createPacks = () => {
    const { subscribe, set, update } = writable<Packs>({ list: [], loading: true });

    const load = async () => {
        const list = await loadPacks();
        set({ list, loading: false })
    };

    load();

    return {
        subscribe,
        reload: () => {
            update(({list}) => ({list, loading: true}));
            load();
        }
    }
};

export const packs = createPacks();
