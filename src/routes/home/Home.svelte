<script context="module">
    export const GRID_X = 12;
    export const GRID_Y = 12;
</script>

<script lang="ts">
	import { config, setConfig, type Config } from "$lib/api";
	import { onMount } from "svelte";
    import { Direction, joystick, nav } from '$lib/joystick';
	import Screen from '../Screen.svelte';
	import Widget from "./Widget.svelte";

    export let goUp: () => void;
    export let goDown: () => void;

    // Enable/disable visual grid
    const grid = false;

    let index = 0;
    $: screens = $config.home.screens;

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
                keep: true,
                action: () => { index = (index - 1 + screens.length) % screens.length; }
            },
            right: {
                keep: true,
                action: () => { index = (index + 1) % screens.length; }
            },
            enter: {
                keep: true,
                id: () => screens[index].widgets.length > 0 ? `home/${index}/0` : undefined
            }
        });
    });

    const widgetId = (screenIndex: number, widgetIndex: number) => (
        `home/${screenIndex}/${widgetIndex}`
    );

    const widgetPropsSave = (screenIndex: number, widgetIndex: number) => (
        (newProps: any) => {
            const newConfig: Config = { ...$config };
            newConfig.home.screens[screenIndex].widgets[widgetIndex].props = newProps;
            setConfig(newConfig);
        }
    );
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
            {#each screen.widgets as { name, coords, props }, widgetIndex}
            <Widget
                name={name}
                coords={coords}
                props={{ ...props }}
                id={widgetId(screenIndex, widgetIndex)}
                leftId={widgetId(screenIndex, (widgetIndex - 1 + screen.widgets.length) % screen.widgets.length)}
                rightId={widgetId(screenIndex, (widgetIndex + 1) % screen.widgets.length)}
                onScreen={joystick.stack.includes('home') && screenIndex === index}
                save={widgetPropsSave(index, widgetIndex)}
            />
            {/each}
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