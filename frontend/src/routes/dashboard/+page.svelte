<script lang="ts">
    import SideBar from '$lib/SideBar.svelte';
    import BottomBar from '$lib/BottomBar.svelte';
    import TopBar from '$lib/TopBar.svelte';
    import AdvisoryWindow from '$lib/AdvisoryWindow.svelte';

    function generate() {}
    function clear() {}
    function import_doc() {}
</script>

<div class="page">
    <TopBar />
    <AdvisoryWindow />
    <BottomBar />
    <SideBar />
</div>

<style>
    .page {
        display: flex;
        flex-flow: column;
        height: 100vh;
        width: 100vw;
    }
</style>
