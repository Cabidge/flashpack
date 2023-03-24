<script lang="ts">
    import type { PageData } from './$types';
    import CreateDealer from './CreateDealer.svelte';

    export let data: PageData;

    $: dealers = data.dealers;
</script>

<CreateDealer />

{#if dealers.length === 0}
    <p>Add a dealer...</p>
{:else}
    <ul>
        {#each dealers as dealer (dealer.id)}
            <li>
                <a href="/study/{dealer.id}">{dealer.title}</a>
            </li>
        {/each}
    </ul>
{/if}
