<script lang="ts">
    export let label: string | undefined = undefined;
</script>

<div>
    {#if label}
    <p id="label">{label}</p>
    {/if}
    <div id="menu">
        <slot />
    </div>
</div>

<style>
    #label {
        font-size: var(--f-1);
    }

    #menu {
        padding-left: var(--md);
    }
</style>