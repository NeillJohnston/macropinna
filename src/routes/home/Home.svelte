<script lang="ts">
	import { layout as _layout } from "$lib/layout";
	import { onMount } from "svelte";
	import Clock from "./widgets/Clock.svelte";
	import Todo from "./widgets/Todo.svelte";
	import Weather from "./widgets/Weather.svelte";
    import { Direction, joystick } from '$lib/joystick';
	import AudioVisualizer from "./widgets/AudioVisualizer.svelte";
	import Screen from '../Screen.svelte';
	import Player from "./widgets/Player.svelte";

    export let goUp: () => void;
    export let goDown: () => void;

    // Enable/disable visual grid
    const grid = false;

    const GRID_X = 12;
    const GRID_Y = 12;

    let index = 0;
    $: screens = $_layout.home;

    const portionX = (n: number) => 100.0 * n / GRID_X;
    const portionY = (n: number) => 100.0 * n / GRID_Y;
    const widgetCoords = (widget: { x: number, y: number, w: number, h: number }): string => {
        if (!widget) {
            return 'width: 0; height: 0';
        }

        let { x, y, w, h } = widget;
        return `
        left: ${portionX(x)}%;
        top: ${portionY(y)}%;
        width: ${portionX(w)}%;
        height: ${portionY(h)}%;
        `;
    };

    const xRange = new Array(GRID_X);
    const yRange = new Array(GRID_Y);

    onMount(() => {
        joystick.register('home', {
            up: {
                id: 'settings',
                action: goUp
            },
            down: {
                id: 'launcher',
                action: goDown
            },
            left: {
                action: () => { index = (index - 1 + $_layout.home.length) % $_layout.home.length; }
            },
            right: {
                action: () => { index = (index + 1) % $_layout.home.length; }
            },
        });
    });
</script>

<Screen
    onUp={joystick.goFromCb('home', Direction.Up)}
    onDown={joystick.goFromCb('home', Direction.Down)}
    onLeft={joystick.goFromCb('home', Direction.Left)}
    onRight={joystick.goFromCb('home', Direction.Right)}
>
    <div id="page" style:left={`${100*-index}%`}>
        {#each screens as screen, screenIndex}
        <div class="screen" style:left={`${100*screenIndex}%`}>
            <!-- Debug grid -->
            {#if grid}
            <div id="grid">
                {#each yRange as _y}
                <div class="grid-row">
                    {#each xRange as _x}
                    <div class="grid-box" />
                    {/each}
                </div>
                {/each}
            </div>
            {/if}
            <!-- Widgets -->
            {#if screen.clock}
            <div class="box" style={widgetCoords(screen.clock.coords)}>
                <Clock props={screen.clock} />
            </div>
            {/if}
            {#if screen.weather}
            <div class="box" style={widgetCoords(screen.weather.coords)}>
                <Weather props={screen.weather} />
            </div>
            {/if}
            {#if screen.todo}
            <div class="box" style={widgetCoords(screen.todo.coords)}>
                <Todo />
            </div>
            {/if}
            {#if screen.audioVisualizer}
            <div class="box" style={widgetCoords(screen.audioVisualizer.coords)}>
                <AudioVisualizer props={screen.audioVisualizer} />
            </div>
            {/if}
            {#if screen.player}
            <div class="box" style={widgetCoords(screen.player.coords)}>
                <Player />
            </div>
            {/if}
        </div>
        {/each}
    </div>
</Screen>

<style>
    #page {
        width: 100%;
        height: 100%;
        position: relative;
        transition: left cubic-bezier(0.2, 1, 0.4, 1) 0.8s;
    }

    .screen {
        width: 100%;
        height: 100%;
        position: absolute;
    }

    .box {
        /* border: 1px solid #333; */
        position: absolute;
        box-sizing: border-box;
        padding: 0.5rem;
    }

    #grid {
        width: 100%;
        height: 100%;
        position: absolute;
        display: flex;
        flex-direction: column;
    }

    .grid-row {
        display: flex;
        flex: 1;
    }

    .grid-box {
        border: 1px solid #333;
        flex: 1;
    }
</style>