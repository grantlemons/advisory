<script lang="ts">
    import Button, { Label } from '@smui/button';

    export let label = 'Default Label';
    export let hidden = false;
</script>

{#if hidden}
    <div class="button" style="visibility: hidden; width: 0;">
        <Button variant="unelevated" />
    </div>
{:else}
    <div class="button">
        <Button color="secondary" on:click variant="unelevated">
            <Label>{label}</Label>
        </Button>
    </div>
{/if}

<style>
    .button {
        display: flex;
        flex-direction: column;
        justify-content: center;
        row-gap: 10px;

        width: 100%;
    }
</style>
