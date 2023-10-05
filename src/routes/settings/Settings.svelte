<script lang="ts">
	import { Direction, joystick } from "$lib/joystick";
	import { onMount } from "svelte";
	import Screen from "../Screen.svelte";
	import NavLabel from "./NavLabel.svelte";
    import General from "./General.svelte";
	import Remotes from "./Remotes.svelte";

    export let goDown: () => void;

    let menuIndex = 0;
    const menu = [
        {
            id: 'settings/general',
            label: 'General',
            component: General,
            entrance: 'general/list'
        },
        {
            id: 'settings/remotes',
            label: 'Remotes',
            component: Remotes,
            entrance: 'remotes/showqr'
        },
    ];

    onMount(() => {
        joystick.register('settings', {
            down: {
                id: 'home',
                action: () => {
                    goDown();
                    menuIndex = 0;
                },
            },
            right: {
                id: () => menu[menuIndex].id,
            },
            enter: { alias: Direction.Right },
            exit: { alias: Direction.Down },
        });

        for (let i = 0; i < menu.length; ++i) {
            const item = menu[i];
            const prev = (i - 1 + menu.length) % menu.length;
            const next = (i + 1) % menu.length;

            joystick.register(item.id, {
                left: {
                    id: 'settings'
                },
                right: {
                    keep: true,
                    id: item.entrance,
                },
                up: {
                    id: menu[prev].id,
                    action: () => { menuIndex = prev; }
                },
                down: {
                    id: menu[next].id,
                    action: () => { menuIndex = next; }
                },
                enter: { alias: Direction.Right },
                exit: { alias: Direction.Left },
            });
        }
    });
</script>

<Screen
    onDown={joystick.goFromCb('settings', Direction.Down)}
>
    <div id="settings">
        <div id="primary" class="column">
            <div class="item">
                <NavLabel id='settings'>Settings</NavLabel>
            </div>
        </div>
        <div id="secondary" class="column">
            {#each menu as item, index}
            <div class="item">
                <NavLabel id={item.id}>{item.label}</NavLabel>
            </div>
            {/each}
        </div>
        <div id="panel">
            <div id="panel-contents">
                <svelte:component this={menu[menuIndex].component} />
            </div>
        </div>
    </div>
</Screen>

<style>
    #settings {
        width: 100%;
        height: 100%;
        display: flex;
    }

    #primary {
        font-weight: bold;
    }

    #secondary {
    }

    #panel {
        flex: 1;
        height: 100%;
        box-sizing: border-box;
        padding: 0.5rem;
    }

    #panel-contents {
        width: 100%;
        height: 100%;
        border: 1px solid var(--fg);
        box-sizing: border-box;
        overflow: scroll;
        padding: 0.5rem;
    }

    .column {
        padding: 0.5rem 1rem;
    }

    .item {
        padding: 0.5rem;
    }
</style>