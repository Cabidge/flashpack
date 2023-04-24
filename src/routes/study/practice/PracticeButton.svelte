<script lang="ts">
    export let icon: string | undefined;
    export let label: string | undefined;
</script>

<button class="rounded bg-white py-1 px-2 shadow" on:click>
    {#if icon !== undefined}
        <i class="fa-solid fa-{icon}" />
    {/if}

    {#if label !== undefined}
        {label}
    {/if}
</button>
