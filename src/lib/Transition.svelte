<script lang="ts">
    import { fly } from 'svelte/transition';

    export let duration = 120;
    export let key: any;

    let clazz: string = '';
    export { clazz as class };
</script>

{#key key}
    <div in:fly={{ x: -10, duration, delay: duration + 10 }} out:fly={{ duration }} class={clazz}>
        <slot />
    </div>
{/key}
