<script lang="ts">
    import { page } from '$app/stores';
    import { cardsContext, createStore } from '$lib/stores/cards';
    import { derived } from 'svelte/store';

    const packId = derived(page, ($page) => parseInt($page.params.id));
    const cards = createStore(packId);

    cardsContext.set(cards);
</script>

<slot />
