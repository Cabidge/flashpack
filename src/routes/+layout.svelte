<script lang="ts">
    import SmartLink from '@lib/SmartLink.svelte';
    import Transition from '@lib/Transition.svelte';
    import '../app.postcss';
    import type { LayoutData } from './$types';

    export let data: LayoutData;

    const routes = Object.entries({
        Packs: '/pack',
        Study: '/study'
    });

    $: root = data.href.split('/')[1] ?? '';
</script>

<div class="h-screen flex flex-col">
    <nav class="bg-slate-300 flex pl-2 item-stretch gap-2">
        {#each routes as route}
            <SmartLink
                styling={{
                    base: 'px-4 m-2 rounded-full',
                    unselected: 'hover:bg-slate-200',
                    selected: 'bg-white hover:bg-slate-100'
                }}
                href={route[1]}
            >
                {route[0]}
            </SmartLink>
        {/each}
    </nav>

    <Transition class="flex flex-row h-full" key={root}>
        <slot />
    </Transition>
</div>
