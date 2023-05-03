import { getContext, setContext } from 'svelte';

export type GetContext<T> = () => T;
export type SetContext<T> = (value: T) => void;

export type Context<T> = {
    get: GetContext<T>;
    set: SetContext<T>;
};

export const createContext = <T>(defaultValue: T | (() => T)): Context<T> => {
    const key = Symbol();

    const getDefaultValue =
        typeof defaultValue === 'function' ? (defaultValue as () => T) : () => defaultValue;

    const get = () => (getContext(key) as T | undefined) ?? getDefaultValue();
    const set = (value: T) => setContext(key, value);

    return { get, set };
};
