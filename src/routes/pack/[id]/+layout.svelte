<script lang="ts">
    import { page } from "$app/stores";
    import { banners } from "$lib/banners";
    import { cardsContext, createStore } from "$lib/stores/cards";

    const cards = createStore();
    cardsContext.set(cards);

    $: {
        let id: number;

        try {
            id = parseInt($page.params.id);
        } catch (err) {
            banners.add("Unable to parse pack id", `pack\[id]: ${$page.params.id}`);
            throw err;
        }

        cards.reload(id);
    }
</script>

<slot />
