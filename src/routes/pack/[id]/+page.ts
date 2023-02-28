import { getPack, listCards } from '@lib/commands';
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
    const id = params.id;

    try {
        const pack = await getPack(id);
        const cards = await listCards(id);

        return {
            pack,
            cards
        };
    } catch {
        throw error(404, "Not found");
    }
};
