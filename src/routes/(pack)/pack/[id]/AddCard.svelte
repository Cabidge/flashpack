<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import { invoke } from '$lib/commands';

    export let id: number;
    export let close: () => void;

    let label = '';

    const submit = async () => {
        close();

        await invoke('create_card', { packId: id, label });
        await invalidateAll();
    };
</script>

<form on:submit|preventDefault={submit}>
    <input type="text" bind:value={label} placeholder="Name" required />

    <button type="submit">Add</button>
</form>
