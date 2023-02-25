import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
    const { cards } = await parent();

    return {
        cards
    };
};
