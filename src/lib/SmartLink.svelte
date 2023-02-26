<script lang="ts">
    import { page } from '$app/stores';

    export let fixedStyle = '';
    export let selectedStyle = 'font-bold';

    export let href: string;

    export let strict = true;

    $: selected = strict ? $page.url.pathname == href : $page.url.pathname.startsWith(href);

    $: classes = `${fixedStyle} ${selected ? selectedStyle : ''}`;
</script>

<a {href} class={classes}>
    <slot />
</a>
