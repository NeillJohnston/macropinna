<script context="module" lang="ts">
    export interface ScreenProps {
        prev: () => void;
        next: () => void;
        config: Config;
    }
</script>

<script lang="ts">
	import { onMount } from "svelte";
	import { joystick } from "$lib/joystick";
    import Welcome from "./welcome/Welcome.svelte";
	import Localization from "./localization/Localization.svelte";
	import type { Config } from "$lib/api";
    import { config } from "$lib/api";
	import Location from "./location/Location.svelte";

    const SCREEN_ANIM_MS = 1600;

    const screens = [
        {
            screen: Welcome,
            prevId: undefined,
            nextId: 'location'
        },
        {
            screen: Location,
            prevId: 'welcome',
            nextId: undefined
        }
    ];
    let index = -1;
    $: scrollbarHeight = `${100 * (index + 1) / screens.length}%`;

    let prevIndex = -1;
    let animTimeout = 0;

    $: top = (screenIndex: number) => {
        if (screenIndex < index) {
            return '-100vw';
        }
        else if (screenIndex > index) {
            return '100vw';
        }
        else {
            return '0';
        }
    };

    const setIndex = (newIndex: number) => {
        prevIndex = index;
        index = newIndex;

        animTimeout = setTimeout(() => {
            clearTimeout(animTimeout);
            prevIndex = -1;
        }, SCREEN_ANIM_MS);
    };

    $: newConfig = structuredClone($config);

    onMount(() => {
        joystick.set(['welcome']);

        setTimeout(() => {
            setIndex(0);
        });
    });
</script>

<div id="setup">
    {#each screens as { screen, prevId, nextId }, screenIndex}
    <div
        class="screen"
        style:top={top(screenIndex)}
    >
        <svelte:component
            this={(screenIndex === index || screenIndex === prevIndex) ? screen : null}
            props={{
                prev: () => {
                    setIndex(screenIndex - 1);
                    if (prevId) {
                        joystick.set([prevId]);
                    }
                },
                next: () => {
                    setIndex(screenIndex + 1);
                    if (nextId) {
                        joystick.set([nextId]);
                    }
                },
                config: newConfig
            }}
        />
    </div>
    {/each}
    <div id="dim-top" />
    <div id="dim-bottom" />
    <div id="scrollbar" style:height={scrollbarHeight} />
</div>

<style>
    #setup {
        position: absolute;
        width: 100%;
        height: 100%;
        z-index: 0;
    }

    .screen {
        position: absolute;
        width: 100%;
        height: 100%;
        box-sizing: border-box;
        padding: var(--md);
        padding-left: var(--lg);
        padding-right: var(--xl);
        transition: top cubic-bezier(0.2, 1, 0.4, 1) 1.6s;
    }

    #dim-top {
        position: absolute;
        top: 0;
        width: 100%;
        height: var(--md);
        background: linear-gradient(to bottom, var(--bg), rgb(0, 0, 0, 0));
        z-index: 1;
    }

    #dim-bottom {
        position: absolute;
        bottom: 0;
        width: 100%;
        height: var(--md);
        background: linear-gradient(to top, var(--bg), rgb(0, 0, 0, 0));
        z-index: 1;
    }

    #scrollbar {
        position: absolute;
        width: var(--xs);
        right: 0;
        z-index: 2;
        background-color: var(--fg);
        transition: height cubic-bezier(0.2, 1, 0.4, 1) 1.6s;
    }
</style>