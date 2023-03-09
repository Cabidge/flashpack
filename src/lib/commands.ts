import type { Card } from '@bindings/Card';
import type { CardAdd } from '@bindings/CardAdd';
import type { Pack } from '@bindings/Pack';
import type { PackCreate } from '@bindings/PackCreate';
import type { PackUpdate } from '@bindings/PackUpdate';

type Commands = {
    create_pack: {
        pack: PackCreate;
    };
    list_packs: {
        $output: Pack[];
    };
    get_pack: {
        id: string;
        $output: Pack;
    };
    delete_pack: {
        id: string;
    };
    update_pack: {
        update: PackUpdate;
    };
    list_cards: {
        id: string;
        $output: Card[];
    };
    add_card: {
        card: CardAdd;
    };
};

type HasOutput = {
    $output: unknown;
};

type Invoke = <T extends keyof Commands>(
    cmd: T,
    args?: Omit<Commands[T], '$output'>
) => Promise<Commands[T] extends HasOutput ? Commands[T]['$output'] : void>;

type Window = {
    __TAURI__: {
        invoke: Invoke;
    };
};

export const invoke: Invoke = async (cmd, args) => {
    const invoke = (window as unknown as Window).__TAURI__.invoke;
    return await invoke(cmd, args);
};
