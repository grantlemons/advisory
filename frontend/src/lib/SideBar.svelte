<script lang="ts">
    import LabeledNumberField from '$lib/LabeledNumberField.svelte';
    import {
        advisory_count,
        teacher_weight,
        grade_weight,
        gender_weight,
    } from '$lib/auth_store';
    import HorizontalRule from '$lib/Horizontal-Rule.svelte';
    import type { Settings } from '$lib/DBTypes';

    let settings: Settings = {
        weights: {
            has_teacher: 0,
            sex_diverse: 0,
            grade_diverse: 0,
        },
        num_advisories: 0,
        teacher_pairs: [],
    };

    advisory_count.subscribe((value: string) => {
        if (isNaN(Number(value)))
            advisory_count.set(String(settings.num_advisories));
        if (Number(value) > 100) advisory_count.set('100');
        if (Number(value) < 0) advisory_count.set('0');

        settings.num_advisories = Number(value);
    });

    teacher_weight.subscribe((value: string) => {
        if (isNaN(Number(value)))
            teacher_weight.set(String(settings.weights.has_teacher));
        if (Number(value) > 10) teacher_weight.set('10');
        if (Number(value) < 0) teacher_weight.set('0');

        settings.weights.has_teacher = Number(value);
    });
    grade_weight.subscribe((value: string) => {
        if (isNaN(Number(value)))
            grade_weight.set(String(settings.weights.grade_diverse));
        if (Number(value) > 10) grade_weight.set('10');
        if (Number(value) < 0) grade_weight.set('0');

        settings.weights.grade_diverse = Number(value);
    });
    gender_weight.subscribe((value: string) => {
        if (isNaN(Number(value)))
            gender_weight.set(String(settings.weights.sex_diverse));
        if (Number(value) > 10) gender_weight.set('10');
        if (Number(value) < 0) gender_weight.set('0');

        settings.weights.sex_diverse = Number(value);
    });
</script>

<div class="side-bar">
    <div class="input-container">
        <div class="input">
            <h1 style="margin-bottom: 6px">Settings</h1>
            <LabeledNumberField
                bind:value={$advisory_count}
                label="Number of Advisories"
                max={100}
            />
        </div>
        <HorizontalRule />
        <div class="input">
            <h3 style="margin-bottom: 6px">Weights</h3>
            <LabeledNumberField
                bind:value={$teacher_weight}
                label="Student has Teacher"
            />
            <LabeledNumberField
                bind:value={$grade_weight}
                label="Grade Diversity"
            />
            <LabeledNumberField
                bind:value={$gender_weight}
                label="Gender Diversity"
            />
        </div>
    </div>
</div>

<style>
    .side-bar {
        font-family: Roboto;
        color: #424242;

        position: absolute;
        left: 0;
        background-color: #d2d2d2;

        width: 15vw;
        min-width: 308px;
        height: 100%;
        display: grid;
    }
    .input-container {
        grid-area: 1/1;
        width: 95%;
        height: fit-content;
        margin-top: 15vh;

        /* flex */
        display: flex;
        flex-direction: column;

        /* Center left-right */
        margin-right: auto;
        margin-left: auto;
    }
    .input {
        width: 90%;
        display: flex;
        flex-direction: column;
        row-gap: 7px;
        margin-bottom: 30px;

        /* Center left-right */
        margin-right: auto;
        margin-left: auto;
    }
</style>
