import type { Pack } from '@bindings/Pack';
import { invoke } from '@lib/commands';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ url }) => {
    const packs = await invoke<Pack[]>("list_packs");

    return { packs, href: url.href };
};
