<script lang="ts">
    /* cspell: disable */
    import { type Student, type Teacher, Sex, Grade } from '$lib/DBTypes';
    import { id_token } from '$lib/auth_store';
    import Button from '$lib/Button.svelte';
    import axios from 'axios';
    import StudentCard from '$lib/StudentCard.svelte';
    import { sets_from_table } from '$lib/TableParsing';

    let data: string;

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
    function test_add_xlsx() {
        let sets: [Set<Teacher>, Set<Student>] = sets_from_table();
        data = JSON.stringify(sets[0]) + JSON.stringify(sets[1]);
    }
</script>

<div>
    <Button on:click={clean_database} label="Clean Database" />
    <Button on:click={test_add_grant} label="Create Grant" />
    <Button on:click={list_people} label="List People" />
    <Button on:click={test_add_xlsx} label="Add from XLSX" />

    {#if data}
        {data}
    {/if}
</div>

<style>
    div {
        width: 50%;
        margin: auto;
    }
</style>
