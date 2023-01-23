<script lang="ts">
    import Button from '$lib/Button.svelte';
    import Input from '$lib/Input.svelte';
    import HRule from '$lib/Horizontal-Rule.svelte';
    import Logo from '$lib/Logo.svelte';
    import { email, id_token } from '$lib/auth_store';
    import { goto } from '$app/navigation';
    import {
        CognitoUserPool,
        CognitoUser,
        AuthenticationDetails,
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
    function redirect_reset() {
        goto('/password_reset');
    }
    function redirect_dashboard() {
        goto('/dashboard');
    }
    function redirect_confirmation() {
        goto('confirmation');
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
            newPasswordRequired: redirect_reset,
        });
    }

    function success(session: CognitoUserSession) {
        alert('success!');
        let token_value = session.getIdToken().getJwtToken();
        id_token.set(token_value);
        console.log(token_value);
        // redirect_dashboard();
    }

    function failure(err: Error) {
        alert(err.message || JSON.stringify(err));
    }
</script>

<div class="half left vert_center hori_center">
    <h1 style="margin-top: 0;margin-bottom: 0;">Image Here</h1>
</div>

<form on:submit|preventDefault class="half right vert_center hori_center">
    <div class="content flex vert_center">
        <div class="logo flex vert_center hori_center">
            <Logo name={true} />
        </div>
        <div class="input flex vert_center hori_center">
            <Input bind:value={$email} label="Email Address" />
            <Input bind:value={form.password} password label="Password" />
        </div>

        <div class="buttons flex vert_center hori_center">
            <Button on:click={sign_in} label="Sign In" />
            <HRule />
            <Button on:click={redirect_reset} label="Forgot Password" />
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
        row-gap: 10px;
        min-width: 70%;
        max-width: 90%;
        max-height: fit-content;
        position: relative;
    }
    .input {
        row-gap: 8px;
        width: 100%;
    }
    .buttons {
        row-gap: 8px;
        width: 100%;
    }
</style>
