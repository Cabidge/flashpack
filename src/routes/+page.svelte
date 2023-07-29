<script lang="ts">
    import { goto } from '$app/navigation';
    import { invoke } from '$lib/commands';
    import { AppBar, modalStore, popup } from '@skeletonlabs/skeleton';

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

                    <div class="card p-4" data-popup="popup-{pack.id}">
                        <div class="arrow bg-surface-100-800-token" />
                        <div class="flex flex-col">
                            <button
                                class="btn"
                                on:click={() => {
                                    modalStore.trigger({
                                        type: 'prompt',
                                        title: 'Rename Pack',
                                        value: pack.title,
                                        valueAttr: {
                                            type: 'text',
                                            placeholder: 'New Title',
                                            required: true
                                        },
                                        response: (newTitle) => {
                                            invoke('pack_modify', {
                                                id: pack.id,
                                                action: { Rename: newTitle }
                                            });
                                        }
                                    });
                                }}
                            >
                                Rename
                            </button>
                            <button
                                class="btn"
                                on:click={() => {
                                    modalStore.trigger({
                                        type: 'confirm',
                                        title: 'Delete Pack',
                                        body: `Are you sure you want to delete '${pack.title}'?`,
                                        response: (doDelete) => {
                                            if (doDelete) {
                                                invoke('pack_modify', {
                                                    id: pack.id,
                                                    action: 'Delete'
                                                });
                                            }
                                        }
                                    });
                                }}
                            >
                                Delete
                            </button>
                        </div>
                    </div>
                {/each}
            </ul>
        </nav>
    {/if}
</div>
