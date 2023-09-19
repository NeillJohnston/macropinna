<script lang="ts">
    import Icon from '@iconify/svelte';

    export let onUp: undefined | (() => void) = undefined;
    export let onDown: undefined | (() => void) = undefined;
    export let onLeft: undefined | (() => void) = undefined;
    export let onRight: undefined | (() => void) = undefined;
</script>

<div id="screen">
    <div class='nav-arrow'>
        {#if onUp}
        <a on:click={onUp}>
            <Icon icon='carbon:chevron-up' inline />
        </a>
        {/if}
    </div>
    <div id="content">
        <slot />
    </div>
    <div class='nav-arrow'>
        {#if onDown}
        <a on:click={onDown}>
            <Icon icon='carbon:chevron-down' inline/>
        </a>
        {/if}
    </div>
</div>

<style>
    #screen {
        width: 100vw;
        height: 100vh;
        display: flex;
        flex-direction: column;
    }

    #content {
        flex: 1;
    }

    .nav-arrow {
        color: #bbb;
        width: 100%;
        font-size: 1rem;
        text-align: center;
    }
</style>