<script lang="ts">
    import LeftArrow from '~icons/mdi/arrow-left';

    export let href: string | undefined | null = undefined;
</script>

<a class="btn-icon btn-icon-sm hover:variant-soft" {href}>
    <LeftArrow class="text-lg" />
</a>
