import { invoke } from '$lib/commands';
import type { Prompt } from '@bindings/Prompt';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
    const {
        card: { script, front, back }
    } = await parent();

    const prompt: Prompt =
        script === null
            ? { front, back }
            : await invoke('generate_prompt', { script, front, back });

    return {
        prompt
    };
};
