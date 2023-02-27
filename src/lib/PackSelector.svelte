<script lang="ts">
    import type { Pack } from '@bindings/Pack';
    import { flip } from 'svelte/animate';
    import { fly } from 'svelte/transition';
    import SmartLink from './SmartLink.svelte';

    export let packs: Pack[];
</script>

<ul class="flex flex-col gap-3 items-stretch">
    {#each packs as pack (pack.id)}
        <li
            in:fly={{ x: -10, duration: 150 }}
            out:fly={{ y: 10, duration: 150 }}
            animate:flip={{ duration: 200 }}
            class="flex items-stretch"
        >
            <SmartLink
                href="/pack/{pack.id}"
                strict={false}
                styling={{
                    base: 'w-full shadow text-center font-semibold py-1 px-2 rounded',
                    unselected: 'bg-slate-100 hover:bg-slate-200',
                    selected: 'bg-indigo-500 hover:bg-indigo-600 text-white',
                }}
            >
                {pack.title}
            </SmartLink>
        </li>
    {/each}
</ul>
