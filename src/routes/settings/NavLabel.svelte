<script lang="ts">
	import { nav } from "$lib/joystick";

    export let id: string;

    $: selected = $nav === id;
</script>

<div id="navlabel">
    <slot />
    <div
        id="underline"
        class:off={!selected}
        class:on={selected}
    />
</div>

<style>
    #navlabel {
        margin: 0.5rem;
    }

    #underline {
        border-top: 2px solid var(--fg);
    }

    .off {
        width: 0;
        transition: width cubic-bezier(0.2, 1, 0.4, 1) 0.2s;
    }

    .on {
        width: 100%;
        transition: width cubic-bezier(0.2, 1, 0.4, 1) 0.4s;
    }
</style>