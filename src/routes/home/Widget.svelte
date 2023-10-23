<script lang="ts">
    import { GRID_X, GRID_Y } from "./Home.svelte";
    import Clock from "./widgets/Clock.svelte";
	import Todo from "./widgets/Todo.svelte";
	import Weather from "./widgets/Weather.svelte";
	import AudioVisualizer from "./widgets/AudioVisualizer.svelte";
	import Player from "./widgets/Player.svelte";

    export let name: string;
    export let coords: { x: number, y: number, w: number, h: number };
    export let props: any;

    const widget = {
        clock: Clock,
        weather: Weather,
        audioVisualizer: AudioVisualizer,
        player: Player,
        todo: Todo,
    }[name];

    const portionX = (n: number) => 100.0 * n / GRID_X;
    const portionY = (n: number) => 100.0 * n / GRID_Y;
    $: style = `
    left: ${portionX(coords.x)}%;
    top: ${portionY(coords.y)}%;
    width: ${portionX(coords.w)}%;
    height: ${portionY(coords.h)}%;
    font-size: calc($1.00rem * ${coords.h});
    `;
</script>

<div class="widget" style={style}>
    <svelte:component this={widget} props={props} />
</div>

<style>
    .widget {
        position: absolute;
        box-sizing: border-box;
    }
</style>