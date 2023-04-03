<script lang="ts">
    import type { Advisory, Student, Teacher, Person } from '$lib/DBTypes';
    import Card, { Content } from '@smui/card';
    import List, { Graphic, Item, Separator, Text } from '@smui/list';
    import Dialog, { Actions } from '@smui/dialog';
    import Button, { Label } from '@smui/button';
    import Autocomplete from '@smui-extra/autocomplete';
    import Radio from '@smui/radio';
    import API from '$lib/API';

    export let advisories: Advisory[] = [];
    export let unallocated_teachers: Teacher[] = [];
    export let data: Advisory;

    let teachers: Teacher[] = data.advisors;
    let students: Student[] = data.students;

    let teacher_dialog_open = false;
    let dialog_teacher: Teacher;
    let teacher_column_selection: Teacher[];

    let student_dialog_open = false;
    let dialog_student: Student;
    let student_banned_options: string[];
    let student_banned_selection: string;

    function get_people(): string[] {
        let people: string[] = [];
        for (let index = 0; index < advisories.length; index++) {
            people = people.concat(
                advisories[index].advisors.map((a) => a.name)
            );
            people = people.concat(
                advisories[index].students.map((s) => s.name)
            );
        }
        return people;
    }

    // run when move accepted
    function teacher_close_handler(e: CustomEvent<{ action: string }>) {
        if (e.detail.action === 'accept') {
            teacher_column_selection.push(dialog_teacher);
            let index = teachers.indexOf(dialog_teacher);
            teachers.splice(index, 1);
        }
        advisories = advisories;
        unallocated_teachers = unallocated_teachers;
    }

    // run when ban accepted
    function student_close_handler(e: CustomEvent<{ action: string }>) {
        if (e.detail.action === 'accept') {
            let selected: Person = {
                name: student_banned_selection,
                banned_pairings: [],
            };
            API.ban_pairing(dialog_student, selected);
        }
        student_banned_selection = '';
    }

    // run when teacher clicked to start move
    function select_teacher(teacher: Teacher) {
        dialog_teacher = teacher;
        teacher_dialog_open = true;
    }

    // run when student clicked
    function select_student(student: Student) {
        dialog_student = student;
        student_banned_options = get_people();
        student_dialog_open = true;
    }

    // reactivity on move
    $: teachers = data.advisors;
    $: students = data.students;
</script>

<div class="advisory-card">
    <Card>
        <Content>
            <List>
                {#each teachers as teacher}
                    <Item on:SMUI:action={() => select_teacher(teacher)}>
                        <Text>{teacher.name}</Text>
                    </Item>
                {/each}
            </List>
            <Separator />
            <List>
                {#each students as student}
                    <Item on:SMUI:action={() => select_student(student)}>
                        <Text>{student.name}</Text>
                    </Item>
                {/each}
            </List>
        </Content>
    </Card>
</div>

<Dialog
    bind:open={teacher_dialog_open}
    selection
    aria-labelledby="list-selection-title"
    aria-describedby="list-selection-content"
    on:SMUIDialog:closed={teacher_close_handler}
>
    <Content>
        <List radioList>
            {#each advisories as advisory, index}
                <Item>
                    <Graphic>
                        <Radio
                            bind:group={teacher_column_selection}
                            value={advisory.advisors}
                        />
                    </Graphic>
                    <Text>Advisory {index + 1}</Text>
                </Item>
            {/each}
            <Separator />
            <Item>
                <Graphic>
                    <Radio
                        bind:group={teacher_column_selection}
                        value={unallocated_teachers}
                    />
                </Graphic>
                <Text>Unallocate</Text>
            </Item>
        </List>
    </Content>
    <Actions>
        <Button color="secondary" variant="unelevated">
            <Label>Cancel</Label>
        </Button>
        <Button action="accept" color="primary" variant="unelevated">
            <Label>Move</Label>
        </Button>
    </Actions>
</Dialog>

<Dialog
    bind:open={student_dialog_open}
    selection
    aria-labelledby="list-selection-title"
    aria-describedby="list-selection-content"
    on:SMUIDialog:closed={student_close_handler}
>
    <Content>
        <Autocomplete
            options={student_banned_options}
            textfield$variant="outlined"
            bind:value={student_banned_selection}
            label="Outlined"
        />
    </Content>
    <Actions>
        <Button color="secondary" variant="unelevated">
            <Label>Cancel</Label>
        </Button>
        <Button action="accept" color="primary" variant="unelevated">
            <Label>Ban Pairing</Label>
        </Button>
    </Actions>
</Dialog>

<style>
    .advisory-card {
        font-family: Roboto;
        height: 100%;
        width: 250px;
        /* border-radius: 12px; */
        /* background-color: rgb(199, 199, 199); */
    }
</style>
