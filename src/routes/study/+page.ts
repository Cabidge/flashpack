import { invoke } from '$lib/commands';
import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
    const dealers = await invoke('list_dealers');

    return { dealers };
};
