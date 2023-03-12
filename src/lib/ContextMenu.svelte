<script lang="ts">
    import type { MouseEventHandler } from 'svelte/elements';
    import { menuStatus } from './context_menu';

    const id = Symbol();

    export const openAt = (x: number, y: number) => menuStatus.open({ id, x, y });
    export const onContextMenu: MouseEventHandler<HTMLElement> = (e) => openAt(e.clientX, e.clientY);
</script>

{#if $menuStatus !== null && $menuStatus.id === id}
    <div
        class="fixed rounded bg-white p-1 shadow"
        style:top="{$menuStatus.y}px"
        style:left="{$menuStatus.x}px"
    >
        <slot />
    </div>
{/if}
