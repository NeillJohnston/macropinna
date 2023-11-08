<script lang="ts">
	import type { XAlign, YAlign } from '$lib/layout';
    import strftime from 'strftime';
    import { onMount } from 'svelte';
	import CardModal from '../../ui/CardModal.svelte';
	import MenuSection from '../../ui/MenuSection.svelte';
	import KeyboardInput from '../../ui/KeyboardInput.svelte';
	import { Direction, joystick, nav } from '$lib/joystick';
	import Button from '../../ui/Button.svelte';
	import Icon from '@iconify/svelte';

    export let props: {
        xAlign: XAlign;
        yAlign: YAlign;
        format: string;
    };
    export let save: (newProps: any) => void;
    export let id: string;
    export const entry = id + '/editButton';
    const formatInputId = id + '/edit/formatInput';
    const saveButtonId = id + '/edit/saveButton';

    $: showEditButton = $nav.startsWith(id + '/');

    let timeString = strftime(props.format);
    onMount(() => {
        const updateClock = setInterval(() => {
            // @ts-ignore
            timeString = strftime(props.format);
        }, 500);

        return updateClock;
    });

    let formatInputValue = '';
    const reset = () => {
        formatInputValue = props.format;
    };

    const _save = () => {
        save({
            ...props,
            format: formatInputValue
        });
        joystick.go(Direction.Exit);
    }

    const xAlignClass = props.xAlign;
    const yAlignClass = props.yAlign;

    const formatInfo = [
        {
            header: 'Date',
            items: [
                ['%Y', 'Year'],
                ['%m', 'Month (0-padded)'],
                ['%B', 'Month name'],
                ['%b', 'Month name (abbr.)'],
                ['%d', 'Day of the month (0-padded)'],
                ['%A', 'Weekday'],
                ['%a', 'Weekday (abbr.)'],
            ]
        },
        {
            header: 'Time',
            items: [
                ['%H', 'Hour (00-23)'],
                ['%k', 'Hour (0-23)'],
                ['%I', 'Hour (01-12)'],
                ['%l', 'Hour (1-12)'],
                ['%M', 'Minute'],
                ['%S', 'Second'],
                ['%p', 'AM/PM'],
                ['%P', 'am/pm'],
            ]
        },
        {
            header: 'Other',
            items: [
                ['%Z', 'Timezone name'],
                ['%z', 'Timezone offset'],
                ['%n', 'Newline'],
            ]
        },
    ];
</script>

<div id="clock" class={yAlignClass}>
    <p id="text" class={xAlignClass}><strong>{timeString}</strong></p>
</div>
{#if showEditButton}
<div id="edit-button">
    <Button
        id={entry}
        component={{
            exit: {}
        }}
        onPress={() => {
            reset();
            joystick.push(formatInputId);
        }}
    >
        <Icon icon='carbon:edit' inline />
    </Button>
</div>
{/if}
<CardModal idPrefix={id + '/edit/'}>
    <div id="edit-modal">
        <p class="info"><em>Some common format specifiers. See the npm package "strftime" for an exhaustive list.</em></p>
        <div class="space" />
        <div class="info info-table">
            {#each formatInfo as col}
            <div class="info-col">
                <!-- <p class="header">{col.header}</p> -->
                {#each col.items as [spec, desc]}
                <p><span class="spec">{spec}</span>: {desc}</p>
                {/each}
            </div>
            {/each}
        </div>
        <MenuSection label="Time format">
            <div id="format-input">
                <KeyboardInput
                    id={formatInputId}
                    bind:value={formatInputValue}
                    component={{
                        down: { id: saveButtonId },
                        exit: {}
                    }}
                    placeholder="Time format"
                />
            </div>
            {#if formatInputValue.length > 0}
            <p class="preview">{strftime(formatInputValue)}</p>
            {:else}
            <p class="preview"><em>Clock preview</em></p>
            {/if}
        </MenuSection>
        <div class="space" />
        <Button
            id={saveButtonId}
            component={{
                up: { id: formatInputId },
                exit: {}
            }}
            onPress={_save}
        >
            <Icon icon='carbon:save' inline /> Save
        </Button>
    </div>
</CardModal>

<style>
    #clock {
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
    }

    p {
        margin: 0;
    }

    #clock > p {
        font-size: 1.41em;
        white-space: pre-line;
    }

    #text {
        width: 100%;
        padding: var(--sm);
        box-sizing: border-box;
    }

    #edit-button {
        position: absolute;
        top: var(--sm);
        right: var(--sm);
        font-size: var(--f-1);
    }

    #edit-modal {
        padding: var(--md);
        font-size: var(--f0);
    }

    .info {
        font-size: var(--f-2);
    }

    .info-table {
        display: flex;
    }

    .info-col {
        flex: 1;
    }

    .header {
        font-weight: bold;
        text-decoration: underline;
    }

    .spec {
        font-family: var(--code);
        font-weight: bold;
    }

    #format-input {
    }

    .preview {
        margin: var(--xs) 0;
        font-size: var(--f-2);
        white-space: pre-line;
    }

    .space {
        height: var(--md);
    }

    /* X/Y alignment classes */
    .left   { text-align: left; }
    .center { text-align: center; }
    .right  { text-align: right; }

    .top    { align-items: flex-start; }
    .middle { align-items: center; }
    .bottom { align-items: flex-end; }
</style>