<script lang="ts">
	import { invoke } from "@tauri-apps/api";
	import { onMount } from "svelte";

    const freq = 30;
    const nBars = 50;
    const data = new Array(nBars).fill(0);
    const smooth = 0.4;
    const scale = 2.0;

    onMount(() => {
        return setInterval(async () => {
            const res: { data: number[] } = await invoke('get_audio_spectrum');
            const raw = res.data;
            for (let i = 0; i < nBars; ++i) {
                const l = Math.floor(i / nBars * raw.length);
                const r = Math.floor((i + 1) / nBars * raw.length);
                const series = raw.slice(l, r);
                const val = series.reduce((a, x) => a + x, 0);
                data[i] = (
                    (1 - smooth) * data[i] +
                    smooth * val
                );
            }
        }, 1000/freq);
    });
</script>

<div id="visualizer">
    {#each data as bar}
    <div class="bar" style:height={`${100 * bar * scale}%`} />
    {/each}
</div>

<style>
    #visualizer {
        width: 100%;
        height: 100%;
        display: flex;
        /* align-items: center; */
    }

    .bar {
        flex: 1;
        background: white;
        margin: 4px;
    }
</style>