<script lang="ts">
	import { config } from "$lib/config";
	import Clock from "./widgets/Clock.svelte";
	import Todo from "./widgets/Todo.svelte";
	import Weather from "./widgets/Weather.svelte";

    const GRID_X = 12;
    const GRID_Y = 12;

    const layout = $config.home;

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
    {#if layout.clock.enabled}
    <div class="box" style={widgetCoords(layout.clock.widget)}>
        <Clock />
    </div>
    {/if}
    {#if layout.weather.enabled}
    <div class="box" style={widgetCoords(layout.weather.widget)}>
        <Weather />
    </div>
    {/if}
    {#if layout.todo.enabled}
    <div class="box" style={widgetCoords(layout.todo.widget)}>
        <Todo />
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