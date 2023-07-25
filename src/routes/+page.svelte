<script lang="ts">
    import { invoke } from '$lib/commands';
    import { modalStore } from '@skeletonlabs/skeleton';

    export let data;
</script>

<button
    class="btn variant-filled"
    on:click={() => {
        modalStore.trigger({
            type: 'prompt',
            title: 'Create a Pack',
            valueAttr: { type: 'text', placeholder: 'Title', required: true },
            response: (title) => invoke('pack_create', { title })
        });
    }}
>
    Create a Pack
</button>

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
