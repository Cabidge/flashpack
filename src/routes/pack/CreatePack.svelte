<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import { createPack } from '$lib/commands';

    export let close = () => {};

    let title = '';

    const submit = async () => {
        close();

        await createPack({ title });
        await invalidateAll();
    };
</script>

<form on:submit|preventDefault={submit}>
    <input type="text" bind:value={title} />
    <button type="submit">+</button>
</form>
