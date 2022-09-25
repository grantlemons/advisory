<script lang="ts">
    import TextField from '@smui/textfield';

    export let value: string = '';
    export let label: string = 'Default Label';
</script>

<TextField style="width: 100%" variant="outlined" bind:value {label} />
