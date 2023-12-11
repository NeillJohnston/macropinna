<script lang="ts">
	import type { Launcher } from "$lib/api";
    import { convertFileSrc } from "@tauri-apps/api/tauri";

    export let launcher: Launcher;
    export let selected: boolean;
    export let size: string;
    export let margin: string;

    const url = launcher.image_path && convertFileSrc(launcher.image_path);
    const cssBackground = launcher.css_background ?? '';
    const bordered = !url && !cssBackground;
</script>

<div
    id="app"
    class:selected={selected}
    style:width={size}
    style:height={size}
    style:margin-right={margin}
>
    <div
        id="inner"
        class:bordered={bordered}
        style:background={cssBackground}
    >
        {#if url}
        <img id="cover" src={url} alt="Launcher cover" />
        {:else}
        {launcher.name}
        {/if}
    </div>
</div>

<style>
    #app {
        box-sizing: border-box;
        padding: 80px;
        opacity: 50%;
        font-size: 1.00rem;
        font-weight: bold;

        transition:
            padding ease 0.4s,
            opacity ease 0.4s,
            font-size ease 0.4s;
    }

    #app.selected {
        padding: 0px;
        opacity: 100%;
        font-size: 2.00rem;
    }

    #inner {
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        text-align: center;
    }
    
    .bordered {
        border: 1px solid var(--fg);
    }

    #cover {
        object-fit: cover;
        width: 100%;
        height: 100%;
    }
</style>