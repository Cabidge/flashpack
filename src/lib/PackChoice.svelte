<script lang="ts">
    import { goto, invalidateAll } from '$app/navigation';
    import type { Pack } from '@bindings/Pack';
    import { deletePack } from './commands';
    import { conditionalClass, type ConditionalClass } from './styling';
    import UrlMatch from './UrlMatch.svelte';

    export let pack: Pack;

    let hovering = false;

    $: href = `/pack/${pack.id}`;

    const remove = async () => {
        await deletePack(pack.id);
        await goto('/pack');

        await invalidateAll();
    };

    const linkClass: ConditionalClass = {
        base: 'relative w-full font-semibold py-1 px-4 rounded',
        on: 'bg-indigo-500 hover:bg-indigo-600 shadow text-white',
        off: 'hover:bg-slate-200'
    };
</script>

<UrlMatch {href} strict={false} let:selected>
    <a
        {href}
        class={conditionalClass(selected, linkClass)}
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
</UrlMatch>
