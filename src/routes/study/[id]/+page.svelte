<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import { invoke } from '$lib/commands';
    import ModalController from '$lib/ModalController.svelte';
    import type { PageData } from './$types';
    import EditFilters from './EditFilters.svelte';
    import EditWeight from './EditWeight.svelte';

    export let data: PageData;

    $: ({ id, dealer } = data);

    $: showWeight = dealer.filters.reduce((acc, filter) => acc || filter.weight !== 1, false);
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
            <ModalController title="Edit Weight" let:open let:close>
                {filter.pack_title}::{filter.label}
                {#if showWeight}
                    ({filter.weight})
                {/if}
                <button on:click={open}>edit weight</button>

                <EditWeight
                    slot="modal"
                    weight={filter.weight}
                    on:save={async (e) => {
                        close();
                        await invoke('modify_dealer', {
                            id,
                            action: { SetWeight: [filter.id, e.detail] }
                        });
                        await invalidateAll();
                    }}
                />
            </ModalController>
        </li>
    {/each}
</ul>
