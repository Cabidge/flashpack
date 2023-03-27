<script lang="ts">
    type Icon = 'book' | 'trash' | 'pen';

    export let icon: Icon | undefined = undefined;
    export let label: string;

    export let danger = false;

    function getIconClass(icon: Icon): string {
        switch (icon) {
            case 'book':
                return 'fa-solid fa-book';
            case 'trash':
                return 'fa-solid fa-trash';
            case 'pen':
                return 'fa-solid fa-pen';
        }
    }

    $: iconClass = icon && getIconClass(icon);
</script>

<button
    on:click
    class="flex w-full items-center gap-4 rounded px-1 font-semibold hover:bg-slate-100
        {danger ? 'text-red-600' : ''}"
>
    <span class="flex-grow text-start">
        {label}
    </span>

    {#if iconClass}
        <i class="flex-none text-sm {iconClass}" />
    {/if}
</button>
