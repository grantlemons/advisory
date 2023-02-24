<script lang="ts">
    import type { Advisory, Student, Teacher } from '$lib/DBTypes';
    export let data: Advisory;

    let teachers: Teacher[] = data.advisors;
    let students: Student[] = data.students;
</script>

<div class="advisory-card">
    ({teachers.map((t) => t.name).join(', ')})
    {#each students as student}
        <div>
            {student.name}
        </div>
    {/each}
</div>

<style>
    .advisory-card {
        border-radius: 12px;
        background-color: rgb(199, 199, 199);
    }
</style>
