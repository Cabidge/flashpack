import { invoke } from '$lib/commands';
import { writable } from 'svelte/store';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async () => {
    const packs = await invoke('list_packs');

    const activePack = writable<number | null>(null);

    return { packs, activePack };
};
