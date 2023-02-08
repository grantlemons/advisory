<script lang="ts">
    import { type Student, type Teacher, Sex, Grade } from '$lib/DBTypes';

    export const data: Student = {
        name: 'First Last',
        teachers: [],
        sex: Sex.Male,
        grade: Grade.Freshman,
    };
</script>

{JSON.stringify(data)}
