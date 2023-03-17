<script lang="ts">
    import AddCard from './AddCard.svelte';
    import ModalController from '$lib/ModalController.svelte';
    import type { PageData } from './$types';
    import CardButton from './CardButton.svelte';

    export let data: PageData;

    $: id = data.id;
    $: pack = data.pack;
    $: cards = pack.cards;
</script>

<h1 class="text-2xl font-semibold">
    {pack.title}
</h1>

<br />

<ModalController title="Add a Card" let:open let:close>
    <h2 class="text-lg">
        Cards
        <button on:click={open}>+</button>
    </h2>

    <AddCard slot="modal" {id} {close} />
</ModalController>

<div class="mb-2 border-b-2" />

{#if cards.length === 0}
    <p>No cards found...</p>
{:else}
    <div
        class="grid w-full grid-cols-2 gap-4 rounded bg-slate-100 p-4 shadow-inner md:grid-cols-3 lg:grid-cols-4"
    >
        {#each cards as card (card.id)}
            <CardButton {card} />
        {/each}
    </div>
{/if}
