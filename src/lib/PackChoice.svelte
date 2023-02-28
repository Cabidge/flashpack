<script lang="ts">
    import { goto, invalidateAll } from '$app/navigation';
    import { page } from '$app/stores';
    import type { Pack } from '@bindings/Pack';
    import { deletePack } from './commands';
    import { conditionalClass, type ConditionalClass } from './styling';

    export let pack: Pack;

    let hovering = false;

    const remove = async () => {
        await deletePack(pack.id);
        await goto('/pack');

        await invalidateAll();
    };

    $: href = `/pack/${pack.id}`
    $: selected = $page.url.pathname.startsWith(href);

    $: linkClass = conditionalClass(selected, {
        base: 'relative w-full font-semibold py-1 px-4 rounded',
        on: 'bg-indigo-500 hover:bg-indigo-600 shadow text-white',
        off: 'hover:bg-slate-200',
    })
</script>

<a 
    {href}
    class={linkClass}
    on:mouseenter={() => (hovering = true)}
    on:mouseleave={() => (hovering = false)}
>
    {pack.title}
    {#if hovering}
        <button
            class="absolute right-2 top-1 bottom-1 hover:bg-black hover:bg-opacity-20 aspect-square rounded-full"
            on:click|preventDefault={remove}
        >
            X
        </button>
    {/if}
</a>
