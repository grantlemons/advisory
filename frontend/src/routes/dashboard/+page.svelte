<script lang="ts">
    import type { Settings, Student, Teacher, Advisory } from '$lib/DBTypes';

    import SideBar from '$lib/SideBar.svelte';
    import BottomBar from '$lib/BottomBar.svelte';
    import TopBar from '$lib/TopBar.svelte';
    import AdvisoryWindow from '$lib/AdvisoryWindow.svelte';

    import API from '$lib/API';
    import { sets_from_table } from '$lib/TableParsing';

    let files: FileList | undefined;
    let settings: Settings = {
        weights: {
            has_teacher: 0,
            sex_diverse: 0,
            grade_diverse: 0,
        },
        num_advisories: 5,
        teacher_groupings: [],
    };

    let unallocated_teachers: Teacher[] = [];
    let advisories: Advisory[] = [];

    function update_advisories(data: Advisory[]) {
        for (let advisory in data) {
            advisories
                .map((a) => a.advisors)
                .filter(
                    (a) => a === (advisory as any as Advisory).advisors.sort()
                )
                .forEach((_) => advisory);
        }
        console.log(advisories);
    }

    function generate() {
        let teacher_groupings = advisories.map((a) => a.advisors);
        if (teacher_groupings.length != settings.num_advisories) {
            return;
        }
        API.get_advisories(teacher_groupings, settings.weights).then(
            (response) => {
                const { data } = response;
                update_advisories(data);
            }
        );
    }

    function clear() {
        API.clean_database();
        unallocated_teachers = [];
        advisories = [];
    }

    function load() {
        API.list_teachers().then((response) => {
            const { data } = response;
            unallocated_teachers = data;
        });
    }

    async function import_doc(files: FileList) {
        for (let index = 0; index < files.length; index += 1) {
            const file = files.item(index) as File;
            const buffer = await file.arrayBuffer();
            const sets: [Set<Teacher>, Set<Student>] = sets_from_table(buffer);
            advisories = [];
            unallocated_teachers = Array.from(sets[0]);
            API.add_teachers_bulk(Array.from(sets[0]));
            API.add_students_bulk(Array.from(sets[1]));
        }
    }

    $: if (files) {
        import_doc(files);
    }
    // force reactivity for number of advisories
    $: {
        while (settings.num_advisories > advisories.length) {
            advisories.push({
                students: [],
                advisors: [],
            });
        }
        while (settings.num_advisories < advisories.length) {
            advisories.pop();
        }
        advisories = advisories;
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
            <AdvisoryWindow bind:advisories bind:unallocated_teachers />
        </div>
    </div>
    <div class="bottom-bar">
        <BottomBar
            bind:files
            on:clear={clear}
            on:generate={generate}
            on:load={load}
        />
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
