<script lang="ts">
    import type { Pack } from '@bindings/Pack';
    import { flip } from 'svelte/animate';
    import { fly } from 'svelte/transition';
    import SmartLink from './SmartLink.svelte';

    export let packs: Pack[];
</script>

{#if packs.length == 0}
    <p>No packs found...</p>
{:else}
    <ul class="flex flex-col items-stretch">
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
                        base: 'w-full font-semibold py-1 px-4 rounded',
                        unselected: 'hover:bg-slate-200',
                        selected: 'bg-indigo-500 hover:bg-indigo-600 shadow text-white'
                    }}
                >
                    {pack.title}
                </SmartLink>
            </li>
        {/each}
    </ul>
{/if}
