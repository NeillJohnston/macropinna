<script lang="ts">
	import Screen from './Screen.svelte';
    import Home from './home/Home.svelte';
    import Launcher from './launcher/Launcher.svelte';
    import { Direction, joystick } from '$lib/joystick';
	import { onMount } from 'svelte';

    let screenIndex = 1

    const goUp = () => {
        screenIndex -= 1;
    }

    const goDown = () => {
        screenIndex += 1;
    }

    onMount(() => {
        joystick.register('settings', {
            down: {
                id: 'home',
                action: goDown
            }
        });
    });
</script>

<div id="page" style={`top: ${-screenIndex * 100}vh;`}>
    <Screen
        onDown={joystick.goFromCb('settings', Direction.Down)}
    >
        <div class="under-construction">
            <div style="height: 50%;" />
            <p>(Under construction)</p>
        </div>
    </Screen>
    <Screen
        onUp={joystick.goFromCb('home', Direction.Up)}
        onDown={joystick.goFromCb('home', Direction.Down)}
    >
        <Home
            goUp={goUp}
            goDown={goDown}
        />
    </Screen>
    <Screen
        onUp={joystick.goFromCb('launcher', Direction.Up)}
    >
        <Launcher
            goUp={goUp}
        />
    </Screen>
</div>

<style>
    #page {
        position: relative;
        transition: top cubic-bezier(0.2, 1, 0.4, 1) 0.4s;
    }

    .under-construction {
        width: 100%;
        height: 100%;
        text-align: center;
        display: flex;
        flex-direction: column-reverse;
    }

    .under-construction p {
        font-size: 0.71rem;
    }
</style>