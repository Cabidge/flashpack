<script lang="ts">
    import AddCard from './AddCard.svelte';
    import CardView from '$lib/CardView.svelte';
    import ModalController from '$lib/ModalController.svelte';
    import RenamePack from './RenamePack.svelte';
    import type { PageData } from './$types';

    export let data: PageData;

    $: pack = data.pack;
    $: cards = data.cards;
</script>

<ModalController let:open let:close>
    <h1>{pack.title} <button on:click={open}>rename</button></h1>

    <RenamePack slot="modal" {close} {pack} />
</ModalController>

<ModalController let:open let:close>
    <h2>Cards ({cards.length}) <button on:click={open}>+</button></h2>

    <AddCard slot="modal" id={pack.id} {close} />
</ModalController>

<ul>
    {#each cards as card (card.id)}
        <li>
            <ModalController let:open>
                <button on:click={open}>{card.front}</button>
                <CardView slot="modal" {card} />
            </ModalController>
        </li>
    {/each}
</ul>
