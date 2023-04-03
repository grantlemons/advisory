<script lang="ts">
    import Button, { Label } from '@smui/button';
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    export let files: FileList | undefined;

    const clear = () => dispatch('clear');
    const generate = () => dispatch('generate');
    const load = () => dispatch('load');
</script>

<div class="bottom-bar">
    <div class="buttonbox">
        <div class="button">
            <input
                bind:files
                accept=".csv, application/vnd.openxmlformats-officedocument.spreadsheetml.sheet, application/vnd.ms-excel"
                type="file"
            />
        </div>
        <div class="button">
            <Button color="primary" on:click={clear} variant="unelevated">
                <Label>Clear</Label>
            </Button>
        </div>
        <div class="button">
            <Button color="primary" on:click={load} variant="unelevated">
                <Label>Fetch</Label>
            </Button>
        </div>
        <div class="button">
            <Button color="secondary" on:click={generate} variant="unelevated">
                <Label>Generate</Label>
            </Button>
        </div>
    </div>
</div>

<style>
    .bottom-bar {
        flex: 0 1 auto;

        background-color: #d9d9d9;
        width: 100%;
        height: 55px;
        display: flex;
        flex-direction: row-reverse;
    }
    .button {
        height: 70%;
        margin: auto;
    }
    .buttonbox {
        display: flex;
        flex-direction: row;
        column-gap: 10px;
        width: fit-content;
        margin-right: 10px;
    }
</style>
