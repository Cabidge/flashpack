import type { ModifyCard } from '@bindings/ModifyCard';
import type { ModifyDealer } from '@bindings/ModifyDealer';
import type { ModifyFilter } from '@bindings/ModifyFilter';
import type { ModifyPack } from '@bindings/ModifyPack';

import type { Pack } from '@bindings/Pack';

import type { PackSummary } from '@bindings/PackSummary';


type Commands = {
    // pack
    create_pack: (args: { title: string }) => void;
    list_packs: () => PackSummary[];
    get_pack: (args: { id: number }) => Pack;
    modify_pack: (args: { id: number, action: ModifyPack }) => void;
    // card
    create_card: (args: { pack_id: number, front: string, back: string }) => void;
    modify_card: (args: { id: number, action: ModifyCard }) => void;
    // dealer
    create_dealer: (args: { title: string }) => void;
    modify_dealer: (args: { id: number, action: ModifyDealer }) => void;
    // filter
    create_filter: (args: { label: string }) => void;
    modify_filter: (args: { id: number, action: ModifyFilter }) => void;
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
