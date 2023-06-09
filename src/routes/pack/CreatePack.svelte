<script lang="ts">
    import { invoke } from '$lib/commands';
    import { getModalContext } from '$lib/modals';
    import { packs } from '$lib/stores/packs';

    const { close } = getModalContext();

    let title = '';

    const submit = async () => {
        close();

        await invoke('pack_create', { title });
    };
</script>

<form on:submit|preventDefault={submit}>
    <input type="text" bind:value={title} required />
    <button type="submit">+</button>
</form>
