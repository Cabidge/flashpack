<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import { invoke } from '$lib/commands';
    import ModalController from '$lib/ModalController.svelte';
    import type { PageData } from './$types';
    import EditFilters from './EditFilters.svelte';

    export let data: PageData;

    $: ({ id, dealer } = data);
</script>

<h1>{dealer.title}</h1>

<ModalController title="Select Filters" let:open let:close>
    <button on:click={open}>Edit Filters</button>

    <EditFilters
        on:save={async (e) => {
            close();

            for (const action of e.detail) {
                await invoke('modify_dealer', { id, action });
            }

            await invalidateAll();
        }}
        dealerFilters={dealer.filters}
        slot="modal"
    />
</ModalController>

<ul>
    {#each dealer.filters as filter (filter.id)}
        <li>
            {filter.pack_title}::{filter.label}
        </li>
    {/each}
</ul>
