<script lang="ts">
	import DPad from "./DPad.svelte";
    import Keyboard from "./Keyboard.svelte";
	import Touchpad from "./Touchpad.svelte";

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

    .tab {
        text-align: center;
        flex: 1;
        border: 1px solid var(--fg);
        border-top-color: transparent;
        font-size: 0.71rem;
        padding: 8px;
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