<script lang="ts">
    import { page } from '$app/stores';

    export let fixedStyle = '';
    export let selectedStyle = 'font-bold';

    export let href: string;

    $: selected = $page.url.pathname.startsWith(href);

    $: classes = `${fixedStyle} ${selected ? selectedStyle : ''}`;
</script>

<a {href} class={classes}>
    <slot />
</a>
