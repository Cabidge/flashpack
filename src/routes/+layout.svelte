<script lang="ts">
    import SmartLink from '@lib/SmartLink.svelte';
    import '../app.postcss';

    const routes = Object.entries({
        Packs: '/pack'
    });
</script>

<nav class="bg-slate-200 flex pl-2 item-stretch gap-2">
    {#each routes as route}
        <SmartLink
            fixedStyle="hover:bg-slate-300 h-full p-2"
            selectedStyle="bg-slate-400 hover:bg-slate-400"
            href={route[1]}>{route[0]}</SmartLink
        >
    {/each}
</nav>

<slot />
