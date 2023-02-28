<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import type { Pack } from '@bindings/Pack';
    import { updatePack } from './commands';
    import Modal from './Modal.svelte';

    export let pack: Pack;

    $: title = pack.title;
    $: id = pack.id;

    let newTitle: string;

    let active = false;

    const close = () => (active = false);

    const submit = async () => {
        close();

        await updatePack({ id, title: newTitle });
        await invalidateAll();
    };

    $: if (active) {
        newTitle = title;
    }

    $: canSave = newTitle !== title && newTitle !== '';
</script>

<button on:click={() => (active = true)}>rename</button>

<Modal {active}>
    <form on:submit|preventDefault={() => submit}>
        <input type="text" bind:value={newTitle} />
        <button type="submit" disabled={!canSave}>save</button>
        <button type="button" on:click={close}>cancel</button>
    </form>
</Modal>
