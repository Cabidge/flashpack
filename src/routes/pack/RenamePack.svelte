<script lang="ts">
    import { invoke } from '$lib/commands';
    import { getModalContext } from '$lib/modals';
    import type { Pack } from '@bindings/Pack';

    export let pack: Pack;

    const { close } = getModalContext();

    $: title = pack.title;
    $: id = pack.id;

    $: newTitle = title;

    const submit = async () => {
        close();

        if (canSave) {
            await invoke('pack_modify', { id, action: { Rename: newTitle } });
        }
    };

    $: canSave = newTitle !== title && newTitle !== '';
</script>

<form on:submit|preventDefault={submit}>
    <input type="text" bind:value={newTitle} />
    <button type="submit">save</button>
</form>
