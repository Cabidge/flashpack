import { page } from '$app/stores';
import { derived } from 'svelte/store';

export const createUrlMatches = (url: string, strict = true) => {
    return derived<typeof page, boolean>(page, ($page, set) => {
        const pathname = $page.url.pathname;
        const matches = strict ? pathname === url : pathname.startsWith(url);

        set(matches);
    });
};
