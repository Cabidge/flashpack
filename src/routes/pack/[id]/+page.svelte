<script lang="ts">
    import { page } from '$app/stores';
    import AddCard from '@lib/AddCard.svelte';
    import CardView from '@lib/CardView.svelte';
    import ModalContext from '@lib/ModalContext.svelte';
    import RenamePack from '@lib/RenamePack.svelte';
    import type { PageData } from './$types';

    export let data: PageData;

    $: pack = data.pack;
    $: cards = data.cards;
</script>

<ModalContext let:open let:close>
    <h1>{pack.title} <button on:click={open}>rename</button></h1>

    <RenamePack slot="modal" on:close={close} {pack} />
</ModalContext>

<ModalContext let:open let:close>
    <h2>Cards ({cards.length}) <button on:click={open}>+</button></h2>

    <AddCard slot="modal" id={pack.id} on:close={close} />
</ModalContext>

<ul>
    {#each cards as card (card.id)}
        <li>
            <ModalContext let:open>
                <button on:click={open}>{card.front}</button>
                <CardView slot="modal" {card} />
            </ModalContext>
        </li>
    {/each}
</ul>
