<script lang="ts">
    import { page } from '$app/stores';
    import type { Pack } from '@bindings/Pack';

    export let pack: Pack;

    $: href = `/pack/${pack.id}`;
    $: selected = $page.url.pathname.startsWith(href);

    $: classes = selected ? 'font-bold' : '';
</script>

<a {href} class={classes}>
    {pack.title}
</a>
