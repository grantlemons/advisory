<script>
    import Button, { Label } from '@smui/button';
    import Card from '@smui/card';
    let count = 0;
</script>

<Button color="primary" on:click={() => count++} variant="unelevated">
    <Label>Press To Increment: {count}</Label>
</Button>
<Button color="secondary" on:click={() => count++} variant="unelevated">
    <Label>Press To Increment: {count}</Label>
</Button>
