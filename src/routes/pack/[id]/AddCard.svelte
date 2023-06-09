<script lang="ts">
    import { invoke } from '$lib/commands';
    import { getModalContext } from '$lib/modals';
    import type { CardsStore } from '$lib/stores/cards';

    export let id: number;
    export let cards: CardsStore;

    const { close } = getModalContext();

    let label = '';

    const submit = async () => {
        close();

        await invoke('card_create', { packId: id, label });
        cards.reload();
    };
</script>

<form on:submit|preventDefault={submit}>
    <input type="text" bind:value={label} placeholder="Name" required />

    <button type="submit">Add</button>
</form>
