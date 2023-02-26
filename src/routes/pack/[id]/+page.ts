import { getPack, invoke, listCards } from '@lib/commands';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
    const id = params.id;

    const pack = await getPack(id);
    const cards = await listCards(id);

    return {
        pack,
        cards,
    };
};
