<script lang="ts">
    import type { Advisory, Student, Teacher } from '$lib/DBTypes';
    import Card, { Content } from '@smui/card';
    import List, { Graphic, Item, Separator, Text } from '@smui/list';
    import Dialog, { Actions } from '@smui/dialog';
    import Button, { Label } from '@smui/button';
    import Radio from '@smui/radio';
    export let advisories: Advisory[] = [];
    export let data: Advisory;

    let teachers: Teacher[] = data.advisors;
    let students: Student[] = data.students;

    let open = false;
    let dialog_teacher = {};
    let selection: Advisory;

    function closeHandler(e: CustomEvent<{ action: string }>) {
        if (e.detail.action === 'accept') {
            // selection.teachers.push(dialog_teacher);
        }
    }

    function select_teacher(teacher: Teacher) {
        dialog_teacher = teacher;
        open = true;
    }
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
                        <Radio bind:group={selection} value={advisory} />
                    </Graphic>
                    <Text>Advisory {index + 1}</Text>
                </Item>
            {/each}
        </List>
    </Content>
    <Actions>
        <Button>
            <Label>Cancel</Label>
        </Button>
        <Button action="accept">
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
