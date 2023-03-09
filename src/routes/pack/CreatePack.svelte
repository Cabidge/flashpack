<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import { invoke } from '$lib/commands';

    export let close: () => void;

    let title = '';

    const submit = async () => {
        close();

        await invoke('create_pack', { pack: { title } });
        await invalidateAll();
    };
</script>

<form on:submit|preventDefault={submit}>
    <input type="text" bind:value={title} required />
    <button type="submit">+</button>
</form>
