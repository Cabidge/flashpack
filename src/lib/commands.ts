import type { Card } from '@bindings/Card';
import type { CardAdd } from '@bindings/CardAdd';
import type { Pack } from '@bindings/Pack';
import type { PackCreate } from '@bindings/PackCreate';
import type { PackUpdate } from '@bindings/PackUpdate';

type Commands = {
    create_pack: (args: { pack: PackCreate }) => void;
    list_packs: () => Pack[];
    get_pack: (args: { id: string }) => Pack;
    delete_pack: (args: { id: string }) => void;
    update_pack: (args: { update: PackUpdate }) => void;
    list_cards: (args: { id: string }) => Card[];
    add_card: (args: { card: CardAdd }) => void;
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
