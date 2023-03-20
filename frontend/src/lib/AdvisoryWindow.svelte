<script lang="ts">
    import type { Advisory as AdvisoryType, Teacher } from '$lib/DBTypes';
    import Card, { Content } from '@smui/card';
    import List, { Item, Text } from '@smui/list';
    import Advisory from '$lib/Advisory.svelte';
    import API from '$lib/API';

    export let unallocated_teachers: Teacher[] = [];
    export let advisories: AdvisoryType[] = [];
</script>

<div class="advisory-window">
    {#if unallocated_teachers.length != 0}
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
    {/if}
    <div class="advisories">
        {#each advisories as advisory}
            <Advisory bind:data={advisory} bind:advisories />
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
    .unallocated {
        width: 250px;
        height: 100%;
    }
    div {
        height: 100%;
        margin: 8px;
    }
</style>
