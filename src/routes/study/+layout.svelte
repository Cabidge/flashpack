<script lang="ts">
    import { activeTab } from '$lib/routing/tabs';
    import { onMount } from 'svelte';

    onMount(() => ($activeTab = 'study'));
</script>

<slot />
