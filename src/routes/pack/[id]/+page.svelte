<script lang="ts">
    import AddCard from './AddCard.svelte';
    import CardButton from './CardButton.svelte';
    import { goto } from '$app/navigation';
    import { modals } from '$lib/modals';
    import { packs } from '$lib/stores/packs';
    import { page } from '$app/stores';
    import { cardsContext } from '$lib/stores/cards';
    import { derived } from 'svelte/store';

    const id = derived(page, ($page) => parseInt($page.params.id));
    const pack = packs.get(id);

    $: packHref = `/pack/${id}`;

    const cards = cardsContext.get();
</script>

<h1 class="text-2xl font-semibold">
    {$pack.title}
</h1>

<br />

<h2 class="text-lg">
    Cards
    <button on:click={() => modals.add(AddCard, { id: $id })}>+</button>
</h2>

<div class="mb-2 border-b-2" />

{#if $cards.length === 0}
    <p>No cards found...</p>
{:else}
    <div class="card-grid grid w-full gap-4 rounded bg-slate-100 p-4 shadow-inner">
        {#each $cards as card (card.id)}
            <CardButton
                label={card.label}
                on:click={() => goto(`${packHref}/card/${card.id}/preview`)}
                on:edit={() => goto(`${packHref}/card/${card.id}`)}
            />
        {/each}
    </div>
{/if}

<style>
    .card-grid {
        grid-template-columns: repeat(auto-fit, minmax(8rem, 1fr));
    }
</style>
