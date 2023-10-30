<script lang="ts">
    export let name: string;
    export let selected: boolean;
    export let size: string;
    export let margin: string;
    export let url: string | undefined;
</script>

<div
    id="app"
    class:selected={selected}
    style:width={size}
    style:height={size}
    style:margin-right={margin}
>
    <div id="inner">
        {#if url}
        <img id="cover" src={url} alt="Launcher cover" />
        {:else}
        {name}
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
        border: 1px solid var(--fg);
    }

    #cover {
        object-fit: cover;
        width: 100%;
        height: 100%;
    }
</style>