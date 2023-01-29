<script lang="ts">
    import { advisory_count, weights } from '$lib/auth_store';
    let advisory_count_var = 0;
    let weights_var = {
        teacher: 0,
        grade_diversity: 0,
        gender_diversity: 0,
    };

    advisory_count.subscribe((value) => {
        advisory_count_var = value;
    });
    weights.subscribe((value) => {
        weights_var = value;
    });
</script>

<div class="side-bar" />

<style>
    .side-bar {
        position: absolute;
        left: 0;
        background-color: #d2d2d2;

        width: 15vw;
        min-width: 200px;
        height: 100%;
    }
</style>
