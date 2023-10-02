<script lang="ts">
	import { onMount } from "svelte";
	import { joystick } from "$lib/joystick";

    export let id: string;
    export let value: string;
    export let placeholder: string = '';

    const focusId = joystick.focusId(id);

    onMount(() => {
        joystick.register(focusId, {
            exit: joystick.focusExit(id)
        })
    });
</script>

<input
    id={focusId}
    bind:value
    placeholder={placeholder}
/>

<style>
    input {
        font-size: 1em;
        background-color: rgba(0, 0, 0, 0);
        color: white;
        border: none;
        outline: none;
    }
</style>