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

    prompt.front = await invoke('render_markdown', { markdown: prompt.front });
    prompt.back = await invoke('render_markdown', { markdown: prompt.back });

    return {
        prompt
    };
};
