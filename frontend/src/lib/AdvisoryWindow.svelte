<script lang="ts">
    import Advisory from '$lib/Advisory.svelte';
    import type { Advisory as AdvisoryType } from '$lib/DBTypes';

    export let advisories: AdvisoryType[] = [];
</script>

<div class="advisory-window">
    {#each advisories as advisory}
        <Advisory bind:data={advisory} />
    {/each}
</div>

<style>
    .advisory-window {
        height: 100%;
        display: flex;
        flex-flow: wrap;
        justify-content: stretch;
    }
</style>
