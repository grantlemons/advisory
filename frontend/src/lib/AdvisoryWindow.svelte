<script lang="ts">
    import type {
        Advisory as AdvisoryType,
        Student,
        Teacher,
    } from '$lib/DBTypes';
    import Card, { Content } from '@smui/card';
    import List, { Item, Text } from '@smui/list';
    import Advisory from '$lib/Advisory.svelte';

    export let unallocated_teachers: Teacher[] = [];
    export let advisories: AdvisoryType[] = [];
</script>

<div class="advisory-window">
    <div class="unallocated">
        <Card>
            <Content>
                <List>
                    {#each unallocated_teachers as teacher}
                        <Item>
                            <Text>{teacher.name}</Text>
                        </Item>
                    {/each}
                </List>
            </Content>
        </Card>
    </div>
    <div class="advisories">
        {#each advisories as advisory}
            <Advisory bind:data={advisory} />
        {/each}
    </div>
</div>

<style>
    .advisory-window {
        height: 100%;
        display: flex;
        flex-flow: row wrap;
        justify-content: stretch;
    }
    .advisories {
        display: flex;
        flex-flow: row wrap;
        justify-content: stretch;
    }
    div {
        height: 100%;
        margin: 8px;
    }
</style>
