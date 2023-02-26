<script lang="ts">
    import type { PackCreate } from '@bindings/PackCreate';
    import { invoke } from '@tauri-apps/api';
    import Modal from './Modal.svelte';
    import { packs } from './packs';

    let active = false;

    let title = '';

    $: {
        if (!active) {
            title = '';
        }
    }

    const submit = async () => {
        active = false;

        const pack: PackCreate = { title };
        await invoke('create_pack', { pack });

        packs.reload();
    };
</script>

<button on:click={() => (active = true)}>create pack...</button>

{#if active}
    <Modal>
        <form on:submit|preventDefault={submit}>
            <input type="text" bind:value={title} />
            <button type="submit">+</button>
            <button type="button" on:click={() => (active = false)}>cancel</button>
        </form>
    </Modal>
{/if}
