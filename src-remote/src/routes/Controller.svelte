<script lang="ts">
	import Icon from "@iconify/svelte";
import DPad from "./DPad.svelte";
    import Keyboard from "./Keyboard.svelte";
	import Touchpad from "./Touchpad.svelte";
	import { connection } from "$lib/api";
	import Button from "./ui/Button.svelte";

    const tabs = [
        {
            label: 'D-Pad',
            component: DPad,
        },
        {
            label: 'Touchpad',
            component: Touchpad
        }
    ];

    const back = () => {
        $connection?.send({ DPad: 'Exit' });
    };

    const home = () => {
        $connection?.send({ Action: 'Home' });
    };

    const altTab = () => {
        $connection?.send({ Action: 'AltTab' });
    };

    let tabIndex = 0;
</script>

<div id="controller">
    <div id="touch">
        <div id="tabs">
            {#each tabs as tab, index}
            <div
                class="tab"
                class:selected={index === tabIndex}
                class:left={index === 0}
                class:right={index === tabs.length - 1}
                on:click={() => { tabIndex = index; }}
                on:keypress={()=>{}}
                role="button"
                tabindex=0
            >
                {tab.label}
            </div>
            {/each}
        </div>
        <svelte:component this={tabs[tabIndex].component} />
    </div>
    <div id="quick-row">
        <div class="quick-row-button">
            <Button onClick={altTab} display='block'>
                <Icon icon='carbon:arrows-horizontal' inline />
            </Button>
        </div>
        <div class="quick-row-button">
            <Button onClick={home} display='block'>
                <Icon icon='carbon:home' inline />
            </Button>
        </div>
        <div class="quick-row-button">
            <Button onClick={back} display='block'>
                <Icon icon='carbon:chevron-left' inline />
            </Button>
        </div>
    </div>
    <Keyboard />
</div>

<style>
    #controller {
        z-index: 0;
    }

    #touch {
        z-index: 0;
    }

    #tabs {
        display: flex;
    }

    #quick-row {
        display: flex;
    }

    .quick-row-button {
        padding: 4px;
        flex: 1;
        text-align: center;
        font-size: 1.41rem;
    }

    .tab {
        text-align: center;
        flex: 1;
        border: 1px solid var(--fg);
        border-top-color: transparent;
        font-size: 0.71rem;
        padding: 8px;
    }

    .tab:active {

    }

    .left {
        border-left-color: transparent;
    }

    .right {
        border-right-color: transparent;
    }

    .selected {
        border-bottom-color: transparent;
    }
</style>