<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import { invoke } from '$lib/commands';
    import { getModalContext } from '$lib/modals';

    export let id: number;

    const { close } = getModalContext();

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
