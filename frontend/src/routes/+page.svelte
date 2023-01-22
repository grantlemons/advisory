<script lang="ts">
    import Button from '$lib/Button.svelte';
    import Input from '$lib/Input.svelte';
    import HRule from '$lib/Horizontal-Rule.svelte';
    import Logo from '$lib/Logo.svelte';
    import { email } from '$lib/auth_store';
    import { goto } from '$app/navigation';
    import {
        CognitoUserPool,
        CognitoUserAttribute,
        CognitoUser,
        AuthenticationDetails,
        type ISignUpResult,
        type IAuthenticationCallback,
        CognitoUserSession,
    } from 'amazon-cognito-identity-js';

    let pool_data = {
        UserPoolId: 'us-east-1_Ye96rGbqV',
        ClientId: '5c6eva8nctpb3aug8l0teak36v',
    };
    let user_pool = new CognitoUserPool(pool_data);
    let cognito_user: CognitoUser;

    let form = {
        email_value: '',
        password: '',
    };

    email.subscribe((value) => {
        form.email_value = value;
    });

    function redirect_signup() {
        goto('/signup');
    }

    function sign_in() {
        if (form.email_value == '' || form.password == '') {
            return;
        }
        cognito_user = new CognitoUser({
            Username: form.email_value,
            Pool: user_pool,
        });
        let auth_details = new AuthenticationDetails({
            Username: form.email_value,
            Password: form.password,
        });
        cognito_user.authenticateUser(auth_details, {
            onSuccess: success,
            onFailure: failure,
        });
    }

    function success(session: CognitoUserSession) {
        alert('success!');
    }

    function failure(err: Error) {
        alert(err.message || JSON.stringify(err));
    }

    function sign_w_google() {
        email.set('');
        form.password = '';
    }
</script>

<div class="half left vert_center hori_center">
    <h1 style="margin-top: 0;margin-bottom: 0;">Image Here</h1>
</div>

<form on:submit|preventDefault class="half right vert_center hori_center">
    <div class="content flex vert_center hori_center">
        <div class="logo flex vert_center hori_center">
            <Logo name={true} />
        </div>
        <div class="input flex vert_center hori_center">
            <Input bind:value={$email} label="Email" />
            <Input bind:value={form.password} label="Password" />
        </div>

        <div class="buttons flex vert_center hori_center">
            <Button on:click={sign_in} label="Sign In" />
            <HRule />
            <Button on:click={sign_w_google} label="Sign In With Google" />
            <div style="height: 20%;" />
            <Button on:click={redirect_signup} label="Sign Up" />
        </div>
    </div>
</form>

<style>
    /* traits */
    .half {
        height: 100vh;
        width: 50vw;
    }
    .flex {
        display: flex;
        flex-direction: column;
    }
    .hori_center {
        justify-content: center;
        text-align: center;
    }
    .vert_center {
        align-items: center;
        text-align: center;
    }

    /* left side */
    .left {
        float: left;
        background-color: #939393;

        line-height: 100vh;
        font-family: Roboto;
        color: #595959;
    }

    /* right side */
    .right {
        float: right;
        display: flex;
    }
    .content {
        row-gap: 20px;
        min-width: 40%;
        max-width: 85%;
        max-height: fit-content;
        min-height: 70vh;
        position: relative;
        bottom: 3%;
    }
    .input {
        row-gap: 10px;
        width: 100%;
    }
    .buttons {
        row-gap: 10px;
        width: 100%;
    }
</style>
