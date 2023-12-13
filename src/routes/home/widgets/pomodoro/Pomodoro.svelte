<script lang="ts">
	import Icon from '@iconify/svelte';
	import CarouselSelector from '../../../ui/CarouselSelector.svelte';
	import Button from '../../../ui/Button.svelte';
	import { joystick, nav } from '$lib/joystick';
	import EditButton from '../EditButton.svelte';
	import EditPomodoroModal from './EditPomodoroModal.svelte';
	import type { XAlign, YAlign } from '$lib/layout';
	import { onMount } from 'svelte';

	export let props: {
		xAlign: XAlign;
		yAlign: YAlign;
		timerTypes: { name: string, duration: number }[];
	};

	// Nav related exports
	export let id: string;
	export const entry = id + '/pomodoro/start_pause';

	let editModal: EditPomodoroModal;
	export let save: () => void;

	const carouselID = id + '/pomodoro/carousel';
	const resetButtonID = id + '/pomodoro/reset';
	const editButtonID = id + '/editButton';
	const editModalID = id + '/editModal';

	$: showEditButton = $nav.startsWith(id + '/');

	let startButtonText = 'Start';

	// Timer related variables and functions
	$: timerTypes = props.timerTypes ?? [];
	$: time = timerTypes?.[0]?.duration;
	let timeRemaining = 0;
	onMount(() => {
		timeRemaining = time;
	});
	let state: 'idle' | 'running' | 'paused' | 'expired' = 'idle';
	let timer: number;
	
	// Carousel Navigation
	let timerTypeIndex = 0;

	$: {
		if(state === 'idle') {
			timeRemaining = timerTypes[timerTypeIndex].duration;
		}
	}


	// Pomodoro Functionality
	const toggleTimer = () => {
		if (state === 'running') {
			clearInterval(timer);
			state = 'paused';
		} else {
			state = 'running';
			if (timeRemaining > 0) {
				timer = setInterval(() => {
					if (timeRemaining <= 0) {
						state = 'expired';
						startButtonText = getButtonText();
						clearInterval(timer);
					} else {
						timeRemaining--;
					}
				}, 1000);
			} else {
				state = 'expired';
			}
		}
		startButtonText = getButtonText();
	};

	const reset = () => {
		clearInterval(timer);
		timeRemaining = time;
		state = 'idle';
		startButtonText = getButtonText();
	};

	const formatTime = (time: number): string => {
		const seconds = time;
		const h = Math.floor(seconds / 3600);
		const m = Math.floor((seconds % 3600) / 60);
		const s = Math.round(seconds % 60);

		const formattedTime = [h, m > 9 ? m : h ? '0' + m : m || '0', s > 9 ? s : '0' + s]
			.filter(Boolean)
			.join(':');

		return time < 0 && seconds ? `-${formattedTime}` : formattedTime;
	};

	const setTimerType = () => {
		reset();

		// Store new time based on timer type
		const newTime = timerTypes[timerTypeIndex].duration;
		if (newTime) time = newTime;
		timeRemaining = time;
	};

	const getButtonText = (): string => {
		if (state === 'running') {
			return 'Pause';
		} else if (state === 'paused') {
			return 'Resume';
		} else if (state === 'expired') {
			return 'Reset';
		} else {
			return 'Start';
		}
	};

	const handleStartButtonPress = () => {
		if (state === 'expired') {
			reset();
		} else {
			toggleTimer();
		}
	};

	$: yAlignClass = props.yAlign;

	// TODO: Auto advance pomodoro (WIP) - want to auto advance to short break, back to pomodoro, and then long break after a user-defined number of cycles has been completed
</script>

<div id="pomodoro" class={yAlignClass}>
	<div id="carousel">
		<CarouselSelector
			id={carouselID}
			component={{
				down: { id: entry },
				exit: {}
			}}
			onChange={setTimerType}
			bind:index={timerTypeIndex}
			values={timerTypes.map(timerType => timerType.name)}
		/>
	</div>
	<span id="timer">{formatTime(timeRemaining)}</span>
	<div id="timer_controls">
		<Button
			id={entry}
			component={{
				up: { id: carouselID },
				right: { id: resetButtonID },
				exit: {}
			}}
			onPress={handleStartButtonPress}
		>
			{startButtonText}
		</Button>
		<Button
			id={resetButtonID}
			component={{
				up: { id: carouselID },
				left: { id: entry },
				right: { id: editButtonID },
				exit: {}
			}}
			onPress={reset}
		>
			<Icon icon="lucide:timer-reset" inline />
		</Button>
	</div>
</div>
{#if showEditButton}
<div id="edit_button">
	<EditButton
		id={editButtonID}
		component={{
			up: { id: carouselID },
			left: { id: resetButtonID },
			exit: {}
		}}
		onPress={() => {
			editModal.reset();
			joystick.push(editModal.entry);
		}}
	/>
</div>
{/if}
<EditPomodoroModal
	props={props}
	save={save}
	id={editModalID}
	bind:this={editModal}
/>

<style>
	#pomodoro {
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	#carousel {
		font-size: var(--f-1);
		width: 70%;
	}

	#timer {
		font-size: 2.5em;
		line-height: 1;
		margin: var(--sm) 0;
	}

	#timer_controls {
		font-size: var(--f0);
		margin-top: var(--md);
	}

	#edit_button {
		position: absolute;
		bottom: var(--xl);
		right: var(--sm);
	}

	/* Y alignment classes */
	.top    { justify-content: flex-start; }
	.middle { justify-content: center; }
	.bottom { justify-content: flex-end; }
</style>
