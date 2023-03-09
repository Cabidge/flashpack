<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import type { Pack } from '@bindings/Pack';
    import { invoke } from '$lib/commands';

    export let pack: Pack;
    export let close: () => void;

    $: title = pack.title;
    $: id = pack.id;

    $: newTitle = title;

    const submit = async () => {
        close();

        if (canSave) {
            await invoke('update_pack', { update: { id, title: newTitle } });
            await invalidateAll();
        }
    };

    $: canSave = newTitle !== title && newTitle !== '';
</script>

<form on:submit|preventDefault={submit}>
    <input type="text" bind:value={newTitle} />
    <button type="submit">save</button>
</form>
