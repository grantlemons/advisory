<script lang="ts">
    import TextField from '@smui/textfield';

    export let value: string = '';
    export let label: string = 'Default Label';
    export let password: boolean = false;
    export let error_text: string = '';
</script>

<TextField
    style="width: 100%"
    variant="outlined"
    type={password ? 'password' : 'text'}
    invalid={error_text != ''}
    bind:value
    {label}
/>
{#if error_text != ''}
    <p class="error">{error_text}</p>
{/if}

<style>
    .error {
        color: red;
        font-size: smaller;
        margin: 0;
        font-family: Roboto;
    }
</style>
