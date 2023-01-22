import { writable } from 'svelte/store';
import { CognitoUserSession } from 'amazon-cognito-identity-js';

export const email = writable('');
export const token = writable('');
