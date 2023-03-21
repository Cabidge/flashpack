<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import { invoke } from '$lib/commands';
    import type { PackSummary } from '@bindings/PackSummary';

    export let pack: PackSummary;
    export let close: () => void;

    $: title = pack.title;
    $: id = pack.id;

    $: newTitle = title;

    const submit = async () => {
        close();

        if (canSave) {
            await invoke('modify_pack', { id, action: { Rename: newTitle } });
            await invalidateAll();
        }
    };

    $: canSave = newTitle !== title && newTitle !== '';
</script>

<form on:submit|preventDefault={submit}>
    <input type="text" bind:value={newTitle} />
    <button type="submit">save</button>
</form>
