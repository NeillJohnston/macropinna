<script lang="ts">
	import { onMount } from "svelte";

    onMount(() => {
        const area = document.getElementById('toucharea')!;
    });
</script>

<div id="touchpad">
    <div id="toucharea" />
</div>

<style>
    #touchpad {
        position: relative;
        width: 100%; /* Very landscape hostile */
        height: auto;
        aspect-ratio: 1/1;
        box-sizing: border-box;
        border: 2rem solid transparent;
    }

    #toucharea {
        background-color: var(--bg2);
        width: 100%; /* Very landscape hostile */
        height: 100%;
        border-radius: 0.5rem;
    }
</style>