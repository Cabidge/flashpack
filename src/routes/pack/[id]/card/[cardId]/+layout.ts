import type { LayoutLoad } from "./$types";

export const load: LayoutLoad = async ({ params, parent }) => {
    const { cardId } = params;
    const { packHref } = await parent();

    const cardHref = `${packHref}/card/${cardId}`

    return {
        cardHref,
    }
};