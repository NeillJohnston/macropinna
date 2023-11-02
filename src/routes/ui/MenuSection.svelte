<script lang="ts">
    export let label: string;
</script>

<div id="menu-section">
    <p id="label"><strong>{label}</strong></p>
    <div id="menu">
        <slot />
    </div>
</div>

<style>
    #menu-section {}

    #label {
        font-size: 0.71rem;
    }

    #menu {
        padding-left: var(--lg);
    }
</style>