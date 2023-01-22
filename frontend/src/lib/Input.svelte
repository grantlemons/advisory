<script lang="ts">
    import TextField from '@smui/textfield';

    export let value: string = '';
    export let label: string = 'Default Label';
    export let password: boolean = false;
</script>

<TextField
    style="width: 100%"
    variant="outlined"
    type={password ? 'password' : 'text'}
    bind:value
    {label}
/>
