<script lang="ts">
    import { menuStatus } from '$lib/context_menu';
    import SmartLink from '$lib/SmartLink.svelte';
    import Transition from '$lib/Transition.svelte';
    import '../app.postcss';
    import type { LayoutData } from './$types';

    export let data: LayoutData;

    const routes = Object.entries({
        Packs: '/pack',
        Study: '/study'
    });

    $: root = data.href.split('/')[1] ?? '';
</script>

<div class="flex h-screen flex-col">
    <nav class="item-stretch flex gap-1 bg-slate-300 py-3 pl-3">
        {#each routes as route}
            <SmartLink
                styling={{
                    base: 'px-4 rounded-full',
                    unselected: 'hover:bg-slate-200',
                    selected: 'bg-white hover:bg-slate-100'
                }}
                href={route[1]}
                strict={false}
            >
                {route[0]}
            </SmartLink>
        {/each}
    </nav>

    <Transition class="h-full w-full overflow-hidden" key={root}>
        <slot />
    </Transition>
</div>

<svelte:body
    on:click={() => {
        menuStatus.close();
    }}
/>
