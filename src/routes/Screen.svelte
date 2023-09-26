<script lang="ts">
    export let onUp: undefined | (() => void) = undefined;
    export let onDown: undefined | (() => void) = undefined;
    export let onLeft: undefined | (() => void) = undefined;
    export let onRight: undefined | (() => void) = undefined;
</script>

<div id="screen">
    {#if onUp}
    <div class='nav nav-v nav-up' on:click={onUp} />
    {/if}
    {#if onDown}
    <div class='nav nav-v nav-down' on:click={onDown} />
    {/if}
    {#if onLeft}
    <div class='nav nav-h nav-left' on:click={onLeft} />
    {/if}
    {#if onRight}
    <div class='nav nav-h nav-right' on:click={onRight} />
    {/if}
    <div id="content">
        <slot />
    </div>
</div>

<style>
    #screen {
        position: relative;
        width: 100vw;
        height: 100vh;
    }

    #content {
        position: absolute;
        width: 100vw;
        height: 100vh;
        z-index: 0;
    }

    .nav-up    { background: linear-gradient(to top,    rgb(0, 0, 0, 0), white); }
    .nav-down  { background: linear-gradient(to bottom, rgb(0, 0, 0, 0), white); bottom: 0; }
    .nav-left  { background: linear-gradient(to left,   rgb(0, 0, 0, 0), white); }
    .nav-right { background: linear-gradient(to right,  rgb(0, 0, 0, 0), white); right: 0; }

    .nav {
        position: absolute;
        opacity: 0;
        transition: opacity ease 0.2s;
        z-index: 1;
    }

    .nav:hover {
        opacity: 0.5;
    }

    .nav-v {
        width: 100%;
        height: 2rem;
    }

    .nav-h {
        width: 2rem;
        height: 100%;
    }
</style>