<script lang="ts">
	import type { XAlign, YAlign } from "$lib/layout";
	import Yapper from "../../../ui/Yapper.svelte";
    import { onMount } from "svelte";
	import { joystick, nav } from "$lib/joystick";
	import Icon from "@iconify/svelte";
	import { getWeather } from "./provider";
	import Button from "../../../ui/Button.svelte";
	import EditWeatherModal from "./EditWeatherModal.svelte";
	import EditButton from "../EditButton.svelte";

    export let props: {
        xAlign: XAlign;
        yAlign: YAlign;
        heading: string;
        subheadings: string[];
    };
    export let save: () => void;
    export let id: string;
    export const entry = id + '/yapper';

    let editModal: EditWeatherModal;

    const editButtonId = id + '/editButton';
    const editModalId = id + '/editModal';

    $: showEditButton = $nav.startsWith(id + '/');

    let weatherData: any = undefined;
    let weather = {
        heading: 'Fetching weather...',
        time: 'never',
        subheadings: ['...']
    };
    $: {
        if (!!weatherData) {
            const getByKey = (key: string) => weatherData[key] ?? '?';
    
            weather = {
                heading: getByKey(props.heading),
                time: weatherData.fetched,
                subheadings: props.subheadings.map(key => getByKey(key))
            };
        }
    }

    onMount(() => {
        const refreshWeather = async () => {
            // TODO unhardcode formats/units
            weatherData = await getWeather({
                timeFormat: '%l:%M%P',
                tempUnit: 'F'
            });
        };

        refreshWeather();

        // Just refresh every 10 minutes, should be fine
        return setInterval(refreshWeather, 10 * 60_000);
    });

    const xAlignClass = props.xAlign;
    const yAlignClass = props.yAlign;
</script>

<div id="weather" class={yAlignClass}>
    <div id="container" class={xAlignClass}>
        <p id="heading">{weather.heading}</p>
        <p id="time">Last refreshed: {weather.time}</p>
        <div class="space" />
        <div id="yapper">
            <Yapper
                blurbs={weather.subheadings}
                cpm={800}
                readDelayMs={10_000}
                id={entry}
                component={{
                    up: { id: editButtonId },
                    exit: {}
                }}
            />
        </div>
    </div>
</div>
{#if showEditButton}
<EditButton
    id={editButtonId}
    component={{
        down: { id: entry },
        exit: {}
    }}
    onPress={() => {
        editModal.reset();
        joystick.push(editModal.entry);
    }}
/>
{/if}
<EditWeatherModal
    props={props}
    save={save}
    id={editModalId}
    bind:this={editModal}
    data={weatherData}
/>

<style>
    #weather {
        width: 100%;
        height: 100%;
        white-space: pre-line;
        display: flex;
        flex-direction: column;
    }

    #container {
        width: 100%;
        padding: var(--sm);
        box-sizing: border-box;
    }

    #yapper {
        font-size: 0.50em;
    }

    p {
        margin: 0;
    }

    #heading {
        font-size: 0.71em;
    }

    #time {
        font-size: 0.35em;
    }

    .space {
        height: var(--md);
    }

    /* X/Y alignment classes */
    .left   { text-align: left; }
    .center { text-align: center; }
    .right  { text-align: right; }

    .top    { align-items: flex-start; }
    .middle { align-items: center; }
    .bottom { align-items: flex-end; }
</style>