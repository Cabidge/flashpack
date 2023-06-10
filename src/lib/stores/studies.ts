import { invoke } from '$lib/commands';
import type { Study } from '@bindings/Study';
import { listen } from '@tauri-apps/api/event';
import { derived, readable, type Readable } from 'svelte/store';

export type StudyWithId = Study & {
    id: number;
};

export type StudiesStore = Readable<StudyWithId[]> & {
    get: (id: Readable<number>) => Readable<Study>;
};

const createStore = (): StudiesStore => {
    const studies = readable([] as StudyWithId[], (set) => {
        const reload = () => {
            invoke('study_list').then((studyMap) => {
                const study = Object.entries(studyMap).map(([id, study]) => ({
                    id: Number(id),
                    ...study
                }));

                set(study);
            });
        };

        reload();

        const unlisten = listen('update:studies', reload);

        return async () => {
            // listen returns a Promise to a function that removes the listener
            (await unlisten)();
        };
    });

    const get = (id: Readable<number>) => 
        derived([studies, id], ([$studies, $id]) => {
            return (
                $studies.find((study) => study.id === $id) ?? {
                    title: 'Deleted Study',
                    pack_id: null,
                    limit: 0
                }
            );
        });

    return {
        ...studies,
        // Creates a store pointing to the specific study
        get
    };
};

export const studies = createStore();
