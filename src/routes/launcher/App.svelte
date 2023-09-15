<script lang="ts">
    export let name: string;
    export let selected: boolean;
</script>

<div id="app" class:selected={selected}>
    <div id="inner">
        {name}
    </div>
</div>

<style>
    #app {
        box-sizing: border-box;
        width: 300px;
        height: 300px;
        padding: 50px;
        opacity: 50%;

        transition:
            padding ease 0.5s,
            opacity ease 0.5s;
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