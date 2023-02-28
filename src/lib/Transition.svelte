<script lang="ts">
    import { fly } from 'svelte/transition';

    export let duration = 120;
    export let key: unknown;

    let clazz = '';
    export { clazz as class };
</script>

{#key key}
    <div in:fly={{ x: -10, duration, delay: duration + 50 }} out:fly={{ duration }} class={clazz}>
        <slot />
    </div>
{/key}
