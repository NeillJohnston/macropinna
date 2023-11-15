<script lang="ts">
	import type { XAlign, YAlign } from "$lib/layout";
	import Icon from "@iconify/svelte";
	import Button from "../../../ui/Button.svelte";
	import CardModal from "../../../ui/CardModal.svelte";
	import CarouselSelector from "../../../ui/CarouselSelector.svelte";
	import { weatherItems } from "./provider";
	import { Direction, NavList, joystick, nav } from "$lib/joystick";
	import MenuSection from "../../../ui/MenuSection.svelte";

    export let props: {
        xAlign: XAlign;
        yAlign: YAlign;
        heading: string;
        subheadings: string[];
    };
    export let save: (newProps: any) => void;
    export let id: string;
    export const entry = id + '/heading';

    export let data: any;
    
    let headingIndex = 0;
    let subheadingIndices: number[] = [];
    let navList = new NavList(id, []);

    export const reset = () => {
        const findByKey = (key: string) => {
            const index = weatherItems.findIndex(({ key: _key }) => key === _key);
            return index === -1 ? 0 : index;
        }

        headingIndex = findByKey(props.heading);
        subheadingIndices = props.subheadings.map(key => findByKey(key));

        rebuildNav();
    };

    export const rebuildNav = () => {
        navList = new NavList(id, [
            { name: 'heading' },
            { name: 'subheading', count: subheadingIndices.length },
            { name: 'addSubheading' },
            { name: 'save' }
        ]);
    }

    const weatherValues = weatherItems.map(({ name }) => name);

    const addSubheading = () => {
        subheadingIndices.push(0);
        rebuildNav();
    }

    const removeSubheading = () => {
        subheadingIndices.pop();
        rebuildNav();
    }

    const _save = () => {
        save({
            ...props,
            heading: weatherItems[headingIndex].key,
            subheadings: subheadingIndices.map(index => weatherItems[index].key),
        });

        joystick.go(Direction.Exit);
    };
</script>

<CardModal idPrefix={id} scroll>
    <div id="edit-weather-modal">
        <MenuSection label="Heading">
            <CarouselSelector
                id={entry}
                component={{
                    down: { id: () => navList.get('heading').down },
                    exit: {}
                }}
                bind:index={headingIndex}
                values={weatherValues}
            />
        </MenuSection>
        <div class="space" />
        <MenuSection label="Subheadings">
            {#each subheadingIndices as subheadingIndex, index}
            <CarouselSelector
                id={navList.get('subheading', index).id}
                component={{
                    up: { id: () => navList.get('subheading', index).up },
                    down: { id: () => navList.get('subheading', index).down },
                    exit: {}
                }}
                bind:index={subheadingIndex}
                values={weatherValues}
            />
            {#if $nav === navList.get('subheading', index).id}
            <p class="preview"><em>{data[weatherItems[subheadingIndex].key]}</em></p>
            {/if}
            <div class="space" />
            {/each}
            <div class="button-row">
                <Button
                    id={navList.get('addSubheading').id}
                    component={{
                        up: { id: () => navList.get('addSubheading').up },
                        down: { id: navList.get('addSubheading').down },
                        right: { id: id + '/removeSubheading' },
                        exit: {}
                    }}
                    onPress={addSubheading}
                >
                    <Icon icon='carbon:add' inline /> Add
                </Button>
                <div class="h-space" />
                <Button
                    id={id + '/removeSubheading'}
                    component={{
                        up: { id: () => navList.get('addSubheading').up },
                        down: { id: navList.get('addSubheading').down },
                        left: { id: navList.get('addSubheading').id },
                        exit: {}
                    }}
                    onPress={removeSubheading}
                >
                    <Icon icon='carbon:close' inline /> Remove
                </Button>
            </div>
        </MenuSection>
        <div class="space" />
        <Button
            id={navList.get('save').id}
            component={{
                up: { id: navList.get('save').up },
                exit: {}
            }}
            onPress={_save}
        >
            <Icon icon='carbon:save' inline /> Save
        </Button>
    </div>
</CardModal>

<style>
    #edit-weather-modal {
        padding: var(--md);
        font-size: var(--f0);
    }

    .preview {
        margin: var(--xs) 0;
        font-size: var(--f-2);
    }

    .space {
        height: var(--md);
    }

    .h-space {
        width: var(--md);
    }

    .button-row {
        display: flex;
    }
</style>