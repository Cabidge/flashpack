import { writable } from "svelte/store"

export type ContextMenuStatus = {
    id: Symbol,
    x: number,
    y: number,
};

const createStatus = () => {
    const { subscribe, set } = writable<ContextMenuStatus | null>(null);

    return {
        subscribe,
        open: (status: ContextMenuStatus) => set(status),
        close: () => set(null),
    }
}

export const menuStatus = createStatus();
