import { invoke } from '$lib/commands';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
    const { id } = await parent();

    const prompt = await invoke('generate_prompt', { cardId: id });

    return {
        prompt
    };
};
