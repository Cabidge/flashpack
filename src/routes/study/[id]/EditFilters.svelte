<script lang="ts">
    import { invoke } from '$lib/commands';
    import type { FilterSummary } from '@bindings/FilterSummary';
    import type { GroupedWeightedFilters } from '@bindings/GroupedWeightedFilters';
    import type { ModifyDealer } from '@bindings/ModifyDealer';
    import { createEventDispatcher, onMount } from 'svelte';

    export let dealerFilters: GroupedWeightedFilters;

    $: oldFilterIds = new Set(Object.values(dealerFilters).flat().map((filter) => filter.summary.id));

    type FilterOption = FilterSummary & {
        selected: boolean,
        newSelected: boolean,
    };

    let filters: Record<string, FilterOption[]> = {};
    $: filterEntries = Object.entries(filters);

    onMount(async () => {
        const summaries = await invoke('list_filters');

        for (const [packTitle, filterGroup] of Object.entries(summaries)) {
            filters[packTitle] = filterGroup.map((filter) => {
                const selected = oldFilterIds.has(filter.id);
                return { ...filter, selected, newSelected: selected };
            })
        }
    });

    const dispatch = createEventDispatcher<{ save: ModifyDealer[] }>();

    const save = () => {
        const modifications: ModifyDealer[] = [];

        for (const { id, selected, newSelected } of Object.values(filters).flat()) {
            if (selected != newSelected) {
                modifications.push(newSelected ? { AddFilter: id } : { RemoveFilter: id });
            }
        }

        dispatch("save", modifications);
    }
</script>

{#if filterEntries.length === 0}
    <p>No pack filters found...</p>
{:else}
    <ul class="max-h-[60vh] w-[70vw] max-w-lg overflow-y-auto">
        {#each filterEntries as [packTitle, filterGroup] (packTitle)}
            <li>
                <span class="font-semibold">{packTitle}</span>
                <ul>
                    {#each filterGroup as filter (filter.id)}
                        <li>
                            <input id="filter-{filter.id}" type="checkbox" bind:checked={filter.newSelected}>
                            <label for="filter-{filter.id}">{filter.label}</label>
                        </li>
                    {/each}
                </ul>
            </li>
        {/each}
    </ul>

    <button on:click={save}>save</button>
{/if}
