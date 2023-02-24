<script lang="ts">
    import type { Advisory } from '$lib/DBTypes';
    export let data: Advisory;
</script>

{#each data.students as student}
    <div>
        {JSON.stringify(student)}
    </div>
{/each}
