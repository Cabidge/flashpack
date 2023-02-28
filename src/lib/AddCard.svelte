<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import { addCard } from '$lib/commands';
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher<{ close: undefined }>();

    export let id: string;

    let front = '';
    let back = '';

    const submit = async () => {
        dispatch('close');

        const card = { pack_id: id, front, back };

        await addCard(card);
        await invalidateAll();
    };
</script>

<form on:submit|preventDefault={submit}>
    <input type="text" bind:value={front} placeholder="front" />
    <input type="text" bind:value={back} placeholder="back" />

    <button type="submit">add</button>
</form>
