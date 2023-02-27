import type { Card } from '@bindings/Card';
import type { CardAdd } from '@bindings/CardAdd';
import type { Pack } from '@bindings/Pack';
import type { PackCreate } from '@bindings/PackCreate';
import type { PackUpdate } from '@bindings/PackUpdate';
import type { invoke as invoke_ } from '@tauri-apps/api';

type Window = {
    __TAURI__: {
        invoke: typeof invoke_;
    };
};

const invoke: typeof invoke_ = async (cmd, args) => {
    const invoke = (window as unknown as Window).__TAURI__.invoke;
    return await invoke(cmd, args);
};

export const createPack = (pack: PackCreate) => {
    return invoke<void>('create_pack', { pack });
};

export const listPacks = () => {
    return invoke<Pack[]>('list_packs');
};

export const getPack = (id: string) => {
    return invoke<Pack>('get_pack', { id });
};

export const deletePack = (id: string) => {
    return invoke<void>('delete_pack', { id });
};

export const updatePack = (update: PackUpdate) => {
    return invoke<void>('update_pack', { update });
};

export const listCards = (id: string) => {
    return invoke<Card[]>('list_cards', { id });
};

export const addCard = (card: CardAdd) => {
    return invoke<void>('add_card', { card });
};
