<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import type { PackCreate } from '@bindings/PackCreate';
    import { invoke } from '@tauri-apps/api';

    let active = false;

    let title = '';

    let creating: Promise<void> = Promise.resolve();

    $: {
        if (!active) {
            title = '';
        }
    }

    const _submit = async () => {
        active = false;

        const pack: PackCreate = { title };
        await invoke('create_pack', { pack });

        await invalidateAll();
    };

    const submit = () => {
        creating = _submit();
    };
</script>

{#await creating}
    <p>creating...</p>
{:then}
    {#if active}
        <form on:submit|preventDefault={submit}>
            <input type="text" bind:value={title} />
            <button type="submit">+</button>
            <button type="button" on:click={() => (active = false)}>cancel</button>
        </form>
    {:else}
        <button on:click={() => (active = true)}>create pack...</button>
    {/if}
{/await}
