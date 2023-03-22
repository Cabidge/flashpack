<script lang="ts">
    import { invalidateAll } from "$app/navigation";
    import { invoke } from "$lib/commands";

    const add = async () => {
        await invoke("create_dealer", { title });
        await invalidateAll();

        title = "";
    }

    let title = "";
</script>

<form on:submit={add}>
    <input bind:value={title} required />
</form>