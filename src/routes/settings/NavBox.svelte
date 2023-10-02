<script lang="ts">
	import { nav } from "$lib/joystick";

    export let id: string;
    export let and: boolean = true;
    export let or: boolean = false;

    $: selected = (($nav === id) && and) || or;
</script>

<div
    id="navbox"
    class:off={!selected}
    class:on={selected}
>
    <slot />
</div>

<style>
    #navbox {
        border-width: 1px;
        border-style: solid;
        margin: 0;
        padding: 0;
    }

    .off {
        border-color: rgba(0, 0, 0, 0);
        transition: border-color ease 0.2s;
    }

    .on {
        border-color: white;
        transition: border-color ease 0.2s;
    }
</style>