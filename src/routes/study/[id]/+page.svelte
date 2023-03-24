<script lang="ts">
    import { invalidateAll } from '$app/navigation';
    import { invoke } from '$lib/commands';
    import ModalController from '$lib/ModalController.svelte';
    import type { PageData } from './$types';
    import EditFilters from './EditFilters.svelte';
    import EditWeight from './EditWeight.svelte';

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
    {#each Object.entries(dealer.filters) as [packTitle, filterGroup] (packTitle)}
        <li>
            <span class="w-full bg-slate-300">
                {packTitle}
            </span>
            <ul>
                {#each filterGroup as filter (filter.summary.id)}
                    <li>
                        <ModalController title="Edit Weight" let:open let:close>
                            <button on:click={open} class="group">
                                {filter.summary.label}
                                <span class="text-xs text-indigo-600 {filter.weight === 1 ? "hidden group-hover:inline" : ""}">
                                    (x{filter.weight})
                                </span>
                            </button>

                            <EditWeight
                                slot="modal"
                                weight={filter.weight}
                                on:save={async (e) => {
                                    close();
                                    await invoke('modify_dealer', {
                                        id,
                                        action: { SetWeight: [filter.summary.id, e.detail] }
                                    });
                                    await invalidateAll();
                                }}
                            />
                        </ModalController>
                    </li>
                {/each}
            </ul>
        </li>
    {/each}
</ul>
