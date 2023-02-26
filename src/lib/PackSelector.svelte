<script lang="ts">
    import type { Pack } from '@bindings/Pack';
    import { flip } from 'svelte/animate';
    import { fade } from 'svelte/transition';
    import SmartLink from './SmartLink.svelte';

    export let packs: Pack[];
</script>

<ul class="flex flex-col gap-2 items-stretch m-2">
    {#each packs as pack (pack.id)}
        <li
            transition:fade={{ duration: 150 }}
            animate:flip={{ duration: 200 }}
            class="flex items-stretch"
        >
            <SmartLink
                href="/pack/{pack.id}"
                strict={false}
                fixedStyle="w-full shadow text-center bg-slate-100 hover:bg-slate-200 font-semibold py-1 px-2 rounded"
                selectedStyle="bg-blue-500 hover:bg-blue-600 text-white"
            >
                {pack.title}
            </SmartLink>
        </li>
    {/each}
</ul>
