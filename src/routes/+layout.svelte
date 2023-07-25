<script lang="ts">
    // Your selected Skeleton theme:
    import '@skeletonlabs/skeleton/themes/theme-skeleton.css';

    // This contains the bulk of Skeletons required styles:
    import '@skeletonlabs/skeleton/styles/skeleton.css';

    // Finally, your application's global stylesheet (sometimes labeled 'app.css')
    import '../app.postcss';

    import { listen } from '@tauri-apps/api/event';
    import { invalidate } from '$app/navigation';
    import { onMount } from 'svelte';

    onMount(async () => await listen('update:packs', () => invalidate('flashpack:packs')));
    onMount(async () => await listen('update:cards', () => invalidate('flashpack:cards')));
</script>

<slot />
