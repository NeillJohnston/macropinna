<script lang="ts">
	// Todo: Unhardcode times and pull from config.json so user can customize length of Pomodoro modes in settings
	import Icon from '@iconify/svelte';

	let timerType = 'pomodoro';
	let pomodoroCount = 0;
	let time = 1500; // Default value for seconds, renders with Pomodoro timer type first
	let timer: number;

	// Map to store Pomodoro lengths in seconds, need to make this configurable in settings
	const timerTypes = new Map<string, number>([
		['pomodoro', 1500],
		['short break', 300],
		['long break', 900]
	]);

	// Timer related variables and functions
	let timeRemaining = time;
	let running = false;

	const start = () => {
		running = true;
		if (timeRemaining <= 0) return;

		timer = setInterval(() => {
			if (timeRemaining <= 0) {
				clearInterval(timer);
			} else {
				timeRemaining--;
			}
		}, 1000);
	};

	const pause = () => {
		if (!running) return;
		clearInterval(timer);
		running = false;
	};

	const reset = () => {
		clearInterval(timer);
		timeRemaining = time;
		running = false;
	};

	// Borrowed function to format time to HH:MM:SS
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

	// Pomodoro specific functions
	const setTimerType = (newType: string) => {
		// Pause timer on switch
		if (running) reset();

		// Set new timer type based on UI
		if (newType === 'pomodoro') {
			timerType = 'pomodoro';
		} else if (newType === 'short break') {
			timerType = 'short break';
		} else {
			timerType = 'long break';
		}

		// Store new time based on timer type
		const newTime = timerTypes.get(timerType);
		if (newTime) time = newTime;
		timeRemaining = time;
	};

	// Auto advance pomodoro (WIP) - want to auto advance to short break, back to pomodoro, and then long break after a user-defined number of cycles has been completed
	// const advancePomodoro = () => {
	// 	// Advance by one Pomodoro, reset after 4
	// 	if (pomodoroCount < 4) {
	// 		if (pomodoroType === 'pomodoro') {
	// 			pomodoroType = 'short break';
	// 		} else if (pomodoroType === 'short break') {
	// 			pomodoroCount++;
	// 			pomodoroType = 'pomodoro';
	// 		}
	// 	} else {
	// 		if (pomodoroType === 'long break') {
	// 			pomodoroCount = 0;
	// 			pomodoroType = 'pomodoro';
	// 		} else {
	// 			pomodoroType = 'long break';
	// 		}
	// 	}

	// 	time = pomodoroTypes.get(pomodoroType) ?? 1500;
	// 	timeRemaining = time;
	// };
</script>

<div id="pomodoro" class="container">
	<p><strong>Focus Module</strong></p>
	<div id="mode_buttons">
		<button
			on:click={() => setTimerType('pomodoro')}
			class:active={timerType === 'pomodoro'}
			disabled={timerType === 'pomodoro'}
		>
			Pomodoro
		</button>
		<button
			on:click={() => setTimerType('short break')}
			class:active={timerType === 'short break'}
			disabled={timerType === 'short break'}
		>
			Short Break
		</button>
		<button
			on:click={() => setTimerType('long break')}
			class:active={timerType === 'long break'}
			disabled={timerType === 'long break'}
		>
			Long Break
		</button>
	</div>
	<span id="timer">{formatTime(timeRemaining)}</span>
	<br />
	<div id="timer_controls">
		{#if !running}
			<button id="start_pause" on:click={start}>Start</button>
		{:else}
			<button id="start_pause" on:click={pause}>Pause</button>
		{/if}
		<button id="reset" on:click={reset}><Icon icon={'lucide:timer-reset'} inline /></button>
	</div>
</div>

<style>
	p {
		text-align: center;
		margin: 0;
		font-size: 1.41rem;
	}

	button {
		cursor: pointer;
		margin: 0 0.1rem;
		border-radius: 5% 5%;
		border: 1px solid #fff;
	}

	#pomodoro {
		height: 100%;
		width: 100%;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: space-evenly;
	}

	#mode_buttons {
		margin: 0.25rem 0 0.7rem;
	}

	#mode_buttons > button {
		padding: 0.4rem;
		border-radius: 7% 5%;
		background-color: transparent;
		border: 1px solid #fff;
		color: #fff;
	}

	#timer {
		font-size: 2rem;
	}

	#timer_controls {
		width: 80%;
		display: flex;
		justify-content: space-between;
	}

	#timer_controls > #start_pause {
		flex: 5;
		border-radius: 2%;
		padding: 0.5rem;
		font-size: 0.71rem;
		border: none;
	}

	#timer_controls > #reset {
		flex: 1;
		border: none;
	}

	#mode_buttons > .active {
		cursor: default;
		background-color: #fff;
		color: black;
	}

	button:hover {
		background-color: #d3d3d3;
	}
</style>
