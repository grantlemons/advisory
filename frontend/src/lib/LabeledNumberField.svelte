<script lang="ts">
    import TextField from '@smui/textfield';

    export let value: string | number = '';
    export let label: string = 'Placeholder Label';
    export let min: number = 0;
    export let max: number = 10;
</script>

<div class="container">
    <div class="label">{label}</div>
    <div class="input-box">
        <TextField
            bind:value
            variant="outlined"
            type="number"
            label="{min}&ndash{max}"
            {min}
            {max}
        />
    </div>
</div>

<style>
    .container {
        border-radius: 6px;
        background-color: #c1c1c1;
        width: 100%;
        display: grid;
        min-width: 263px;
    }
    .label {
        grid-area: 1/1;
        margin-top: auto;
        margin-bottom: auto;
        margin-left: 8px;

        color: #313131;
    }
    .input-box {
        grid-area: 1/2;
        background-color: #b4b4b4;
        width: 100px;
        margin-left: auto;
        margin-right: 0px;
    }
</style>
