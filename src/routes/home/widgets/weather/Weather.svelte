<script lang="ts">
	import type { XAlign, YAlign } from "$lib/layout";
	import Yapper from "./Yapper.svelte";
    import { invoke } from '@tauri-apps/api';
    import { onMount } from "svelte";
    import Qty from 'js-quantities';
    import strftime from 'strftime';
	import { joystick, nav } from "$lib/joystick";
	import Modal from "../../../ui/Modal.svelte";
	import Icon from "@iconify/svelte";
	import NavText from "../../../ui/NavText.svelte";

    export let props: {
        xAlign: XAlign;
        yAlign: YAlign;
    };
    export let id: string;
    export const entry = id + '/yapper';

    const editButtonId = id + '/editButton';
    // const editModalId = id + '/editModal';

    $: showEditButton = $nav.startsWith(id + '/');
    // $: showEditModal = $nav.startsWith(editModalId);

    const xAlignClass = props.xAlign;
    const yAlignClass = props.yAlign;

    const fmt = (x: number, from: string, to: string, d?: number): string => (
        Qty(x, from).to(to).scalar.toFixed(d ?? 0)
    );

    // TODO unhardcode time format
    const timeFmt = (t: number) => (
        strftime('%l:%M%P', new Date(1000 * t))
    );

    let weather = {
        heading: 'Fetching weather...',
        time: 'never',
        subheadings: ['...']
    };

    let yapper: any;

    onMount(() => {
        joystick.register(entry, {
            left: {
                keep: true,
                action: yapper.prev
            },
            right: {
                keep: true,
                action: yapper.next
            },
            up: { id: editButtonId },
            exit: {}
        });

        joystick.register(editButtonId, {
            enter: {
                keep: true,
                // id: editModalId
            },
            down: { id: entry },
            exit: {}
        });

        const refreshWeather = async () => {
            const res: any = await invoke('get_weather');
            if (!!res) {
                const extract = (w: any) => ({
                    time: timeFmt(w.dt),
                    condition: w.weather[0].description,
                    realTemp: fmt(w.main.temp, 'tempK', 'tempF'),
                    feelsLikeTemp: fmt(w.main.feels_like, 'tempK', 'tempF'),
                });

                // TODO should switch by provider in the future, right now this
                // is hardcoded for OpenWeatherMap
                const cityName = res.current.name;

                const now = extract(res.current);
                const later1 = extract(res.forecast.list[0]);
                const later2 = extract(res.forecast.list[1]);

                const sunsetTime = timeFmt(res.current.sys.sunset);

                weather = {
                    heading: `It's ${now.realTemp}°F (feels like ${now.feelsLikeTemp}°F) here in ${cityName}, with ${now.condition}.`,
                    time: now.time,
                    subheadings: [
                        `Sunset today is at ${sunsetTime}.`,
                        `${later1.time}: expect ${later1.condition} and temperatures around ${later1.realTemp}°F.`,
                        `${later2.time}: expect ${later2.condition} and temperatures around ${later2.realTemp}°F.`,
                    ]
                };
            }
        };

        refreshWeather();

        // Just refresh every 10 minutes, should be fine
        return setInterval(refreshWeather, 10 * 60_000);
    });
</script>

<div id="weather" class={yAlignClass}>
    <div id="container" class={xAlignClass}>
        <p id="heading">{weather.heading}</p>
        <p id="time">Last refreshed: {weather.time}</p>
        <div class="space" />
        <div id="yapper">
            <Yapper
                bind:this={yapper}
                blurbs={weather.subheadings}
                cpm={800}
                readDelayMs={10_000}
                focused={$nav === entry}
            />
        </div>
    </div>
</div>
{#if showEditButton}
<div id="edit-button">
    <NavText id={editButtonId}><Icon icon='carbon:edit' inline /></NavText>
</div>
{/if}
<!-- {#if showEditModal}
<Modal>hello</Modal>
{/if} -->

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
        padding: 0.25rem;
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
        height: 0.5rem;
    }

    #edit-button {
        position: absolute;
        top: 0.25rem;
        right: 0.25rem;
        font-size: 0.71rem;
    }

    /* X/Y alignment classes */
    .left   { text-align: left; }
    .center { text-align: center; }
    .right  { text-align: right; }

    .top    { align-items: flex-start; }
    .middle { align-items: center; }
    .bottom { align-items: flex-end; }
</style>