<script lang="ts">
    import { goto } from '$app/navigation';
    import { invoke } from '$lib/commands';
    import { AppBar, modalStore } from '@skeletonlabs/skeleton';

    export let data;
</script>

<AppBar>
    <h1 class="text-xl font-semibold">Flashpack</h1>

    <svelte:fragment slot="trail">
        <button
            class="chip variant-filled"
            on:click={() => {
                modalStore.trigger({
                    type: 'prompt',
                    title: 'Create a Pack',
                    valueAttr: { type: 'text', placeholder: 'Title', required: true },
                    response: async (title) => {
                        const id = await invoke('pack_create', { title });
                        await goto(`/pack/${id}`);
                    }
                });
            }}
        >
            Create a Pack
        </button>
    </svelte:fragment>
</AppBar>

<div class="p-4">
    <nav class="list-nav">
        <ul>
            {#each data.packs as pack (pack.id)}
                <li>
                    <a href="/pack/{pack.id}">
                        {pack.title}
                    </a>
                </li>
            {/each}
        </ul>
    </nav>
</div>
