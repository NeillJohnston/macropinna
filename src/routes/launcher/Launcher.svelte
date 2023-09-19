<script lang="ts">
	import { onMount } from "svelte";
	import App from "./App.svelte";
	import { joystick } from "$lib/joystick";

    export let goUp: () => void;

    const SIZE_PX = 480;
    const MARGIN_PX = 0;
    const size = `${SIZE_PX}px`;
    const margin = `${MARGIN_PX}px`;

    const apps = [
        { name: 'App' },
        { name: 'App' },
        { name: 'App' },
        { name: 'App' },
        { name: 'App' },
        { name: 'App' },
        { name: 'App' },
        { name: 'App' },
        { name: 'App' },
        { name: 'App' },
    ];

    let index = 0;
    $: offset = `${-index*(SIZE_PX + MARGIN_PX)}px`;

    onMount(() => {
        joystick.register('launcher', {
            up: {
                id: 'home',
                action: goUp
            },
            left: {
                action: () => { index = (apps.length + index - 1) % apps.length; }
            },
            right: {
                action: () => { index = (apps.length + index + 1) % apps.length; }
            }
        });
    });
</script>

<div id="page">
    <div id="dim-left" />
    <div id="dim-right" />
    <div id="carousel" style:width={size}>
        <div id="container" style:left={offset}>
            {#each apps as app, _index}
            <App
                name={app.name}
                selected={index === _index}
                size={size}
                margin={margin}
            />
            {/each}
        </div>
    </div>
</div>

<style>
    #page {
        position: relative;
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    #carousel {
        position: relative;
    }

    #container {
        position: relative;
        display: flex;
        width: 100000%;

        transition: left cubic-bezier(0.2, 1, 0.4, 1) 0.4s;
        z-index: -1;
    }

    #dim-left {
        position: absolute;
        top: 0;
        left: 0;
        width: 120px;
        height: 100%;
        background: linear-gradient(to right, black, rgb(0, 0, 0, 0));
        z-index: 1;
    }

    #dim-right {
        position: absolute;
        top: 0;
        right: 0;
        width: 120px;
        height: 100%;
        background: linear-gradient(to left, black, rgb(0, 0, 0, 0));
        z-index: 1;
    }
</style>