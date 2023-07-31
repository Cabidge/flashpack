<script lang="ts">
    import { goto } from '$app/navigation';
    import { invoke } from '$lib/commands';
    import { AppBar, modalStore, popup } from '@skeletonlabs/skeleton';
    import PackContextMenu from './PackContextMenu.svelte';

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
    {#if data.packs.length === 0}
        <p>No packs yet, click 'Create a Pack.'</p>
    {:else}
        <nav class="list-nav">
            <ul>
                {#each data.packs as pack (pack.id)}
                    <li>
                        <a href="/pack/{pack.id}">
                            <span class="flex-auto">
                                {pack.title}
                            </span>
                            <button
                                class="btn-icon btn-icon-sm text-xs"
                                on:click|preventDefault={() => {}}
                                use:popup={{
                                    event: 'click',
                                    target: `popup-${pack.id}`,
                                    placement: 'bottom'
                                }}
                            >
                                :
                            </button>
                        </a>
                    </li>
                    <PackContextMenu {pack} popup="popup-{pack.id}" />
                {/each}
            </ul>
        </nav>
    {/if}
</div>
