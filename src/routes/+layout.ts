import type { LayoutLoad } from './$types';

export const prerender = 'auto';
export const ssr = false;

export const load: LayoutLoad = async ({ url }) => {
    return {
        href: url.pathname
    };
};
