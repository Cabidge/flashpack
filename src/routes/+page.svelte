<script lang="ts">
    export let data;
</script>

<nav class="list-nav">
    <ul>
        {#each data.packs as pack (pack.id)}
            <li>
                <a href="/pack/{pack.id}">
                    {pack.title}
                </a>
            </li>
        {/each}
    </ul>
</nav>
