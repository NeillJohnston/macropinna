<script lang="ts">
	import { onMount } from "svelte";
	import App from "./App.svelte";
	import { Direction, joystick } from "$lib/joystick";
	import Screen from "../Screen.svelte";
	import { config } from "$lib/api";
	import { invoke } from "@tauri-apps/api";
    import { convertFileSrc } from "@tauri-apps/api/tauri";

    export let goUp: () => void;

    const SIZE_PX = 480;
    const MARGIN_PX = 0;
    const size = `${SIZE_PX}px`;
    const margin = `${MARGIN_PX}px`;

    $: launchers = $config.launchers;

    let index = 0;
    $: offset = `${-index*(SIZE_PX + MARGIN_PX)}px`;

    const launch = () => {
        const name = launchers[index].name;
        invoke('launch', { name })
            .then(console.log);
    }

    onMount(() => {
        joystick.register('launcher', {
            up: {
                id: 'home',
                action: goUp
            },
            left: {
                keep: true,
                action: () => { index = (launchers.length + index - 1) % launchers.length; }
            },
            right: {
                keep: true,
                action: () => { index = (launchers.length + index + 1) % launchers.length; }
            },
            enter: {
                keep: true,
                action: launch
            }
        });
    });
</script>

<Screen
    onUp={joystick.goFromCb('launcher', Direction.Up)}
>
<div id="page">
    <div id="dim-left" />
    <div id="dim-right" />
    <div id="carousel" style:width={size}>
        {#if launchers.length > 0}
        <div id="container" style:left={offset}>
            {#each launchers as launcher, _index}
            <App
                name={launcher.name}
                selected={index === _index}
                size={size}
                margin={margin}
                url={!!launcher.image_path ? convertFileSrc(launcher.image_path) : undefined}
            />
            {/each}
        </div>
        {:else}
        <p id="info"><em>No launchers found. Add some to your config!</em></p>
        {/if}
    </div>
</div>
</Screen>

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

    #info {
        text-align: center;
        font-size: 0.71rem;
    }

    #dim-left {
        position: absolute;
        top: 0;
        left: 0;
        width: 120px;
        height: 100%;
        background: linear-gradient(to right, var(--bg), rgb(0, 0, 0, 0));
        z-index: 1;
    }

    #dim-right {
        position: absolute;
        top: 0;
        right: 0;
        width: 120px;
        height: 100%;
        background: linear-gradient(to left, var(--bg), rgb(0, 0, 0, 0));
        z-index: 1;
    }
</style>