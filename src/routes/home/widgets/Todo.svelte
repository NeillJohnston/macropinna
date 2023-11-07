<script lang="ts">
	import type { XAlign } from '$lib/layout';
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';
	import { joystick, nav } from '$lib/joystick';

	export let props: {
		xAlign: XAlign;
	};

	// Nav related exports
	export let id: string;
	export const entry = id + '/todo';

	let selectedItemIndex = 0;

	export const up = () => {
		if (selectedItemIndex === 0) {
			selectedItemIndex = list.length - 1;
		} else {
			selectedItemIndex--;
		}
	};

	export const down = () => {
		if (selectedItemIndex === list.length - 1) {
			selectedItemIndex = 0;
		} else {
			selectedItemIndex++;
		}
	};

	const list = [
		{
			index: 0,
			task: 'Argue with strangers on the internet',
			done: true
		},
		{
			index: 1,
			task: 'Make this component',
			done: false
		},
		{
			index: 2,
			task: 'Implement navigation',
			done: false
		}
	];

	onMount(() => {
		joystick.register(entry, {
			up: {
				keep: true,
				action: up
			},
			down: {
				keep: true,
				action: down
			},
			enter: {
				keep: true,
				action: () => {
					list[selectedItemIndex].done = !list[selectedItemIndex].done;
				}
			},
			exit: {
				action: () => {
					selectedItemIndex = 0;
				}
			}
		});
	});
</script>

<div id="todo">
	<p id="title"><strong>Today's Tasks:</strong></p>
	<ul>
		{#each list as item}
			<p>
				{#if $nav === entry}
					<span class="select-border" class:selected={selectedItemIndex === item.index}>
						<Icon icon={item.done ? 'carbon:checkbox-checked-filled' : 'carbon:checkbox'} inline />
					</span>
					<span class:crossed={item.done}>{item.task}</span>
				{:else}
					<Icon icon={item.done ? 'carbon:checkbox-checked-filled' : 'carbon:checkbox'} inline />
					<span class:crossed={item.done}>{item.task}</span>
				{/if}
			</p>
		{/each}
	</ul>
</div>

<style>
	#title {
		font-size: 1rem;
	}

	p {
		margin: 0;
	}

	ul {
		font-size: 0.71rem;
	}

	ul p {
		margin-bottom: 0.5em;
	}

	.crossed {
		text-decoration: line-through;
	}

	/* Open to different styling */
	.select-border {
		border: 1px solid rgba(0, 0, 0, 0);
		transition: border-color ease 0.2s;
	}

	.selected {
		box-shadow: 0 0 25px var(--fg);
		border-radius: 10px;
	}
</style>
