<script lang="ts">
	import type { XAlign, YAlign } from '$lib/layout';
	import CardModal from '../../../ui/CardModal.svelte';
	import Button from '../../../ui/Button.svelte';
	import KeyboardInput from '../../../ui/KeyboardInput.svelte';
	import { Direction, joystick, nav } from '$lib/joystick';
	import MenuSection from '../../../ui/MenuSection.svelte';
	import Icon from '@iconify/svelte';

	export let props: {
		xAlign: XAlign;
		yAlign: YAlign;
		timerTypes: { [key: string]: number };
	};

	export let save: (newProps: any) => void;
	export let id: string;
	export const entry = id + '/timerSettings';

	let timerTypes: { [key: string]: number };

  // TODO: Make reset make sense
	export const reset = () => {
		// This reset is not working. Might be causing a bug.
		// Maintain an 'instance' based timerTypes that resets when modal is reopened.
		timerTypes = props.timerTypes;
	};

  // TODO: Disallow user from leaving input blank
	export const validate = (input: string) => {
		const parsedInput = parseInt(input);
		if (input.trim() === '') {
			return true;
		}
		if (isNaN(parsedInput)) {
			return false;
		}

		return true;
	};

  // TODO: Not updating config/props properly
	const _save = () => {
		save({
			...props,
			timerTypes: timerTypes
		});

		joystick.go(Direction.Exit);
	};
</script>

<CardModal idPrefix={id} scroll>
	<div id="edit-pomodoro-modal">
		<MenuSection label="Timer Lengths (in Minutes):">
			<KeyboardInput
				id={entry}
				component={{
					down: { id: id + '/shortBreakInput' },
					exit: {}
				}}
				value={timerTypes['Pomodoro'].toString()}
				placeholder="Pomodoro Length"
				validate={(value) => validate(value)}
			/>
			<div class="space" />
			<KeyboardInput
				id={id + '/shortBreakInput'}
				component={{
					up: { id: entry },
					down: { id: id + '/longBreakInput' },
					exit: {}
				}}
				value={timerTypes['Short break'].toString()}
				placeholder="Short break Length"
				validate={(value) => validate(value)}
			/>
			<div class="space" />
			<KeyboardInput
				id={id + '/longBreakInput'}
				component={{
					up: { id: id + '/shortBreakInput' },
					down: { id: id + '/saveButton' },
					exit: {}
				}}
				value={timerTypes['Long break'].toString()}
				placeholder="Long break Length"
				validate={(value) => validate(value)}
			/>
		</MenuSection>
		<div class="space" />
		<Button
			id={id + '/saveButton'}
			component={{
				up: { id: id + '/longBreakInput' },
				exit: {}
			}}
			onPress={_save}
		>
			<Icon icon="carbon:save" inline /> Save
		</Button>
	</div>
</CardModal>

<style>
	#edit-pomodoro-modal {
		padding: var(--md);
		font-size: var(--f0);
	}

	.space {
		height: var(--md);
	}
</style>
