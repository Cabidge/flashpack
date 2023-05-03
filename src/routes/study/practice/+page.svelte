<script lang="ts">
    import { page } from '$app/stores';
    import { invoke } from '$lib/commands';
    import PracticePage from './PracticePage.svelte';

    const executeQuery = (query: URLSearchParams) => {
        const maybeParseInt = (n: string | null) => {
            return n === null ? undefined : parseInt(n);
        };

        const packId = maybeParseInt(query.get('pack'));

        const include = query.getAll('include[]');
        const exclude = query.getAll('exclude[]');

        // first convert to number definite number, then change to undefined for zero case
        // TODO: change query_cards to only accept a number, then handle the zero case in rust
        let limit: number | undefined = maybeParseInt(query.get('limit')) ?? 0;
        limit = Math.max(limit, 0);
        if (limit === 0) {
            limit = undefined;
        }

        return invoke('card_query', { packId, include, exclude, limit });
    };

    $: questions = executeQuery($page.url.searchParams);
</script>

{#await questions}
    <p>Loading questions...</p>
{:then questions}
    <PracticePage {questions} />
{:catch err}
    <p>Error: {err}</p>
{/await}
