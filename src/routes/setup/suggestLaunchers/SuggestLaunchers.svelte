<script lang="ts">
	import Icon from "@iconify/svelte";
    import type { ScreenProps } from "../+page.svelte";
	import Button from "../../ui/Button.svelte";
	import ContinueButton from "../ContinueButton.svelte";
	import { invoke } from "@tauri-apps/api";
	import type { Launcher } from "$lib/api";
	import { NavList, joystick } from "$lib/joystick";
	import { onMount } from "svelte";
	import MenuSection from "../../ui/MenuSection.svelte";
	import LauncherItem from "./LauncherItem.svelte";
	import NavLabel from "../../ui/NavLabel.svelte";

    export let props: ScreenProps;

    const runSuggestionsButtonId = 'suggest-launchers';
    const continueButtonId = 'suggest-launchers/continue';

    let navList: NavList;
    let hasRunSuggestions = false;
    let suggested: Launcher[] = [];
    let toggled: boolean[] = [];

    onMount(() => {
        navList = new NavList('suggest-launchers', [
            { name: runSuggestionsButtonId, isId: true },
            { name: 'existing', count: props.config.launchers.length },
            { name: 'suggested', count: 0 },
            { name: continueButtonId, isId: true }
        ]);
    });

    const runSuggestions = async () => {
        // TODO I usually hate this but this function runs too fast, should
        // decide to either 1. remove it and just run this invoke in onMount, or
        // 2. introduce an artificial delay
        suggested = await invoke('suggest_launchers');
        suggested = suggested.filter(launcher => (
            !props.config.launchers.find(_launcher => (
                // Filter out existing launchers with either the same name or
                // command - in either case, a duplicate is likely
                _launcher.name === launcher.name ||
                _launcher.command === launcher.command
            ))
        ));

        toggled = suggested.map(() => false);
        hasRunSuggestions = true;

        if (suggested.length > 0) {
            navList = new NavList('suggest-launchers', [
                { name: runSuggestionsButtonId, isId: true },
                { name: 'existing', count: props.config.launchers.length },
                { name: 'suggested', count: suggested.length },
                { name: continueButtonId, isId: true }
            ]);

            joystick.setTop(navList.get('suggested', 0).id);
        }
        else {
            navList = new NavList('suggest-launchers', [
                { name: runSuggestionsButtonId, isId: true },
                { name: 'existing', count: props.config.launchers.length },
                { name: 'no-suggestions' },
                { name: continueButtonId, isId: true }
            ]);

            const noSuggestions = navList.get('no-suggestions');

            joystick.register(noSuggestions.id, {
                up: { id: noSuggestions.up },
                down: { id: noSuggestions.down }
            });

            joystick.setTop(navList.get('no-suggestions').id);
        }
    };

    const saveAndContinue = () => {
        const added = suggested.filter((_launcher, index) => toggled[index]);
        for (const launcher of added) {
            props.config.launchers.push(launcher);
        }

        props.next();
    };
</script>

<div id="suggest-launchers">
    <!-- TODO disable after running -->
    <Button
        id={runSuggestionsButtonId}
        component={{
            up: { action: props.prev },
            down: { id: () => navList.getById(runSuggestionsButtonId).down },
        }}
        onPress={runSuggestions}
        
    >
        <Icon icon='carbon:run' inline /> Find suggested launchers
    </Button>

    {#if navList}
    <MenuSection label='Launchers'>
        {#each props.config.launchers as launcher, index}
        <LauncherItem
            launcher={launcher}
            id={navList.get('existing', index).id}
            component={{
                up: { id: () => navList.get('existing', index).up },
                down: { id: () => navList.get('existing', index).down },
            }}
        />
        {/each}
    </MenuSection>
    <MenuSection label='Suggestions'>
        {#if hasRunSuggestions && suggested.length === 0}
        <div id="no-suggestions">
            <NavLabel id={navList.get('no-suggestions').id}>
                <em >
                    No suggestions found
                </em>
            </NavLabel>
        </div>
        {/if}
        {#each suggested as launcher, index}
        <LauncherItem
            launcher={launcher}
            id={navList.get('suggested', index).id}
            component={{
                up: { id: () => navList.get('suggested', index).up },
                down: { id: () => navList.get('suggested', index).down },
            }}
            suggestion
            bind:toggled={toggled[index]}
        />
        {/each}
    </MenuSection>

    <ContinueButton
        id={continueButtonId}
        component={{
            up: { id: () => navList.getById(continueButtonId).up },
        }}
        onPress={saveAndContinue}
    />
    {/if}
</div>

<style>
    #suggest-launchers {
        width: 100%;
        height: 100%;
        box-sizing: border-box;
        font-size: var(--f0);
        overflow-y: scroll;
        padding-right: var(--md);
        /* Space for the continue button on the button */
        padding-bottom: 2em;
    }

    #no-suggestions {
        display: inline-block;
        font-size: var(--f-1);
    }
</style>