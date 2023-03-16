import type { CardAdd } from '@bindings/CardAdd';
import type { Pack } from '@bindings/Pack';
import type { PackCreate } from '@bindings/PackCreate';
import type { PackSummary } from '@bindings/PackSummary';
import type { PackUpdate } from '@bindings/PackUpdate';

type Commands = {
    create_pack: (args: { pack: PackCreate }) => void;
    list_packs: () => PackSummary[];
    get_pack: (args: { id: number }) => Pack;
    delete_pack: (args: { id: number }) => void;
    update_pack: (args: { update: PackUpdate }) => void;
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
