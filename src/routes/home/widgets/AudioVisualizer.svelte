<script lang="ts">
	import { joystick, nav } from "$lib/joystick";
	import type { YAlign } from "$lib/layout";
	import { invoke } from "@tauri-apps/api";
	import { onMount } from "svelte";

    export let props: {
        yAlign: YAlign;
    };
    export let id: string;
    export const entry = id + '/amp';

    $: showMenu = $nav.startsWith(id + '/');

    // TODO put all of these params in props

    // Update frequency, in Hz
    const freq = 30;
    // Number of bars in the display
    const nBars = 50;

    const data = new Array(nBars).fill(0);
    const displayData = new Array(nBars).fill(0);
    // let running = true;

    // Exponential time-series smoothing
    const smooth = 0.4;
    // Window smoothing
    const wSmooth = 0.3;
    const wSmoothRadius = 2;
    
    // Function that maps raw volumes to bar heights (as proportions). Chosen
    // s.t. clipping is impossible but the height of each bar still looks
    // roughly proportional to its real volume. `amp` can be adjusted to make
    // bars reach the ceiling faster or slower
    let amp = 0.0;
    const bar = (x: number) => (1 - 2/(1 + Math.exp(Math.pow(2, amp/8) * x)));

    // Function that maps proportions of graph to proportion of audio spectrum.
    // Must be monotonic from (0, 0) to (1, 1). Chosen to boost the area taken
    // by lower frequencies a bit
    const a2 = 0.1;
    const a3 = 0.5;
    const dens = (x: number) => a3*x*x*x + a2*x*x + (1 - a3 - a2)*x;

    onMount(() => {
        joystick.register(entry, {
            up: {
                keep: true,
                action: () => {
                    amp += 1;
                }
            },
            down: {
                keep: true,
                action: () => {
                    amp -= 1;
                }
            },
            exit: {}
        })

        const interval = setInterval(async () => {
            // if (!running) return;

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

        return () => {
            clearInterval(interval);
        }
    });

    const yAlignClass = props.yAlign;
</script>

<div id="visualizer" class={yAlignClass}>
    {#each displayData as x}
    <div class="bar" style:height={`${100 * bar(x)}%`} />
    {/each}
    {#if showMenu}
    <div id="amp">
        <span>Amp:&nbsp;<span class="mono"><strong>{amp > 0 ? '+' : ''}{amp}</strong></span></span>
    </div>
    {/if}
</div>

<style>
    #visualizer {
        width: 100%;
        height: 100%;
        display: flex;
        position: relative;
    }

    .bar {
        flex: 1;
        background: var(--fg);
        margin: 4px;
        transition: height linear 0.033s;
    }

    #amp {
        width: 100%;
        height: 100%;
        padding: var(--md);
        position: absolute;
        background-color: rgba(0, 0, 0, 0.5);
        font-size: var(--f-1);
        display: flex;
        flex-direction: column;
        justify-content: center;
        text-align: center;
    }

    /* Y alignment classes */
    .top    { align-items: flex-start; }
    .middle { align-items: center; }
    .bottom { align-items: flex-end; }
</style>