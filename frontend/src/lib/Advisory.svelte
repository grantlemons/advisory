<script lang="ts">
    import type { Advisory, Student, Teacher } from '$lib/DBTypes';
    import Card, { Content } from '@smui/card';
    import List, { Item, Separator, Text } from '@smui/list';
    export let data: Advisory;

    let teachers: Teacher[] = data.advisors;
    let students: Student[] = data.students;
</script>

<div class="advisory-card">
    <Card>
        <Content>
            <List nonInteractive>
                {#each teachers as teacher}
                    <Item>
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

<style>
    .advisory-card {
        font-family: Roboto;
        /* border-radius: 12px; */
        /* background-color: rgb(199, 199, 199); */
    }
</style>
