<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import type { Pack } from '@bindings/Pack';
    import { updatePack } from '$lib/commands';

    export let pack: Pack;
    export let close = () => {};

    $: title = pack.title;
    $: id = pack.id;

    $: newTitle = title;

    const submit = async () => {
        close();

        if (canSave) {
            await updatePack({ id, title: newTitle });
            await invalidateAll();
        }
    };

    $: canSave = newTitle !== title && newTitle !== '';
</script>

<form on:submit|preventDefault={submit}>
    <input type="text" bind:value={newTitle} />
    <button type="submit">save</button>
</form>
