<script lang="ts">
	import { layout } from "$lib/layout";
	import Yapper from "./Yapper.svelte";
    import { invoke } from '@tauri-apps/api';
    import { onMount } from "svelte";
    import Qty from 'js-quantities';
    import strftime from 'strftime';

    let weather = {
        heading: 'Fetching weather...',
        time: '',
        subheadings: ['It might be sunny. It might be rainy. Who knows?']
    };

    const xAlignClass = $layout.home.weather.xAlign;
    const yAlignClass = $layout.home.weather.yAlign;

    const fmt = (x: number, from: string, to: string, d?: number): string => (
        Qty(x, from).to(to).scalar.toFixed(d ?? 0)
    );

    const timeFmt = (t: number) => (
        strftime('%l:%M%P', new Date(1000 * t))
    );

    onMount(() => {
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

        return setInterval(refreshWeather, 10 * 60_000);
    });
</script>

<div id="weather" class={yAlignClass}>
    <div id="container" class={xAlignClass}>
        <p>{weather.heading}</p>
        <p id="time">Last refreshed: {weather.time}</p>
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

    #time {
        font-size: 0.50rem;
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