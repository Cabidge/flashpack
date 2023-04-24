import { getContext, setContext, type ComponentType, type SvelteComponentTyped } from "svelte";
import { derived, writable } from "svelte/store";

export type ModalContent = {
    component: ComponentType<SvelteComponentTyped>;
    props: object;
    close: () => void;
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type Props<T> = T extends SvelteComponentTyped<infer P, any, any> ? P : never;

const createStore = () => {
    const contents = writable<Map<symbol, ModalContent>>(new Map());

    const add = <T extends SvelteComponentTyped>(component: ComponentType<T>, props: Props<T>) => {
        const id = Symbol();
        const close = () => remove(id);

        contents.update((map) => {
            const content = { id, component, props, close };
            map.set(id, content);
            return map;
        });

        return id;
    }

    const remove = (id: symbol) => {
        contents.update((map) => {
            map.delete(id);
            return map;
        });
    }

    const { subscribe } = derived(contents, (map) => [...map.entries()]);

    return {
        subscribe,
        add,
        remove,
    }
}

export const modals = createStore();

type ModalContext = {
    close: () => void;
};

const contextKey = Symbol();

export const getModalContext = () => {
    return getContext(contextKey) as ModalContext ?? { close: () => { return; } };
};

export const setModalContext = (context: ModalContext) => {
    setContext(contextKey, context);
};
