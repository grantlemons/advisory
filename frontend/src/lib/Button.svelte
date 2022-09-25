<script lang="ts">
    import Button, { Label } from '@smui/button';

    export let label = 'Default Label';
</script>

<div class="button">
    <Button color="secondary" on:click variant="unelevated">
        <Label>{label}</Label>
    </Button>
</div>

<style>
    .button {
        display: flex;
        flex-direction: column;
        justify-content: center;
        row-gap: 10px;

        width: 100%;
    }
</style>
