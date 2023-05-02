import { invoke } from '$lib/commands';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async () => {
    const packs = await invoke('list_packs');

    return { packs };
};
