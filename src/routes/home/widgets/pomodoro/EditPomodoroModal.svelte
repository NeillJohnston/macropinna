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
		timerTypes: { name: string; duration: number }[];
	};

	export let save: (newProps: any) => void;
	export let id: string;
	export const entry = id + '/timerSettings';

	let timerTypes = props.timerTypes;

	let timerDurationsList = timerTypes.map((timerType) => timerType.duration.toString());

	// TODO: Make reset make sense
	export const reset = () => {
		// This reset is not working. Might be causing a bug.
		// Maintain an 'instance' based timerTypes that resets when modal is reopened.
		timerTypes = props.timerTypes;
		timerDurationsList = timerTypes.map((timerType) => (timerType.duration / 60).toString());
	};

	// TODO: Disallow user from leaving input blank
	export const validate = (input: string) => {
		const regex = /^\d+$/;
		return regex.test(input);
	};

	// TODO: Not updating config/props properly
	const _save = () => {
		timerTypes = timerDurationsList.map((timerDuration, index) => ({
			name: timerTypes[index].name,
			duration: parseInt(timerDuration) * 60
		}));
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
			<p id="label">{timerTypes[0].name}</p>
			<KeyboardInput
				id={entry}
				component={{
					down: { id: id + '/shortBreakInput' },
					exit: {}
				}}
				bind:value={timerDurationsList[0]}
				placeholder={timerTypes[0].name}
				valid={validate(timerDurationsList[0])}
			/>
			<p id="label">{timerTypes[1].name}</p>
			<KeyboardInput
				id={id + '/shortBreakInput'}
				component={{
					up: { id: entry },
					down: { id: id + '/longBreakInput' },
					exit: {}
				}}
				bind:value={timerDurationsList[1]}
				placeholder={timerTypes[1].name}
				valid={validate(timerDurationsList[1])}
			/>
			<p id="label">{timerTypes[2].name}</p>
			<KeyboardInput
				id={id + '/longBreakInput'}
				component={{
					up: { id: id + '/shortBreakInput' },
					down: { id: id + '/saveButton' },
					exit: {}
				}}
				bind:value={timerDurationsList[2]}
				placeholder={timerTypes[2].name}
				valid={validate(timerDurationsList[2])}
			/>

			<div class="space" />
			<Button
				id={id + '/saveButton'}
				component={{
					up: { id: id + '/longBreakInput' },
					exit: {}
				}}
				onPress={_save}
				disabled={timerDurationsList.some((timerDuration) => !validate(timerDuration))}
				alertText="Please enter a valid number for each timer."
			>
				<Icon icon="carbon:save" inline /> Save
			</Button>
		</MenuSection>
	</div>
</CardModal>

<style>
	#label {
		font-size: var(--f-1);
		margin: var(--sm) 0;
	}

	#edit-pomodoro-modal {
		padding: var(--md);
		font-size: var(--f0);
	}

	.space {
		height: var(--md);
	}
</style>
