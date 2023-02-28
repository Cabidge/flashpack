<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import { createEventDispatcher } from 'svelte';
    import { createPack } from './commands';

    const dispatch = createEventDispatcher<{ close: undefined }>();

    let title = '';

    const submit = async () => {
        dispatch('close');

        await createPack({ title });
        await invalidateAll();
    };
</script>

<form on:submit|preventDefault={submit}>
    <input type="text" bind:value={title} />
    <button type="submit">+</button>
</form>
