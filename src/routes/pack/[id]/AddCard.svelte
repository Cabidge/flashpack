<script lang="ts">
    import { invoke } from '$lib/commands';
    import { getModalContext } from '$lib/modals';
    import { cardsContext } from '$lib/stores/cards';

    export let id: number;

    const { close } = getModalContext();

    let label = '';

    const cards = cardsContext.get();

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
