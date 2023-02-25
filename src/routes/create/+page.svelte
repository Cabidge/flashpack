<script lang="ts">
    import { goto, invalidateAll } from '$app/navigation';
    import { invoke } from '@tauri-apps/api';
    import type { PackCreate } from '@bindings/PackCreate';

    let title = '';

    const submit = async () => {
        if (title == '') {
            // TODO: display error
            return;
        }

        const pack: PackCreate = { title };
        await invoke('create_pack', { pack });

        await invalidateAll();

        await goto('/');
    };
</script>

<form on:submit|preventDefault={submit}>
    <input bind:value={title} />
    <button type="submit">Create</button>
</form>
