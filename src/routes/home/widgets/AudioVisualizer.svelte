<script lang="ts">
	import type { YAlign } from "$lib/layout";
	import { invoke } from "@tauri-apps/api";
	import { onMount } from "svelte";

    export let props: {
        yAlign: YAlign;
    };

    // TODO put all of these params in props

    // Update frequency, in Hz
    const freq = 30;
    // Number of bars in the display
    const nBars = 50;

    const data = new Array(nBars).fill(0);
    const displayData = new Array(nBars).fill(0);

    // Exponential time-series smoothing
    const smooth = 0.4;
    // Window smoothing
    const wSmooth = 0.3;
    const wSmoothRadius = 2;
    
    // Function that maps raw volumes to bar heights (as proportions). Chosen
    // s.t. clipping is impossible but the height of each bar still looks
    // roughly proportional to its real volume. `amp` can be adjusted to make
    // bars reach the ceiling faster or slower
    const amp = 4.0;
    const bar = (x: number) => (1 - 2/(1 + Math.exp(amp*x)));

    // Function that maps proportions of graph to proportion of audio spectrum.
    // Must be monotonic from (0, 0) to (1, 1). Chosen to boost the area taken
    // by lower frequencies a bit
    const a2 = 0.1;
    const a3 = 0.5;
    const dens = (x: number) => a3*x*x*x + a2*x*x + (1 - a3 - a2)*x;

    onMount(() => {
        return setInterval(async () => {
            // TODO wrote this in a rush, needs clean-up
            const res: { data: number[] } = await invoke('get_audio_spectrum');
            const raw = res.data;
            for (let i = 0; i < nBars; ++i) {
                const l = Math.round(dens(i / nBars) * raw.length);
                const r = Math.round(dens((i + 1) / nBars) * raw.length);
                const series = raw.slice(l, r);
                const val = series.reduce((a, x) => a + x, 0);
                data[i] = (
                    (1 - smooth) * data[i] +
                    smooth * val
                );
            }
            // Smoothing
            for (let i = 0; i < nBars; ++i) {
                let x = data[i];
                let af = 1;
                for (let f = wSmooth, k = 1; k < wSmoothRadius; ++k, f *= wSmooth) {
                    if (i - k >= 0) {
                        x += f * data[i - k];
                    }
                    if (i + k < nBars) {
                        x += f * data[i + k];
                    }
                    af += 2*f;
                }
                displayData[i] = x/af;
            }
        }, 1000/freq);
    });

    const yAlignClass = props.yAlign;
</script>

<div id="visualizer" class={yAlignClass}>
    {#each displayData as x}
    <div class="bar" style:height={`${100 * bar(x)}%`} />
    {/each}
</div>

<style>
    #visualizer {
        width: 100%;
        height: 100%;
        display: flex;
    }

    .bar {
        flex: 1;
        background: var(--fg);
        margin: 4px;
        transition: height linear 0.033s;
    }

    /* Y alignment classes */
    .top    { align-items: flex-start; }
    .middle { align-items: center; }
    .bottom { align-items: flex-end; }
</style>