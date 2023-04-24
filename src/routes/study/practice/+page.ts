import { invoke } from '$lib/commands';
import type { Prompt } from '@bindings/Prompt';
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

const maybeParseInt = (n: string | null) => {
    return n === null ? undefined : parseInt(n);
};

const promptFromCardId = async (id: number): Promise<FullPrompt> => {
    const { script, front, back, tags } = await invoke('get_card', { id });

    const prompt =
        script === null
            ? { front, back }
            : await invoke('generate_prompt', { script, front, back });

    return {
        ...prompt,
        tags
    };
};

export type FullPrompt = Prompt & {
    tags: string[];
};

export const load: PageLoad = async ({ url }) => {
    const packId = maybeParseInt(url.searchParams.get('pack'));

    if (packId === undefined) {
        throw error(404, 'Not Found');
    }

    const include = url.searchParams.getAll('include[]');
    const exclude = url.searchParams.getAll('exclude[]');

    // first convert to number definite number, then change to undefined for zero case
    // TODO: change query_cards to only accept a number, then handle the zero case in rust
    let limit: number | undefined = maybeParseInt(url.searchParams.get('limit')) ?? 0;
    limit = Math.max(limit, 0);
    if (limit === 0) {
        limit = undefined;
    }

    const cardIds = await invoke('query_cards', { packId, include, exclude, limit });
    const questions = await Promise.all(cardIds.map(promptFromCardId));

    return {
        questions
    };
};
