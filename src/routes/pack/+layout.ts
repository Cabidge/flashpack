import { invoke } from '@tauri-apps/api';
import type { Pack } from '@bindings/Pack';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async () => {
    const packs: Pack[] = await invoke('list_packs');
    return { packs };
};
