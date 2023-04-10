<script lang="ts">
    import { banners } from '$lib/banners';
    import { menuStatus } from '$lib/context_menu';
    import { flip } from 'svelte/animate';
    import '../app.postcss';
    import TabButton from './TabButton.svelte';
    import { fade } from 'svelte/transition';
    import { invoke } from '$lib/commands';

    type TabInfo = {
        label: string;
        href: string;
    };

    $: bannerArray = [...$banners.values()];
</script>

<div class="flex h-screen flex-row">
    <nav class="bg-slate-300">
        <ul class="flex flex-col items-stretch">
            <li class="flex"><TabButton tab="pack" icon="layer-group" href="/pack" /></li>
            <li class="flex"><TabButton tab="study" icon="book" href="/study" /></li>
        </ul>
    </nav>

    <div class="h-full w-full overflow-hidden">
        <slot />
    </div>
</div>

<div class="fixed bottom-0 m-2 flex flex-col gap-2">
    {#each bannerArray as banner (banner)}
        <div
            animate:flip={{ duration: 200 }}
            transition:fade={{ duration: 200 }}
            class="flex flex-row items-start gap-4 rounded bg-red-500 px-4 py-2 text-white shadow"
        >
            <span><i class="fa-solid fa-triangle-exclamation" /></span>
            <div>
                <h2 class="font-semibold">{banner.heading}</h2>
                {#if banner.details !== undefined}
                    <p>{banner.details}</p>
                {/if}
            </div>
            <button on:click={() => banners.remove(banner)}>x</button>
        </div>
    {/each}
</div>

<svelte:body
    on:click={() => {
        menuStatus.close();
    }}
/>
