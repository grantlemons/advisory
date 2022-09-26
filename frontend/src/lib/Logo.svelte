<script lang="ts">
    import Name from '$lib/Name.svelte';

    export let name: boolean = false;
</script>

{#if name}
    <img src="" alt="Project Logo" />
    <Name />
{:else}
    <img src="" alt="Project Logo" />
{/if}

<style>
    img {
        background-color: #939393;
        border-radius: 30px;
        padding: 40px;
        padding-left: 80px;
        padding-right: 80px;
        width: 100%;
        font-family: Roboto;
        font-size: 200%;
    }
</style>
