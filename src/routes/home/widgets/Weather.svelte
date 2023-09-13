<script lang="ts">
	import { config } from "$lib/config";
	import Yapper from "./Yapper.svelte";
    import { invoke } from '@tauri-apps/api';

    const heading = 'It\'s 88°F outside in Gainesville, FL.';

    const subheadings = [
        'Sunset today is at 7:42pm.',
        'Expect a low chance of rain until 5pm.',
        'The overnight high is 80°F and the low is 74°F.'
    ];

    const xAlignClass = $config.home.weather.xAlign;
    const yAlignClass = $config.home.weather.yAlign;

    invoke('get_weather')
        .then(console.log)
        .catch(console.log);
</script>

<div id="weather" class={yAlignClass}>
    <div id="container" class={xAlignClass}>
        <p>{heading}</p>
        <div class="space" />
        <div id="yapper">
            <Yapper
                blurbs={subheadings}
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