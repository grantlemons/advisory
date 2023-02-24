<script lang="ts">
    import Advisory from '$lib/Advisory.svelte';
    import type { Advisory as AdvisoryType } from '$lib/DBTypes';

    let advisories: AdvisoryType[];
</script>

<div class="advisory-window">
    {#each advisories as advisory}
        <Advisory data={advisory} />
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
