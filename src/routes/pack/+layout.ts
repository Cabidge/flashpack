import { invoke, listPacks } from '@lib/commands';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ url }) => {
    const packs = await listPacks();

    return { packs, href: url.href };
};
