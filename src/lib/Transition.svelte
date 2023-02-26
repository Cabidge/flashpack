<script lang="ts">
    import { fly } from 'svelte/transition';

    export let duration = 120;
    export let key: any;
</script>

{#key key}
    <div in:fly={{ x: -10, duration, delay: duration }} out:fly={{ duration }}>
        <slot />
    </div>
{/key}
