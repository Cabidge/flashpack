<script lang="ts">
    import { page } from '$app/stores';

    export let href: string;
    export let strict = true;

    $: selected = strict ? $page.url.pathname == href : $page.url.pathname.startsWith(href);
</script>

<slot {selected}></slot>
