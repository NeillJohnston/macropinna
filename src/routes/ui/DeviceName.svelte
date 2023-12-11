<script lang="ts">
	import type { Agent } from "$lib/api";
	import Icon from "@iconify/svelte";

    export let name: string;
    export let agent: Agent;

    const icon = {
        Android: 'ic:baseline-phone-android',
        IPhone: 'ic:baseline-phone-iphone',
        Desktop: 'ic:baseline-laptop',
        Unknown: 'ic:baseline-device-unknown'
    }[agent];
</script>

<Icon icon={icon} inline /> {name}