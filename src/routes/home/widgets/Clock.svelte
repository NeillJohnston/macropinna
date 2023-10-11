<script lang="ts">
	import type { XAlign, YAlign } from '$lib/layout';
    import strftime from 'strftime';
    import { onMount } from 'svelte';

    export let props: {
        xAlign: XAlign;
        yAlign: YAlign;
    };

    // TODO unhardcode
    const timeFormat = '%A, %b %e%n%l:%M%P';

    let timeString = strftime(timeFormat);
    onMount(() => {
        const updateClock = setInterval(() => {
            // @ts-ignore
            timeString = strftime(timeFormat);
        }, 500);

        return updateClock;
    });

    const xAlignClass = props.xAlign;
    const yAlignClass = props.yAlign;
</script>

<div id="clock" class={yAlignClass}>
    <p id="text" class={xAlignClass}><strong>{timeString}</strong></p>
</div>

<style>
    #clock {
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
    }

    #text {
        width: 100%;
    }

    p {
        margin: 0;
        font-size: 2.00em;
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