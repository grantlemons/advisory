<script lang="ts">
    let advisory_count = 0;

    let weights = {
        teacher: 0,
        grade_diversity: 0,
        gender_diversity: 0,
    };

    function generate() {}
    function clear() {}
    function import_doc() {}
</script>

<div class="page">
    <div class="topbar flex" />
    <div class="contents" />
    <div class="bottombar flex" />
    <div class="left flex" />
</div>

<style>
    .page {
        display: flex;
        flex-flow: column;
        height: 100vh;
        width: 100vw;
    }

    .flex {
        display: flex;
        flex-direction: column;
    }
    .hori_center {
        justify-content: center;
        text-align: center;
    }
    .vert_center {
        align-items: center;
        text-align: center;
    }

    /* top bar */
    .topbar {
        flex: 0 1 auto;

        background-color: #ededed;
        flex-direction: row;
        width: 100vw;
        height: 8vh;
        min-height: 76px;
    }

    /* left side */
    .left {
        position: absolute;
        left: 0;
        background-color: #939393;

        width: 22vw;
        min-width: 410px;
        height: 100%;
    }

    /* main content */
    .contents {
        flex: 1 1 auto;
        background-color: aliceblue;
    }

    /* bottom bar */
    .bottombar {
        flex: 0 1 auto;

        background-color: #c1c1c1;
        flex-direction: row;
        width: 100vw;
        height: 8vh;
        min-height: 76px;
    }
</style>
