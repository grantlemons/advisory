import { writable } from 'svelte/store';

export const email = writable('');
export const id_token = writable('');
export const weights = writable({
    teacher: 0,
    grade_diversity: 0,
    gender_diversity: 0,
});
export const advisory_count = writable(0);
