<script lang="ts">
    import { fly } from 'svelte/transition';

    export let duration = 100;
    export let key: any;
</script>

{#key key}
    <div in:fly={{ x: -5, duration, delay: duration }} out:fly={{ x: 5, duration }}>
        <slot />
    </div>
{/key}
