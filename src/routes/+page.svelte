<script lang="ts">
	import { config } from '$lib/api';
	import { onMount } from 'svelte';
    import Home from './home/Home.svelte';
    import Launcher from './launcher/Launcher.svelte';
	import Settings from './settings/Settings.svelte';
	import { joystick } from '$lib/joystick';

    let screenIndex = 1;

    const goUp = () => {
        screenIndex -= 1;
    }

    const goDown = () => {
        screenIndex += 1;
    }

    onMount(() => {
        joystick.set(['home']);
    });

    $: {
        if ($config.needs_setup) {
            window.location.href = '/setup';
        }
    }
</script>

<div id="page" style={`top: ${-screenIndex * 100}vh;`}>
    <Settings
        goDown={goDown}
    />
    <Home
        goUp={goUp}
        goDown={goDown}
    />
    <Launcher
        goUp={goUp}
    />
</div>

<style>
    #page {
        position: relative;
        transition: top cubic-bezier(0.2, 1, 0.4, 1) 0.4s;
    }
</style>