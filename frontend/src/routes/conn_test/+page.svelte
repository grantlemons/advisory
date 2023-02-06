<script lang="ts">
    /* cspell: disable */
    import { id_token } from '$lib/auth_store';
    import Button from '$lib/Button.svelte';
    import axios from 'axios';

    let data: object | null;

    const BASE_URL = 'https://localhost:81';
    let auth: string;
    id_token.subscribe((value) => {
        auth = value;
    });

    enum Sex {
        Male = 'male',
        Female = 'female',
    }
    enum Grade {
        Freshman = 'freshman',
        Sophomore = 'sophomore',
        Junior = 'junior',
        Senior = 'senior',
    }
    interface Teacher {
        name: string;
        sex: Sex;
    }
    interface Student {
        name: string;
        sex: Sex;
        teachers: Teacher[];
        grade: Grade;
    }

    const WESSELS: Teacher = {
        name: 'Mark Wessels',
        sex: Sex.Male,
    };
    const DOWNES: Teacher = {
        name: 'Edward Downes',
        sex: Sex.Male,
    };
    const LUNDBERG: Teacher = {
        name: 'Matthew Lundberg',
        sex: Sex.Male,
    };
    const FLEISHER: Teacher = {
        name: 'Gregg Fleisher',
        sex: Sex.Male,
    };
    const HESSELTINE: Teacher = {
        name: 'Ashley Hesseltine',
        sex: Sex.Female,
    };
    const SIMS: Teacher = {
        name: 'Holly Sims',
        sex: Sex.Female,
    };

    function clean_database() {
        axios
            .delete(`${BASE_URL}/people`, {
                headers: {
                    Authorization: auth,
                },
            })
            .then((_) => (data = null));
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
        }).then((res) => (data = res));
    }

    function test_add_hesseltine() {
        add_teacher(HESSELTINE);
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
</script>

<div>
    <Button on:click={clean_database} label="Clean Database" />
    <Button on:click={test_add_grant} label="Create Grant" />
    <Button on:click={test_add_hesseltine} label="Create Hesseltine" />
    <Button on:click={list_people} label="List People" />

    {#if data}
        data
    {/if}
</div>

<style>
    div {
        width: 50%;
        margin: auto;
    }
</style>
