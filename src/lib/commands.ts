import type { Card } from '@bindings/Card';
import type { CardSlides } from '@bindings/CardSlides';
import type { ModifyCard } from '@bindings/ModifyCard';
import type { ModifyPack } from '@bindings/ModifyPack';
import type { Pack } from '@bindings/Pack';

type Commands = {
    generate_card_slides: (args: { script: string; template: string }) => CardSlides;
    // pack
    pack_create: (args: { title: string }) => void;
    pack_list: () => Pack[];
    pack_by_id: (args: { id: Pack['id'] }) => Pack | null;
    pack_modify: (args: { id: Pack['id']; action: ModifyPack }) => void;
    // card
    card_create: (args: { packId: Pack['id']; label: string }) => void;
    card_by_pack: (args: { id: Pack['id'] }) => Card[];
    card_by_id: (args: { id: Card['id'] }) => Card | null;
    card_modify: (args: { id: Card['id']; action: ModifyCard }) => void;
};

type Invoke = <T extends keyof Commands>(
    cmd: T,
    ...args: Parameters<Commands[T]>
) => Promise<ReturnType<Commands[T]>>;

type Window = {
    __TAURI__: {
        invoke: Invoke;
    };
};

export const invoke: Invoke = async (cmd, ...args) => {
    const invoke = (window as unknown as Window).__TAURI__.invoke;
    return await invoke(cmd, ...args);
};
