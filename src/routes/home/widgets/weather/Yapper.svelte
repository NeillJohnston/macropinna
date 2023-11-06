<!--A component for typing-animated, cycling text blurbs.-->
<script lang="ts">
    import { onMount } from "svelte";

    export let blurbs: string[];
    export let cpm = 1200;
    export let readDelayMs = 3000;
    export let waitDelayMs = 250;
    export let focused = false;

    export const prev = () => {
        t = 0;
        state = 'typing';
        textIndex = 0;
        blurbIndex = (blurbIndex - 1 + blurbs.length) % blurbs.length;
    };
    export const next = () => {
        t = 0;
        state = 'typing';
        textIndex = 0;
        blurbIndex = (blurbIndex + 1) % blurbs.length;
    };

    const charDelayMs = 60_000/cpm;
    const tReadDelay = Math.round(readDelayMs/charDelayMs);
    const tWaitDelay = Math.round(waitDelayMs/charDelayMs);

    let blurbIndex = 0;
    let textIndex = 0;

    let t = 0;
    let state: 'typing' | 'reading' | 'waiting' = 'typing';
    onMount(() => {
        return setInterval(() => {
            t += 1;

            switch (state) {
                case 'typing':
                    ++textIndex;

                    if (textIndex === blurbs[blurbIndex].length) {
                        t = 0;
                        state = 'reading';
                    }
                    break;
                case 'reading':
                    if (t === tReadDelay) {
                        t = 0;
                        blurbIndex = (blurbIndex + 1) % blurbs.length;
                        textIndex = 0;
                        state = 'waiting';
                    }
                    break;
                case 'waiting':
                    if (t === tWaitDelay) {
                        t = 0;
                        state = 'typing';
                    }
                    break;
            }
        }, charDelayMs);
    });

    $: shown = blurbs[blurbIndex].slice(0, textIndex);
    $: hidden = blurbs[blurbIndex].slice(textIndex);
    // TODO maybe unhardcode the cursor delay, it's not that serious though
    $: cursorOff = state !== 'typing' && (t % 20 >= 10);

    $: {
        blurbs;
        t = 0;
        state = 'typing';
        textIndex = 0;
        blurbIndex = 0;
    }
</script>

<p>
    <span class="light" class:focused={focused}>[{blurbIndex+1}/{blurbs.length}]</span>
    {shown}<span class={cursorOff ? 'hidden' : 'light'}>|</span><span class="hidden">{hidden}</span>
</p>

<style>
    p {
        margin: 0;
        /* max-width: 30em; */
    }
    
    .light {
        opacity: 0.6;
    }

    .focused {
        opacity: 1.0;
        text-decoration: underline;
    }

    .hidden {
        color: rgba(0, 0, 0, 0);
    }
</style>