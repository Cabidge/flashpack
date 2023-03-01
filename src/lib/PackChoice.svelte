<script lang="ts">
    import { goto, invalidateAll } from '$app/navigation';
    import { page } from '$app/stores';
    import type { Pack } from '@bindings/Pack';
    import { deletePack } from './commands';
    import { conditionalClass } from './styling';

    export let pack: Pack;

    let hovering = false;

    $: href = `/pack/${pack.id}`;
    $: selected = $page.url.pathname.startsWith(href);

    const remove = async () => {
        if (selected) {
            await goto('/pack');
        }

        await deletePack(pack.id);

        await invalidateAll();
    };

    $: linkClass = conditionalClass(selected, {
        base: 'relative w-full font-semibold text-ellipsis overflow-hidden py-1 px-4 hover:pr-8 rounded',
        on: 'bg-indigo-500 hover:bg-indigo-600 shadow text-white',
        off: 'hover:bg-slate-200'
    });
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
            class="absolute right-2 top-1 bottom-1 aspect-square rounded-full hover:bg-black hover:bg-opacity-20"
            on:click|preventDefault={remove}
        >
            X
        </button>
    {/if}
</a>
