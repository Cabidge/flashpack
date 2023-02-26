import type { Card } from "@bindings/Card";
import type { Pack } from "@bindings/Pack";
import { invoke } from "@tauri-apps/api";

export type CardPack = {
    pack: Pack,
    cards: Card[],
}

export const loadPack = async (id: string) => {
    const pack = await invoke("get_pack", { id }) as Pack;
    const cards = await invoke("list_cards", { id }) as Card[];

    return {
        pack,
        cards,
    } satisfies CardPack;
};