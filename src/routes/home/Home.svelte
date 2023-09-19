<script lang="ts">
	import { layout as _layout } from "$lib/layout";
	import { onMount } from "svelte";
	import Clock from "./widgets/Clock.svelte";
	import Todo from "./widgets/Todo.svelte";
	import Weather from "./widgets/Weather.svelte";
    import { joystick } from '$lib/joystick';
	import AudioVisualizer from "./widgets/AudioVisualizer.svelte";

    export let goUp: () => void;
    export let goDown: () => void;

    const GRID_X = 12;
    const GRID_Y = 12;

    let index = 0;
    $: layout = $_layout.home[index];

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

    const grid = false;
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

<div id="page">
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
    {#if layout.clock}
    <div class="box" style={widgetCoords(layout.clock.coords)}>
        <Clock props={layout.clock} />
    </div>
    {/if}
    {#if layout.weather}
    <div class="box" style={widgetCoords(layout.weather.coords)}>
        <Weather props={layout.weather} />
    </div>
    {/if}
    {#if layout.todo}
    <div class="box" style={widgetCoords(layout.todo.coords)}>
        <Todo />
    </div>
    {/if}
    {#if layout.audioVisualizer}
    <div class="box" style={widgetCoords(layout.audioVisualizer.coords)}>
        <AudioVisualizer />
    </div>
    {/if}
</div>

<style>
    #page {
        width: 100%;
        height: 100%;
        position: relative;
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