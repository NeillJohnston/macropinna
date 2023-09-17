<script lang="ts">
    export let name: string;
    export let selected: boolean;
    export let size: string;
    export let margin: string;
</script>

<div
    id="app"
    class:selected={selected}
    style:width={size}
    style:height={size}
    style:margin-right={margin}
>
    <div id="inner">
        {name}
    </div>
</div>

<style>
    #app {
        box-sizing: border-box;
        padding: 80px;
        opacity: 50%;

        transition:
            padding ease 0.4s,
            opacity ease 0.4s;
    }

    #app.selected {
        padding: 0px;
        opacity: 100%;
    }

    #inner {
        width: 100%;
        height: 100%;
        border: 1px solid white;
    }
</style>