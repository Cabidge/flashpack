<script lang="ts">
    import UrlMatch from './UrlMatch.svelte';

    export let styling = {
        base: '',
        selected: 'font-bold',
        unselected: ''
    };

    export let href: string;
    export let strict = true;
</script>

<UrlMatch {href} {strict} let:selected>
    <a {href} class="{styling.base} {selected ? styling.selected : styling.unselected}">
        <slot />
    </a>
</UrlMatch>
