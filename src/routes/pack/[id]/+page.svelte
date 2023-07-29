<script lang="ts">
    import { goto } from '$app/navigation';
    import { invoke } from '$lib/commands.js';
    import ModalCardPreview from '$lib/components/ModalCardPreview.svelte';
    import { AppBar, modalStore, popup } from '@skeletonlabs/skeleton';

    export let data;

    $: hasCards = data.cards.length > 0;
</script>

<AppBar>
    <svelte:fragment slot="lead">
        <a href="/">
            <i class="fa-solid fa-arrow-left fa-lg" />
        </a>
    </svelte:fragment>

    <h1 class="text-xl font-semibold">
        {data.pack.title}
    </h1>

    <svelte:fragment slot="trail">
        <button
            class="chip variant-filled"
            on:click={() => {
                modalStore.trigger({
                    type: 'prompt',
                    title: 'Add a Card',
                    valueAttr: { type: 'text', placeholder: 'Label', required: true },
                    response: async (label) => {
                        const id = await invoke('card_create', { packId: data.pack.id, label });
                        await goto(`/card/${id}`);
                    }
                });
            }}
        >
            Add a Card
        </button>
    </svelte:fragment>
</AppBar>

<div class="space-y-4 p-4">
    <section class="space-y-4 border border-surface-500 p-4 rounded-container-token">
        <p class="font-semibold">Cards</p>

        {#if hasCards}
            <nav class="list-nav">
                <ul>
                    {#each data.cards as card (card.id)}
                        <li>
                            <button
                                class="w-full"
                                on:click={() => {
                                    modalStore.trigger({
                                        type: 'component',
                                        component: {
                                            ref: ModalCardPreview,
                                            props: { card }
                                        }
                                    });
                                }}
                            >
                                <span class="flex-auto text-left">
                                    {card.label}
                                </span>
                                <button
                                    class="btn-icon btn-icon-sm text-xs"
                                    on:click|stopPropagation={() => {}}
                                    use:popup={{
                                        event: 'click',
                                        target: `popup-${card.id}`,
                                        placement: 'bottom'
                                    }}
                                >
                                    :
                                </button>
                            </button>
                        </li>

                        <div class="card z-10 p-4" data-popup="popup-{card.id}">
                            <div class="arrow bg-surface-100-800-token" />
                            <div class="flex flex-col">
                                <a href="/card/{card.id}" class="btn">Edit</a>
                                <button
                                    class="btn"
                                    on:click={() => {
                                        modalStore.trigger({
                                            type: 'prompt',
                                            title: 'Rename Card',
                                            value: card.label,
                                            valueAttr: {
                                                type: 'text',
                                                placeholder: 'New Label',
                                                required: true
                                            },
                                            response: (newLabel) => {
                                                invoke('card_modify', {
                                                    id: card.id,
                                                    action: {
                                                        Edit: {
                                                            label: newLabel,
                                                            script: null,
                                                            template: null
                                                        }
                                                    }
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
                                            title: 'Delete Card',
                                            body: `Are you sure you want to delete '${card.label}'?`,
                                            response: (doDelete) => {
                                                if (doDelete) {
                                                    invoke('card_modify', {
                                                        id: card.id,
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
        {:else}
            <p>No cards yet, click 'Add a Card.'</p>
        {/if}
    </section>

    {#if hasCards}
        {#if data.cards.length >= 2}
            <a class="btn variant-filled-primary w-full" href="/pack/{data.pack.id}/practice">
                Flash Study
            </a>
            <a class="btn variant-ghost w-full" href="/pack/{data.pack.id}/endless">
                Endless Study
            </a>
        {:else}
            <button class="btn variant-filled-primary w-full" disabled>
                Add More Cards to Begin Practice
            </button>
        {/if}
    {/if}
</div>
