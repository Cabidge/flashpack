import { invoke } from '$lib/commands';
import type { Study } from '@bindings/Study';
import { derived, writable, type Readable } from 'svelte/store';

export type StudyWithId = Study & {
    id: number;
};

export type StudiesStore = Readable<StudyWithId[]> & {
    reload: () => void;
    get: (id: Readable<number>) => Readable<Study>;
};

const createStore = (): StudiesStore => {
    const studies = writable<Record<number, Study>>({});

    // Turn the studies into an array
    const { subscribe } = derived(studies, ($studies) => {
        return Object.entries($studies).map(([id, study]) => ({ id: Number(id), ...study }));
    });

    const reload = () => invoke('study_list').then((latest) => studies.set(latest));

    const get = (id: Readable<number>) =>
        derived([studies, id], ([$studies, $id]) => {
            return (
                $studies[$id] ?? {
                    title: 'Deleted Study',
                    pack_id: null,
                    limit: 0
                }
            );
        });

    reload();

    return {
        subscribe,
        // Refetches the studies from the database
        reload,
        // Creates a store pointing to the specific study
        get
    };
};

export const studies = createStore();
