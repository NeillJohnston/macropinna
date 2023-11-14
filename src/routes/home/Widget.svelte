<script lang="ts">
    import { GRID_X, GRID_Y } from "./Home.svelte";
    import { joystick, nav } from '$lib/joystick';
    import Clock from "./widgets/Clock.svelte";
	import Todo from "./widgets/Todo.svelte";
	import Weather from "./widgets/weather/Weather.svelte";
	import AudioVisualizer from "./widgets/AudioVisualizer.svelte";
	import Player from "./widgets/Player.svelte";
    import Pomodoro from "./widgets/pomodoro/Pomodoro.svelte";
	import { onMount } from "svelte";

    /*
    THE WIDGET INTERFACE

    Typing across Svelte components is an unknown art to me, so right now
    widgets can only follow a really informal contract about how to behave.

    Widgets *can* have the following exports:
    - An `export let props: ...;`
    - An `export let save: (newProps: any) => void;`
    - An `export let id: string;`
    - An `export const entry = id + ...;`
    - An `export const pause = () => { ... };`
    - An `export const resume = () => { ... };`
    
    Props allow widgets to receive data from config - if a widget doesn't need
    to receive anything from config, this can be safely omitted. Props can be
    saved to the current config with the save function.
    
    Id and entry enable navigation for a widget - in particular, when a widget
    is mounted, navigating into the widget will push whatever is defined in
    entry to the nav stack. If a widget does not wish to enable navigation, then
    this should be omitted.

    Pause and resume are callbacks that are triggered when the widget exits/
    enters the screen.

    It's best for entry to include id as a prefix, so that multiple instances of
    the same widget have separate navigation.
    */

    export let name: string;
    export let coords: { x: number, y: number, w: number, h: number };
    // export let upId: string;
    // export let downId: string;
    export let leftId: string;
    export let rightId: string;
    // Props are arbitrary data that get passed down to the rendered widget
    export let props: any;
    export let id: string;
    export let onScreen: boolean;
    export let save: (props: any) => void;

    $: {
        if (onScreen && widget?.resume) {
            widget.resume();
        }
        else if (!onScreen && widget?.pause) {
            widget.pause();
        }
    }

    // Forcing widget to be an `any` to prevent typechecking errors on props
    const widgetConstructor: any = {
        clock: Clock,
        weather: Weather,
        audioVisualizer: AudioVisualizer,
        player: Player,
        pomodoro: Pomodoro,
        todo: Todo,
    }[name];

    let widget: any;

    const portionX = (n: number) => 100.0 * n / GRID_X;
    const portionY = (n: number) => 100.0 * n / GRID_Y;
    $: style = `
    left: ${portionX(coords.x)}%;
    top: ${portionY(coords.y)}%;
    width: ${portionX(coords.w)}%;
    height: ${portionY(coords.h)}%;
    font-size: calc(0.38rem * ${coords.h});
    `;

    onMount(() => {
        joystick.register(id, {
            left: { id: leftId },
            right: { id: rightId },
            enter: {
                keep: true,
                id: widget.entry,
            },
            exit: {}
        });
    });
</script>

<div id="widget" style={style}>
    <svelte:component
        this={widgetConstructor}
        bind:this={widget}
        props={props}
        save={save}
        id={id}
    />
</div>
<div
    class="select-border"
    class:selected={$nav === id}
    style={style}
/>

<style>
    #widget {
        position: absolute;
        box-sizing: border-box;
    }

    .select-border {
        position: absolute;
        border: 1px solid rgba(0, 0, 0, 0);
        transition: border-color ease 0.2s;
    }

    .selected {
        border-color: var(--fg);
    }
</style>