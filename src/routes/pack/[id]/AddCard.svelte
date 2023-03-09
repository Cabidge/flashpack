<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import { invoke } from '$lib/commands';

    export let id: string;
    export let close: () => void;

    let front = '';
    let back = '';

    const submit = async () => {
        close();

        const card = { pack_id: id, front, back };

        await invoke('add_card', { card });
        await invalidateAll();
    };
</script>

<form on:submit|preventDefault={submit}>
    <input type="text" bind:value={front} placeholder="front" required />
    <input type="text" bind:value={back} placeholder="back" required />

    <button type="submit">add</button>
</form>
