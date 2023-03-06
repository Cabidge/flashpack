<script lang="ts">
    import AddCard from './AddCard.svelte';
    import CardView from '$lib/CardView.svelte';
    import ModalController from '$lib/ModalController.svelte';
    import type { PageData } from './$types';

    export let data: PageData;

    $: pack = data.pack;
    $: cards = data.cards;
</script>

<h1 class="text-2xl font-semibold">
    {pack.title}
</h1>

<br />

<ModalController let:open let:close>
    <h2 class="text-lg">
        Cards
        <button on:click={open}>+</button>
    </h2>

    <AddCard slot="modal" id={pack.id} {close} />
</ModalController>

<div class="mb-2 border-b-2" />

{#if cards.length === 0}
    <p>No cards found...</p>
{:else}
    <div
        class="grid w-full grid-cols-2 gap-4 rounded bg-slate-100 p-4 shadow-inner md:grid-cols-3 lg:grid-cols-4"
    >
        {#each cards as card (card.id)}
            <ModalController let:open>
                <button
                    class="h-32 rounded border-2 border-transparent bg-white text-lg shadow transition hover:-translate-y-1 hover:border-indigo-400 hover:shadow-md"
                    on:click={open}
                >
                    {card.front}
                    <br />
                    {#each ['test', 'hello', 'definition'] as tag}
                        <span class="m-1 rounded-full bg-slate-100 hover:bg-slate-200 px-2 text-sm">{tag}</span>
                    {/each}
                </button>

                <CardView slot="modal" {card} />
            </ModalController>
        {/each}
    </div>
{/if}
