<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import { createPack } from './commands';
    import Modal from './Modal.svelte';

    let active = false;

    let title = '';

    $: {
        if (!active) {
            title = '';
        }
    }

    const submit = async () => {
        active = false;

        await createPack({ title });
        await invalidateAll();
    };
</script>

<button class="rounded bg-blue-500 text-white hover:bg-blue-600 text-center w-full py-2 shadow font-semibold" on:click={() => (active = true)}>Create Pack</button>

<Modal bind:active>
    <form on:submit|preventDefault={submit}>
        <input type="text" bind:value={title} />
        <button type="submit">+</button>
        <button type="button" on:click={() => (active = false)}>cancel</button>
    </form>
</Modal>
