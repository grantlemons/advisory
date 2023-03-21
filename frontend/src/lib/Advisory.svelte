<script lang="ts">
    import type { Advisory, Student, Teacher } from '$lib/DBTypes';
    import Card, { Content } from '@smui/card';
    import List, { Graphic, Item, Separator, Text } from '@smui/list';
    import Dialog, { Actions } from '@smui/dialog';
    import Button, { Label } from '@smui/button';
    import Radio from '@smui/radio';
    export let advisories: Advisory[] = [];
    export let unallocated_teachers: Teacher[] = [];
    export let data: Advisory;

    let teachers: Teacher[] = data.advisors;
    let students: Student[] = data.students;

    let open = false;
    let dialog_teacher: Teacher;
    let selection: Teacher[];

    // run when move accepted
    function closeHandler(e: CustomEvent<{ action: string }>) {
        if (e.detail.action === 'accept') {
            selection.push(dialog_teacher);
            let index = teachers.indexOf(dialog_teacher);
            teachers.splice(index, 1);
        }
        advisories = advisories;
        unallocated_teachers = unallocated_teachers;
    }

    // run when teacher clicked to start move
    function select_teacher(teacher: Teacher) {
        dialog_teacher = teacher;
        open = true;
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
            <List nonInteractive>
                {#each students as student}
                    <Item>
                        <Text>{student.name}</Text>
                    </Item>
                {/each}
            </List>
        </Content>
    </Card>
</div>

<Dialog
    bind:open
    selection
    aria-labelledby="list-selection-title"
    aria-describedby="list-selection-content"
    on:SMUIDialog:closed={closeHandler}
>
    <Content>
        <List radioList>
            {#each advisories as advisory, index}
                <Item>
                    <Graphic>
                        <Radio
                            bind:group={selection}
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
                        bind:group={selection}
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

<style>
    .advisory-card {
        font-family: Roboto;
        height: 100%;
        width: 250px;
        /* border-radius: 12px; */
        /* background-color: rgb(199, 199, 199); */
    }
</style>
