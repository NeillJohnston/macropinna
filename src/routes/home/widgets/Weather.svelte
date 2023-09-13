<script lang="ts">
	import { layout } from "$lib/layout";
	import Yapper from "./Yapper.svelte";
    import { invoke } from '@tauri-apps/api';
    import { onMount } from "svelte";

    let weather = {
        heading: 'Fetching weather...',
        subheadings: ['It might be sunny. It might be rainy. Who knows?']
    };

    const xAlignClass = $layout.home.weather.xAlign;
    const yAlignClass = $layout.home.weather.yAlign;

    onMount(() => {
        return setInterval(async () => {
            const res: any = await invoke('get_weather');
            if (!!res) {
                // TODO should switch by provider in the future, right now this
                // is hardcoded for OpenWeatherMap
                const curr = res.list[0];
                const next = res.list[1];

                const feelsLike = curr.main.feels_like;
            }
        }, 10 * 60_000);
    });
</script>

<div id="weather" class={yAlignClass}>
    <div id="container" class={xAlignClass}>
        <p>{weather.heading}</p>
        <div class="space" />
        <div id="yapper">
            <Yapper
                blurbs={weather.subheadings}
                cpm={800}
                readDelayMs={10_000}
            />
        </div>
    </div>
</div>

<style>
    #weather {
        width: 100%;
        height: 100%;
        font-size: 1.00rem;
        white-space: pre-line;
        display: flex;
    }

    #container {
        width: 100%;
    }

    #yapper {
        font-size: 0.71rem;
    }

    p {
        margin: 0;
        max-width: 30em;
    }

    .space {
        height: 0.5rem;
    }

    /* X/Y alignment classes */
    .left   { text-align: left; }
    .center { text-align: center; }
    .right  { text-align: right; }

    .top    { align-items: flex-start; }
    .middle { align-items: center; }
    .bottom { align-items: flex-end; }
</style>