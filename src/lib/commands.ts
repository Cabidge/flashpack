import type { Card } from '@bindings/Card';
import type { FullPrompt } from '@bindings/FullPrompt';
import type { ModifyCard } from '@bindings/ModifyCard';
import type { ModifyPack } from '@bindings/ModifyPack';
import type { ModifyStudy } from '@bindings/ModifyStudy';
import type { Pack } from '@bindings/Pack';
import type { Prompt } from '@bindings/Prompt';
import type { Study } from '@bindings/Study';
import type { StudyTags } from '@bindings/StudyTags';

import { banners } from './banners';

type PackId = number;
type CardId = number;
type StudyId = number;

type Commands = {
    render_markdown: (args: { markdown: string }) => string;
    generate_prompt: (args: { script: string; front: string; back: string }) => Prompt;
    // pack
    pack_create: (args: { title: string }) => void;
    pack_list: () => Record<PackId, Pack>;
    pack_cards: (args: { id: PackId }) => Record<CardId, Card>;
    pack_modify: (args: { id: PackId; action: ModifyPack }) => void;
    // card
    card_create: (args: { packId: PackId; label: string }) => void;
    card_query: (args: {
        packId?: number;
        include: string[];
        exclude: string[];
        limit?: number;
    }) => FullPrompt[];
    card_tags: (args: { id: CardId }) => string[];
    card_modify: (args: { id: CardId; action: ModifyCard }) => void;
    // study
    study_create: (title: string) => void;
    study_list: () => Record<StudyId, Study>;
    study_tags: (args: { id: StudyId }) => StudyTags;
    study_modify: (args: { id: StudyId; action: ModifyStudy }) => void;
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
