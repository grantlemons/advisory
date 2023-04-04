<script lang="ts">
    import type { Advisory as AdvisoryType, Teacher } from '$lib/DBTypes';
    import Card, { Content } from '@smui/card';
    import List, { Graphic, Item, Text } from '@smui/list';
    import Dialog, { Actions } from '@smui/dialog';
    import Button, { Label } from '@smui/button';
    import Radio from '@smui/radio';
    import Advisory from '$lib/Advisory.svelte';

    export let unallocated_teachers: Teacher[] = [];
    export let advisories: AdvisoryType[] = [];

    let open = false;
    let dialog_teacher: Teacher;
    let selection: Advisory;

    // run when move accepted
    function closeHandler(e: CustomEvent<{ action: string }>) {
        if (e.detail.action === 'accept') {
            selection.advisors.push(dialog_teacher);
            let index = unallocated_teachers.indexOf(dialog_teacher);
            unallocated_teachers.splice(index, 1);
        }
        advisories = advisories;
        unallocated_teachers = unallocated_teachers;
    }

    // run when move started
    function select_teacher(teacher: Teacher) {
        dialog_teacher = teacher;
        open = true;
    }
</script>

<div class="advisory-window">
    {#if unallocated_teachers.length != 0}
        <div class="unallocated">
            <Card style="height:100%">
                <Content>
                    <List>
                        {#each unallocated_teachers as teacher}
                            <Item
                                on:SMUI:action={() => select_teacher(teacher)}
                            >
                                <Text>{teacher.name}</Text>
                            </Item>
                        {/each}
                    </List>
                </Content>
            </Card>
        </div>
    {/if}
    <div class="advisories">
        {#each advisories as advisory}
            <Advisory
                bind:data={advisory}
                bind:advisories
                bind:unallocated_teachers
            />
        {/each}
    </div>
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
        <Button color="secondary" variant="unelevated">
            <Label>Cancel</Label>
        </Button>
        <Button action="accept" color="primary" variant="unelevated">
            <Label>Move</Label>
        </Button>
    </Actions>
</Dialog>

<style>
    .advisory-window {
        height: 97.5%;
        display: flex;
        flex-flow: row wrap;
        justify-content: stretch;
        margin: 0.5%;
    }
    .advisories {
        display: flex;
        flex-flow: row wrap;
        justify-content: stretch;
        gap: 8px;
    }
    .unallocated {
        width: 250px;
    }
</style>
