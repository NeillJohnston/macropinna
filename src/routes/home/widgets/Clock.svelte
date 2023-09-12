<script lang="ts">
	import { config } from '$lib/config';
    import * as strftime from 'strftime';
    import { onMount } from 'svelte';

    const timeFormat = '%A, %b %e%n%l:%M%P';

    // @ts-ignore: strftime is a function actually
    let timeString = strftime(timeFormat);
    onMount(() => {
        const updateClock = setInterval(() => {
            // @ts-ignore
            timeString = strftime(timeFormat);
        }, 500);

        return updateClock;
    });

    const xAlignClass = $config.home.clock.xAlign;
    const yAlignClass = $config.home.clock.yAlign;
</script>

<div id="clock" class={yAlignClass}>
    <p id="text" class={xAlignClass}><strong>{timeString}</strong></p>
</div>

<style>
    #clock {
        width: 100%;
        height: 100%;
        display: flex;
    }

    #text {
        width: 100%;
    }

    p {
        margin: 0;
        font-size: 2.00rem;
        white-space: pre-line;
    }

    /* X/Y alignment classes */
    .left   { text-align: left; }
    .center { text-align: center; }
    .right  { text-align: right; }

    .top    { align-items: flex-start; }
    .middle { align-items: center; }
    .bottom { align-items: flex-end; }
</style>