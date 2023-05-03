import { derived, writable } from 'svelte/store';

const BANNER_TIMEOUT = 5000;

export type Banner = {
    heading: string;
    details?: string;
};

const createStore = () => {
    const banners = writable<Set<Banner>>(new Set());

    const remove = (banner: Banner) => {
        banners.update((banners) => {
            banners.delete(banner);
            return banners;
        });
    };

    const addUntimed = (heading: string, details?: string) => {
        const banner = { heading, details };
        banners.update((banners) => banners.add(banner));
        return banner;
    };

    const add = (heading: string, details?: string) => {
        const banner = addUntimed(heading, details);
        setTimeout(() => remove(banner), BANNER_TIMEOUT);
    };

    const { subscribe } = derived(banners, ($banners) => [...$banners]);

    return {
        subscribe,
        add,
        remove
    };
};

export const banners = createStore();
