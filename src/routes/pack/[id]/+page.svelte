<script lang="ts">
    import AddCard from './AddCard.svelte';
    import ModalController from '$lib/ModalController.svelte';
    import type { PageData } from './$types';
    import CardButton from './CardButton.svelte';
    import { goto } from '$app/navigation';
    import AddFilter from './AddFilter.svelte';

    export let data: PageData;

    $: ({ id, pack } = data);
    $: ({ cards, filters } = pack);
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
    <div class="card-grid grid w-full gap-4 rounded bg-slate-100 p-4 shadow-inner">
        {#each cards as card (card.id)}
            <CardButton label={card.label} on:click={() => {}} on:edit={() => goto(`/pack/${id}/card/${card.id}`)} />
        {/each}
    </div>
{/if}

<br />

<ModalController title="Add a Filter" let:open let:close>
    <h2 class="text-lg">
        Filters
        <button on:click={open}>+</button>
    </h2>

    <AddFilter slot="modal" {id} {close} />
</ModalController>

<div class="mb-2 border-b-2" />

{#if filters.length === 0}
    <p>No filters created...</p>
{:else}
    <ul>
        {#each filters as filter (filter.id)}
            <li>
                <a class="hover:underline" href="/pack/{id}/filter/{filter.id}">
                    {filter.label}
                </a>
            </li>
        {/each}
    </ul>
{/if}

<style>
    .card-grid {
        grid-template-columns: repeat(auto-fit, minmax(8rem, 1fr));
    }
</style>
