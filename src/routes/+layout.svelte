<script lang="ts">
    import SmartLink from '@lib/SmartLink.svelte';
    import '../app.postcss';

    const routes = Object.entries({
        Packs: '/pack'
    });
</script>

<div class="h-screen flex flex-col">
    <nav class="bg-slate-300 flex pl-2 item-stretch gap-2">
        {#each routes as route}
            <SmartLink
                styling={{
                    base: "px-4 m-2 rounded-full",
                    unselected: "hover:bg-slate-200",
                    selected: "bg-white hover:bg-slate-100",
                }}
                href={route[1]}
            >
                {route[0]}
            </SmartLink>
        {/each}
    </nav>

    <slot />
</div>
