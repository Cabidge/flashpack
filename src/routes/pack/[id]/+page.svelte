<script lang="ts">
    import { goto } from '$app/navigation';
    import { invoke } from '$lib/commands.js';
    import CardPreview from '$lib/components/CardPreview.svelte';
    import { AppBar, modalStore } from '@skeletonlabs/skeleton';

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
                                            ref: CardPreview,
                                            props: { card }
                                        }
                                    });
                                }}
                            >
                                {card.label}
                            </button>
                        </li>
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
                Begin Practice
            </a>
        {:else}
            <button class="btn variant-filled-primary w-full" disabled>
                Add More Cards to Begin Practice
            </button>
        {/if}
    {/if}
</div>
