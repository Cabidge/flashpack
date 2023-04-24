import type { Card } from '@bindings/Card';

import type { ModifyCard } from '@bindings/ModifyCard';
import type { ModifyPack } from '@bindings/ModifyPack';

import type { Pack } from '@bindings/Pack';

import type { PackSummary } from '@bindings/PackSummary';
import type { Prompt } from '@bindings/Prompt';
import { banners } from './banners';

type Commands = {
    render_markdown: (args: { markdown: string }) => string;
    generate_prompt: (args: { script: string; front: string; back: string }) => Prompt;
    // pack
    create_pack: (args: { title: string }) => void;
    list_packs: () => PackSummary[];
    get_pack: (args: { id: number }) => Pack;
    modify_pack: (args: { id: number; action: ModifyPack }) => void;
    // card
    create_card: (args: { packId: number; label: string }) => void;
    query_cards: (args: {
        packId: number;
        include: string[];
        exclude: string[];
        limit?: number;
    }) => number[];
    get_card: (args: { id: number }) => Card;
    modify_card: (args: { id: number; action: ModifyCard }) => void;
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

    try {
        return await invoke(cmd, ...args);
    } catch (err) {
        banners.add(`${cmd} Error`, String(err));
        throw err;
    }
};
