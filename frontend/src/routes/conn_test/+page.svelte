<script lang="ts">
    /* cspell: disable */
    import {
        type Student,
        type Teacher,
        type Weights,
        type Settings,
        Sex,
        Grade,
    } from '$lib/DBTypes';
    import { id_token } from '$lib/auth_store';
    import Button from '$lib/Button.svelte';
    import axios from 'axios';
    import { sets_from_table } from '$lib/TableParsing';
    import Input from '$lib/Input.svelte';

    let data: string;
    let files: FileList | undefined;

    let num_advisories: string = '10';
    const settings: Settings = {
        weights: {
            has_teacher: 8,
            sex_diverse: 5,
            grade_diverse: 5,
        },
        num_advisories: 8,
    };

    const BASE_URL = '/api';
    let auth: string;
    id_token.subscribe((value) => {
        auth = value;
    });

    const WESSELS: Teacher = {
        name: 'Mark Wessels',
    };
    const DOWNES: Teacher = {
        name: 'Edward Downes',
    };
    const LUNDBERG: Teacher = {
        name: 'Matthew Lundberg',
    };
    const FLEISHER: Teacher = {
        name: 'Gregg Fleisher',
    };
    const HESSELTINE: Teacher = {
        name: 'Ashley Hesseltine',
    };
    const SIMS: Teacher = {
        name: 'Holly Sims',
    };

    function clean_database() {
        axios
            .delete(`${BASE_URL}/people`, {
                headers: {
                    Authorization: auth,
                },
            })
            .then((_) => (data = ''));
    }
    function add_teacher(teacher: Teacher) {
        axios({
            method: 'post',
            url: `${BASE_URL}/people/teacher`,
            data: teacher,
            headers: {
                Authorization: auth,
            },
        });
    }
    function add_teachers_bulk(teachers: Teacher[]) {
        axios({
            method: 'post',
            url: `${BASE_URL}/people/teacher/bulk`,
            data: teachers,
            headers: {
                Authorization: auth,
            },
        });
    }
    function add_student(student: Student) {
        axios({
            method: 'post',
            url: `${BASE_URL}/people/student`,
            data: student,
            headers: {
                Authorization: auth,
            },
        });
    }
    function add_students_bulk(students: Student[]) {
        axios({
            method: 'post',
            url: `${BASE_URL}/people/student/bulk`,
            data: students,
            headers: {
                Authorization: auth,
            },
        });
    }
    function list_people() {
        axios({
            method: 'get',
            url: `${BASE_URL}/people`,
            headers: {
                Authorization: auth,
            },
        }).then((res) => (data = res.data));
    }
    function test_add_grant() {
        let teachers: Teacher[] = [
            WESSELS,
            DOWNES,
            LUNDBERG,
            FLEISHER,
            HESSELTINE,
            SIMS,
        ];

        add_teachers_bulk(teachers);
        add_student({
            name: 'Grant Lemons',
            sex: Sex.Male,
            teachers,
            grade: Grade.Senior,
        });
    }
    async function test_add_xlsx() {
        if (files != undefined) {
            for (let index = 0; index < files.length; index += 1) {
                const file = files.item(index) as File;
                const buffer = await file.arrayBuffer();
                const sets: [Set<Teacher>, Set<Student>] =
                    sets_from_table(buffer);
                add_teachers_bulk(Array.from(sets[0]));
                add_students_bulk(Array.from(sets[1]));
            }
        }
    }
    $: if (files) {
        test_add_xlsx();
    }
    function get_advisories() {
        settings.num_advisories = parseInt(num_advisories);
        axios({
            method: 'put',
            url: `${BASE_URL}/`,
            data: settings,
            headers: {
                Authorization: auth,
            },
        }).then((res) => (data = res.data));
    }
</script>

<div>
    <Button on:click={clean_database} label="Clean Database" />
    <Button on:click={test_add_grant} label="Create Grant" />
    <Button on:click={list_people} label="List People" />
    <input
        bind:files
        accept=".csv, application/vnd.openxmlformats-officedocument.spreadsheetml.sheet, application/vnd.ms-excel"
        type="file"
    />
    <Button on:click={get_advisories} label="Get Advisories" />

    <form><Input bind:value={num_advisories} /></form>

    {#if data}
        {JSON.stringify(data)}
    {/if}
</div>

<style>
    div {
        width: 50%;
        margin: auto;
    }
</style>
