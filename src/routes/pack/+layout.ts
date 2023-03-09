import { invoke } from '$lib/commands';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ url }) => {
    const packs = await invoke('listPacks');

    return { packs, href: url.href };
};
