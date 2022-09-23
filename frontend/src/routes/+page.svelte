<script>
    import Button, { Label } from '@smui/button';
    let primary_count = 0;
    let secondary_count = 0;
</script>

<Button color="primary" on:click={() => primary_count++} variant="unelevated">
    <Label>Click to increment: {primary_count}</Label>
</Button>
<Button
    color="secondary"
    on:click={() => secondary_count++}
    variant="unelevated"
>
    <Label>Click to increment: {secondary_count}</Label>
</Button>
