import { invoke } from '$lib/commands';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
    const id = parseInt(params.cardId);

    const prompt = await invoke('generate_prompt', { cardId: id });

    return {
        id,
        prompt,
    };
};
