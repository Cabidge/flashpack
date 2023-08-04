<script lang="ts">
    // Your selected Skeleton theme:
    import '../theme.postcss';

    // This contains the bulk of Skeletons required styles:
    import '@skeletonlabs/skeleton/styles/skeleton.css';

    // Finally, your application's global stylesheet (sometimes labeled 'app.css')
    import '../katex.css';
    import '../app.postcss';

    import { listen } from '@tauri-apps/api/event';
    import { invalidate } from '$app/navigation';
    import { onMount } from 'svelte';
    import { Modal } from '@skeletonlabs/skeleton';

    import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
    import { storePopup } from '@skeletonlabs/skeleton';

    storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

    onMount(async () => await listen('update:packs', () => invalidate('flashpack:packs')));
    onMount(async () => await listen('update:cards', () => invalidate('flashpack:cards')));
</script>

<Modal />

<slot />
