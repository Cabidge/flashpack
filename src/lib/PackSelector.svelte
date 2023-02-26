<script lang="ts">
    import type { Pack } from '@bindings/Pack';
    import { flip } from 'svelte/animate';
    import { fade } from 'svelte/transition';
    import SmartLink from './SmartLink.svelte';

    export let packs: Pack[];
    export let disabled = false;
</script>

<ul class="flex flex-col gap-2 items-stretch m-2">
    {#each packs as pack (pack.id)}
        <li
            transition:fade={{ duration: 150 }}
            animate:flip={{ duration: 200 }}
            class="flex items-stretch"
        >
            <SmartLink
                fixedStyle="w-full shadow text-center bg-gray-100 hover:bg-gray-200 font-semibold py-1 px-2 rounded"
                selectedStyle="bg-gray-300 hover:bg-gray-300"
                href="/pack/{pack.id}"
                {disabled}>{pack.title}</SmartLink
            >
        </li>
    {/each}
</ul>
