import { invoke } from '$lib/commands';
import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
    const sessions = await invoke('list_sessions');

    return {
        sessions
    };
};
