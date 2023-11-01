<script lang="ts">
	import { nav } from "$lib/joystick";

    export let id: string;
    export let and: boolean = true;
    export let or: boolean = false;

    $: selected = (($nav === id) && and) || or;
</script>

<div class:off={!selected} class:on={selected}>
    <slot />
</div>

<style>
    .off {
        opacity: 0.6;
        transition: opacity ease 0.2s;
    }

    .on {
        opacity: 1.0;
        transition: opacity ease 0.2s;
    }
</style>