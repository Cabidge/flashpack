<script lang="ts">
    import type { Pack } from '@bindings/Pack';

    export let pack: Pack;
    export let selectedId: undefined | string;

    $: classes = selectedId === pack.id ? 'font-bold' : '';
</script>

<a href="/pack/{pack.id}" class={classes}>
    {pack.title}
</a>
