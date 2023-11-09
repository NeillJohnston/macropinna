<script lang="ts">
	import Icon from '@iconify/svelte';
	import CarouselSelector from '../../ui/CarouselSelector.svelte';
	import Button from '../../ui/Button.svelte';

	// Nav related exports
	export let id: string;
	export const entry = id + '/pomodoro/start_pause';

	const carouselID = id + '/pomodoro/carousel';
	const resetButtonID = id + '/pomodoro/reset';

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
	let startButtonText = 'Start';

	// Carousel Navigation
	const timerTypeValues = ['pomodoro', 'short break', 'long break'];
	let timerTypeIndex = 0;

	// Pomodoro Functionality
	const toggleTimer = () => {
		if (running) {
			clearInterval(timer);
			running = false;
			paused = true;
		} else {
			running = true;
			paused = false;
			if (timeRemaining > 0) {

				timer = setInterval(() => {
					if (timeRemaining <= 0) {
						expired = true;
						running = false;
						startButtonText = getButtonText();
						clearInterval(timer);
					} else {
						timeRemaining--;
					}
				}, 1000);

			} else {
				expired = true;
				running = false;
			}
		}
		startButtonText = getButtonText();
	};

	const reset = () => {
		clearInterval(timer);
		timeRemaining = time;
		running = false;
		paused = false;
		expired = false;
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

		timerType = timerTypeValues[timerTypeIndex];

		// Store new time based on timer type
		const newTime = timerTypes.get(timerType);
		if (newTime) time = newTime;
		timeRemaining = time;
	};

	const getButtonText = (): string => {
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

	const handleStartButtonPress = () => {
		if (expired) {
			reset();
		} else {
			toggleTimer();
		}
	};

	// TODO: Auto advance pomodoro (WIP) - want to auto advance to short break, back to pomodoro, and then long break after a user-defined number of cycles has been completed
</script>

<div id="pomodoro">
	<p id="title"><strong>Focus Module:</strong></p>
	<div id="mode_buttons">
		<CarouselSelector
			id={carouselID}
			component={{
				down: { id: entry },
				exit: {}
			}}
			onChange={setTimerType}
			bind:index={timerTypeIndex}
			values={timerTypeValues}
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
				exit: {}
			}}
			onPress={reset}
		>
			<Icon icon="lucide:timer-reset"></Icon>
		</Button>
	</div>
</div>

<style>
	#title {
		margin: 0;
		font-size: var(--f0);
	}

	#pomodoro {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}

	#mode_buttons {
		font-size: var(--f-1);
		margin-top: var(--sm);
	}

	#timer {
		font-size: var(--f2);
		margin-bottom: var(--sm);
	}
</style>
