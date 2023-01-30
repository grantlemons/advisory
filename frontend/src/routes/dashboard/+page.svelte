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
    <div class="top-bar">
        <TopBar />
    </div>
    <div class="content">
        <div class="left-content">
            <SideBar />
        </div>
        <div class="right-content">
            <AdvisoryWindow />
        </div>
    </div>
</div>

<style>
    .page {
        background-color: brown;
        display: flex;
        flex-flow: column;

        height: 100vh;
        width: 100vw;
    }
    .top-bar {
        flex: top;
    }
    .content {
        flex: bottom;

        display: flex;
        flex-flow: row;
        justify-content: space-between;
    }
    .left-content {
        flex: left;
    }
    .right-content {
        flex: right;
    }
</style>
