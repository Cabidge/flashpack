import { invoke } from '$lib/commands';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
    const {
        card: { script, front, back }
    } = await parent();

    const prompt = await invoke('generate_prompt', { script, front, back });

    return {
        prompt
    };
};
