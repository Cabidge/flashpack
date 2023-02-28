<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import type { Pack } from '@bindings/Pack';
    import { createEventDispatcher } from 'svelte';
    import { updatePack } from './commands';

    export let pack: Pack;

    $: title = pack.title;
    $: id = pack.id;

    $: newTitle = title;

    const dispatch = createEventDispatcher<{ close: undefined }>();

    const submit = async () => {
        dispatch('close');

        await updatePack({ id, title: newTitle });
        await invalidateAll();
    };

    $: canSave = newTitle !== title && newTitle !== '';
</script>

<form on:submit|preventDefault={submit}>
    <input type="text" bind:value={newTitle} />
    <button type="submit" disabled={!canSave}>save</button>
    <button type="button" on:click={() => dispatch('close')}>cancel</button>
</form>
