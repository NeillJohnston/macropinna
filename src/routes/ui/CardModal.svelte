<script lang="ts">
	import { nav } from "$lib/joystick";

    // The prefix of nav IDs that use this modal (used to determine whether to render).
    export let idPrefix: string;
    // Set to true if you want to vertically center the contents.
    export let center: boolean = false;
    // Set to true if you need to enable vertical scrolling.
    export let scroll: boolean = false;
    // The card's aspect ratio (sets CSS property aspect-ratio).
    export let aspectRatio = '4/3';

    $: shown = $nav.startsWith(idPrefix);
</script>

{#if shown}
<div id="modal">
    <div
        id="card"
        class:center={center}
        class:scroll={scroll}
        style:aspect-ratio={aspectRatio}
    >
        <slot />
    </div>
</div>
{/if}

<style>
    #modal {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        display: flex;
        justify-content: center;
        padding: var(--xl);
        background-color: rgba(0, 0, 0, 0.5);
        box-sizing: border-box;
        z-index: 10000;
    }

    #card {
        width: auto;
        height: 100%;
        background-color: var(--bg);
        border: 1px solid var(--fg);
    }

    .center {
        display: flex;
        align-items: center;
    }

    .scroll {
        overflow-y: scroll;
    }
</style>