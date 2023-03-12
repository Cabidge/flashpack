<script lang="ts">
    import { menuStatus } from './context_menu';

    const id = Symbol();

    export const select = (x: number, y: number) => menuStatus.open({ id, x, y });
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
