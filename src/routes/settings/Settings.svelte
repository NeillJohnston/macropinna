<script lang="ts">
	import { Direction, joystick } from "$lib/joystick";
	import { onMount } from "svelte";
	import Screen from "../Screen.svelte";

    export let goDown: () => void;

    onMount(() => {
        joystick.register('settings', {
            down: {
                id: 'home',
                action: goDown
            }
        });
    });
</script>

<Screen
    onDown={joystick.goFromCb('settings', Direction.Down)}
>
    <div id="settings">
        <p>Settings! Finally!</p>
    </div>
</Screen>

<style>
    #settings {
        width: 100%;
        height: 100%;
    }
</style>