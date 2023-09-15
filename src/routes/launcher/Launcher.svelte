<script>
	import { onMount } from "svelte";
	import App from "./App.svelte";

    const apps = [
        { name: 'Netflix' },
        { name: 'Wickerpedia' },
        { name: 'Zombo.com' },
    ];

    let index = 0;
    
    let offset = `${index*50}%`;

    onMount(() => setInterval(() => {
        index = (index + 1) % apps.length;
    }, 2000));
</script>

<div id="page">
    <div id="carousel" style:left={offset}>
        {#each apps as app, _index}
        <App name={app.name} selected={index === _index} />
        {/each}
    </div>
</div>

<style>
    #page {
        width: 100%;
        height: 100%;
        position: relative;
    }

    #carousel {
        display: flex;
        transition: left ease 0.5s;
    }
</style>