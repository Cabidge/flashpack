<script lang="ts">
    import type { PageData } from './$types';

    export let data: PageData;

    $: data?.activePack.set(null);
</script>

<p>Select a pack...</p>
