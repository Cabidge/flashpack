<script lang="ts">
    import { menuStatus } from './context_menu';

    const id = Symbol();

    export const select = (x: number, y: number) => menuStatus.open({ id, x, y });

    let menuEl: HTMLDivElement;

    $: selected = $menuStatus?.id === id;

    $: x = $menuStatus?.x ?? 0;
    $: y = $menuStatus?.y ?? 0;
</script>

{#if selected}
    <div
        bind:this={menuEl}
        class="absolute rounded bg-white p-1 shadow"
        style:top="{y}px"
        style:left="{x}px"
    >
        <slot />
    </div>
{/if}
