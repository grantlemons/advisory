import { writable } from 'svelte/store';
import { write } from 'xlsx';

export const email = writable('');
export const id_token = writable('');

export const teacher_weight = writable('1');
export const grade_weight = writable('1');
export const gender_weight = writable('1');
export const equal_people_weight = writable('1');

export const advisory_count = writable('');
