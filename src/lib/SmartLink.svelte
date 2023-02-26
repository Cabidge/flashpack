<script lang="ts">
    import { page } from '$app/stores';

    export let fixedStyle = '';
    export let disabledStyle = 'text-gray-400';
    export let selectedStyle = 'font-bold';

    export let href: string;

    export let strict = true;

    export let disabled = false;

    $: link = disabled ? 'javascript:void(0)' : href;

    $: selected = strict ? $page.url.pathname == href : $page.url.pathname.startsWith(href);

    $: classes = `${fixedStyle} ${selected ? selectedStyle : ''} ${disabled ? disabledStyle : ''}`;
</script>

<a href={link} class={classes}>
    <slot />
</a>
