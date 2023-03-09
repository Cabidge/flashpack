import type { Card } from '@bindings/Card';
import type { CardAdd } from '@bindings/CardAdd';
import type { Pack } from '@bindings/Pack';
import type { PackCreate } from '@bindings/PackCreate';
import type { PackUpdate } from '@bindings/PackUpdate';

type Commands = {
    createPack: {
        pack: PackCreate;
    };
    listPacks: {
        $output: Pack[];
    };
    getPack: {
        id: string;
        $output: Pack;
    };
    deletePack: {
        id: string;
    };
    updatePack: {
        update: PackUpdate;
    };
    listCards: {
        id: string;
        $output: Card[];
    };
    addCard: {
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
