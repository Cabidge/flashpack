<script lang="ts">
    import { invoke } from '$lib/commands';
    import type { FilterSummary } from '@bindings/FilterSummary';
    import type { GroupedWeightedFilters } from '@bindings/GroupedWeightedFilters';
    import type { ModifyDealer } from '@bindings/ModifyDealer';
    import { createEventDispatcher, onMount } from 'svelte';

    export let dealerFilters: GroupedWeightedFilters;

    $: oldFilterIds = new Set(Object.values(dealerFilters).flatMap((group) => group).map((filter) => filter.summary.id));

    let filters: Record<string, FilterSummary[]> = {};

    onMount(async () => {
        filters = await invoke('list_filters');
    });

    // TODO: recreate filter selection

    const dispatch = createEventDispatcher<{ save: ModifyDealer[] }>();
</script>

<p>placeholder</p>
