import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ params }) => {
    const packHref = `/pack/${params.id}`;

    return { packHref };
};
