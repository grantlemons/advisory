<script lang="ts">
    import type { Settings, Student, Teacher, Advisory } from '$lib/DBTypes';

    import SideBar from '$lib/SideBar.svelte';
    import BottomBar from '$lib/BottomBar.svelte';
    import TopBar from '$lib/TopBar.svelte';
    import AdvisoryWindow from '$lib/AdvisoryWindow.svelte';

    import API from '$lib/API';
    import { sets_from_table } from '$lib/TableParsing';

    let files: FileList | undefined;
    let settings: Settings;
    let teacher_pairs: [Teacher, Teacher][] = [
        [{ name: 'Garcia' }, { name: 'Downes' }],
        [{ name: 'Hardy' }, { name: 'Sims' }],
        [{ name: 'Bobbit' }, { name: 'Gross' }],
        [{ name: 'Mir' }, { name: 'Fleisher' }],
        [{ name: 'Hesseltine' }, { name: 'McGarvey' }],
        [{ name: 'Doongaji' }, { name: 'Sim' }],
        [{ name: 'Li' }, { name: 'Lundberg' }],
        [{ name: 'Curiel' }, { name: 'Wessels' }],
    ];
    let advisories: Advisory[] = [];

    function generate() {
        API.get_advisories(teacher_pairs, settings.weights).then((response) => {
            const { data } = response;
            advisories = data;
        });
    }

    function clear() {
        API.clean_database();
    }

    async function import_doc(files: FileList) {
        for (let index = 0; index < files.length; index += 1) {
            const file = files.item(index) as File;
            const buffer = await file.arrayBuffer();
            const sets: [Set<Teacher>, Set<Student>] = sets_from_table(buffer);
            API.add_teachers_bulk(Array.from(sets[0]));
            API.add_students_bulk(Array.from(sets[1]));
        }
    }
    $: if (files) {
        import_doc(files);
    }
</script>

<div class="page">
    <div class="top-bar">
        <TopBar />
    </div>
    <div class="content">
        <div class="left-content">
            <SideBar bind:settings />
        </div>
        <div class="right-content">
            <AdvisoryWindow bind:advisories />
        </div>
    </div>
    <div class="bottom-bar">
        <BottomBar bind:files on:clear={clear} on:generate={generate} />
    </div>
</div>

<style>
    .page {
        display: flex;
        flex-flow: column;

        height: 100vh;
        width: 100vw;
    }
    .top-bar {
        flex: top;
    }
    .content {
        flex-grow: 1;

        display: flex;
        flex-flow: row;
        justify-content: space-between;
    }
    .bottom-bar {
        flex: bottom;
    }
    .left-content {
        flex: left;
        height: 100%;
        width: fit-content;
    }
    .right-content {
        flex: right;
        height: 100%;
        flex-grow: 1;
    }
</style>
