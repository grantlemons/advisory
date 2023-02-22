<script lang="ts">
    /* cspell: disable */
    import type { Student, Teacher, Weights } from '$lib/DBTypes';
    import Button from '$lib/Button.svelte';
    import { sets_from_table } from '$lib/TableParsing';
    import Input from '$lib/Input.svelte';
    import API from '$lib/API';

    let data: string;
    let files: FileList | undefined;

    const setting_strings = {
        weights: {
            has_teacher: '8',
            sex_diverse: '5',
            grade_diverse: '5',
        },
        num_advisories: '8',
    };

    async function test_add_xlsx(files: FileList) {
        for (let index = 0; index < files.length; index += 1) {
            const file = files.item(index) as File;
            const buffer = await file.arrayBuffer();
            const sets: [Set<Teacher>, Set<Student>] = sets_from_table(buffer);
            API.add_students_bulk(Array.from(sets[1]));
            API.add_teachers_bulk(Array.from(sets[0]));
        }
    }
    $: if (files) {
        test_add_xlsx(files);
    }

    function get_advisories() {
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

        let weights: Weights = {
            has_teacher: parseInt(setting_strings.weights.has_teacher),
            sex_diverse: parseInt(setting_strings.weights.sex_diverse),
            grade_diverse: parseInt(setting_strings.weights.has_teacher),
        };

        API.get_advisories(teacher_pairs, weights);
    }
</script>

<div>
    <Button on:click={API.clean_database} label="Clean Database" />
    <Button on:click={API.list_people} label="List People" />
    <input
        bind:files
        accept=".csv, application/vnd.openxmlformats-officedocument.spreadsheetml.sheet, application/vnd.ms-excel"
        type="file"
    />
    <Input
        bind:value={setting_strings.num_advisories}
        label="Number of Advisories"
    />
    <Input
        bind:value={setting_strings.weights.has_teacher}
        label="Has-Teacher Weight"
    />
    <Input
        bind:value={setting_strings.weights.sex_diverse}
        label="Sex Diversity Weight"
    />
    <Input
        bind:value={setting_strings.weights.grade_diverse}
        label="Grade Diversity Weight"
    />
    <Button on:click={get_advisories} label="Get Advisories" />

    {#if data}
        {JSON.stringify(data)}
    {/if}
</div>

<style>
    div {
        display: flex;
        flex-direction: column;
        row-gap: 8px;
    }
    div {
        width: 50%;
        max-width: 800px;
        margin: auto;
    }
</style>
