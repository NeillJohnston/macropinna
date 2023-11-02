<script lang="ts">
	import { joystick } from '$lib/joystick';
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';

	// Nav related exports
	export let id: string;
	export const entry = id + '/pomodoro';

	let timerType = 'pomodoro';
	let time = 1500;
	let timer: number;

	// Map to store Pomodoro lengths in seconds, TODO: need to make this configurable in settings
	const timerTypes = new Map<string, number>([
		['pomodoro', 1500],
		['short break', 300],
		['long break', 900]
	]);

	// Timer related variables and functions
	let timeRemaining = time;
	let running = false;
	let paused = false;
	let expired = false;

	// Pomodoro Navigation
	const buttonLayout = [
		['pomodoro', 'short break', 'long break'], // [0][0, 1, 2]
		['start', 'reset']												 // [1][0, 1]
	];

	let currentRow = 0;
	let currentCol = 0;
	let currentSelectedButton: string;

	export const navigate = (direction: string) => {
		switch (direction) {
			case 'up':
				if (currentRow > 0) {
					currentRow--;
				}
				break;
			case 'down':
				if (currentRow < buttonLayout.length - 1) {
					currentRow++;
				}
				break;
			case 'left':
				if (currentCol > 0) {
					currentCol--;
				}
				break;
			case 'right':
				if (currentCol < buttonLayout[currentRow].length - 1) {
					currentCol++;
				}
				break;
		}

		currentSelectedButton = buttonLayout[currentRow][currentCol];
	};

	// Pomodoro Functionality
	const toggleTimer = () => {
		if (running) {
			clearInterval(timer);
			running = false;
			paused = true;
		} else {
			running = true;
			paused = false;
			if (timeRemaining <= 0) return;

			timer = setInterval(() => {
				if (timeRemaining <= 0) {
					clearInterval(timer);
					expired = true;
				} else {
					timeRemaining--;
				}
			}, 1000);
		}
	};

	const reset = () => {
		clearInterval(timer);
		timeRemaining = time;
		running = false;
		paused = false;
		expired = false;
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

	const setTimerType = (newType: string) => {
		reset();

		timerType = newType;

		// Store new time based on timer type
		const newTime = timerTypes.get(timerType);
		if (newTime) time = newTime;
		timeRemaining = time;
	};

	const setButtonText = (): string => {
		if (running) {
			return 'Pause';
		} else if (paused) {
			return 'Resume';
		} else if (expired) {
			return 'Reset';
		} else {
			return 'Start';
		}
	};

	// TODO: Auto advance pomodoro (WIP) - want to auto advance to short break, back to pomodoro, and then long break after a user-defined number of cycles has been completed

	onMount(() => {
		joystick.register(entry, {
			up: {
				keep: true,
				action: () => navigate('up')
			},
			down: {
				keep: true,
				action: () => navigate('down')
			},
			left: {
				keep: true,
				action: () => navigate('left')
			},
			right: {
				keep: true,
				action: () => navigate('right')
			},
			exit: {}
		});
	});
</script>

<div id="pomodoro">
	<p id="title"><strong>Focus Module:</strong></p>
	<div>
		<button class:selected={currentSelectedButton === 'pomodoro'} class:active={timerType === 'pomodoro'}> Pomodoro </button>
		<button class:selected={currentSelectedButton === 'short break'} class:active={timerType === 'short break'}> Short Break </button>
		<button
			class:selected={currentSelectedButton === 'long break'}
			class:active={timerType === 'long break'}
		>
			Long Break
		</button>
	</div>
	<span id="timer">{formatTime(timeRemaining)}</span>
	<br />
	<div id="timer_controls">
		<button id="start_pause" class:selected={currentSelectedButton === 'start'}
			>{setButtonText()}</button
		>
		<button id="reset" class:selected={currentSelectedButton === 'reset'}
			><Icon icon={'lucide:timer-reset'} inline /></button
		>
	</div>
</div>

<style>
	button {
		cursor: pointer;
		border-radius: 5% 5%;
		border: 1px solid var(--fg);
	}

	#title {
		margin: 0;
		font-size: 1rem;
	}

	#pomodoro {
		font-size: 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}

	#mode_buttons {
		font-size: 0.71rem;
		margin-top: 0.3rem;
	}

	#mode_buttons > button {
		padding: 0.4rem;
		border-radius: 7% 5%;
		background-color: transparent;
		border: 2px solid var(--fg);
		color: var(--fg);
	}

	#timer {
		font-size: 2rem;
		margin-top: 0.4rem;
	}

	#timer_controls {
		margin-top: 0.5rem;
		width: 80%;
		display: flex;
	}

	#timer_controls > #start_pause {
		margin-right: 0.2rem;
		flex: 4;
		border-radius: 2%;
		padding: 0.35rem;
		font-size: 0.71rem;
		border: none;
	}

	#timer_controls > #reset {
		flex: 1;
		border: none;
	}

	#mode_buttons > .active {
		cursor: default;
		background-color: var(--fg);
		color: black;
	}

	.selected {
		background-color: #d6caca;
	}
</style>
