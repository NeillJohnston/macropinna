<script lang="ts">
	import type { Launcher } from "$lib/api";
	import { joystick, type Component } from "$lib/joystick";
	import Icon from "@iconify/svelte";
	import Button from "../../ui/Button.svelte";
	import NavLabel from "../../ui/NavLabel.svelte";
	import { onMount } from "svelte";
	import Checkbox from "../../ui/Checkbox.svelte";

    export let launcher: Launcher;
    export let id: string;
    export let component: Component;
    export let suggestion = false;
    export let toggled = false;

    onMount(() => {
        if (!suggestion) {
            console.log(id, component);
            joystick.register(id, component);
        }
    });

    const toggle = () => {
        toggled = !toggled;
    }
</script>

<div class="launcher">
    <div class="icon">
        {#if launcher.image_path}
        <img class="cover" src={launcher.image_path} alt="launcher cover" />
        {:else if launcher.css_background}
        <div class="cover" style:background={launcher.css_background} />
        {:else}
        <div class="cover bordered" />
        {/if}
    </div>
    <div class="info">
        <div class="name">
        {#if !suggestion}
            <div style:display="inline-block">
                <NavLabel id={id}>{launcher.name}</NavLabel>
            </div>
            {:else}
            {launcher.name}
            {/if}
        </div>
        <div>
            <span class="command">$ {launcher.command}</span>
        </div>
    </div>
    {#if suggestion}
    <Checkbox
        id={id}
        component={component}
        bind:on={toggled}
    >
        Add
    </Checkbox>
    {/if}
</div>

<style>
    .launcher {
        margin-bottom: var(--md);
        box-sizing: border-box;
        height: 3em;
        display: flex;
        align-items: center;
    }

    .icon {
        width: auto;
        height: 100%;
        aspect-ratio: 1/1;
    }

    img {
        object-fit: cover;
    }

    .cover {
        width: 100%;
        height: 100%;
    }

    .bordered {
        border: 1px solid var(--fg);
    }

    .info {
        flex: 1;
        padding: 0 var(--md);
    }

    .name {
        margin: 0;
        margin-bottom: var(--sm);
    }

    .command {
        padding: var(--xs);
        font-family: var(--code);
        font-size: var(--f-2);
        background-color: var(--bg2);
    }
</style>