<script lang="ts">
    import Name from '$lib/Name.svelte';

    export let name: boolean = false;
</script>

{#if name}
    <Name />
{:else}
    <!-- <img src="" alt="Project Logo" /> -->
{/if}
