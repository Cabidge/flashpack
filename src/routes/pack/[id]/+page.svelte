<script lang="ts">
    export let data;
</script>

<a href="/">Return to Packs</a>

<h1>{data.pack.title}</h1>

<nav class="list-nav">
    <ul>
        {#each data.cards as card (card.id)}
            <li>
                <a href="/card/{card.id}">
                    {card.label}
                </a>
            </li>
        {/each}
    </ul>
</nav>
