<script lang="ts">
    import { createEventDispatcher } from 'svelte';

    export let title: string;
    export const id: string = '';

    let newTitle = title;

    const dispatch = createEventDispatcher<{ end: undefined }>();

    const submit = (save: boolean) => {
        if (save) {
            // TODO: tauri command
            title = newTitle;
        }

        dispatch('end');
    };

    $: canSave = newTitle !== title && newTitle !== '';
</script>

<form on:submit|preventDefault={() => submit(true)}>
    <input type="text" bind:value={newTitle} />
    <button type="submit" disabled={!canSave}>save</button>
    <button type="button" on:click={() => submit(false)}>cancel</button>
</form>
