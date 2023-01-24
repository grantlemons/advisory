<script lang="ts">
    import Button from '$lib/Button.svelte';
    import Input from '$lib/Input.svelte';
    import HRule from '$lib/Horizontal-Rule.svelte';
    import Logo from '$lib/Logo.svelte';
    import { email } from '$lib/auth_store';
    import { goto } from '$app/navigation';
    import { CognitoUserPool, CognitoUser } from 'amazon-cognito-identity-js';

    // variables used for Cognito
    const pool_data = {
        UserPoolId: 'us-east-1_Ye96rGbqV',
        ClientId: '5c6eva8nctpb3aug8l0teak36v',
    };
    const user_pool = new CognitoUserPool(pool_data);
    let cognito_user: CognitoUser;

    // state of page
    let sent = false;

    // variables bound to input boxes
    let form = {
        email_value: '',
        code: '',
        password: '',
        pass_verify: '',
    };

    // stores email in state in order to keep between pages
    email.subscribe((value) => {
        form.email_value = value;
    });

    // function to redirect to other internal page
    // this is a function so it can be used by other code as well as elements on the page
    function redirect_login() {
        goto('/');
    }

    // begin password reset process with Cognito
    // this sends a reset code to the user's email
    function start_reset() {
        if (form.email_value == '') {
            return;
        }
        cognito_user = new CognitoUser({
            Username: form.email_value,
            Pool: user_pool,
        });
        cognito_user.forgotPassword({
            onSuccess: sent_message,
            onFailure: failure,
        });
    }

    // completes the password reset process with Cognito
    // requires the reset code from the user's email
    function change_pass() {
        if (
            form.email_value == '' ||
            form.password == '' ||
            form.pass_verify == ''
        ) {
            return;
        }
        if (form.password !== form.pass_verify) {
            alert('Password inputs do not match!');
            form.password = '';
            form.pass_verify = '';
            return;
        }

        cognito_user = new CognitoUser({
            Username: form.email_value,
            Pool: user_pool,
        });
        cognito_user.confirmPassword(form.code, form.password, {
            onSuccess: redirect_login,
            onFailure: failure,
        });
    }

    // callback called when the initiation of the reset process is successful (i.e. email sent)
    // switches a flag in order to reveal inputs for finishing the process
    function sent_message() {
        sent = true;
        alert('Confirmation code sent to inbox');
    }

    // general callback called when either step of the process fails
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
            {#if sent}
                <Input bind:value={form.code} label="Confirmation Code" />
                <Input bind:value={form.password} password label="Password" />
                <Input
                    bind:value={form.pass_verify}
                    password
                    label="Repeat Password"
                />
            {/if}
        </div>

        <div class="buttons flex vert_center hori_center">
            {#if !sent}
                <Button on:click={start_reset} label="Send Confirmation Code" />
            {:else}
                <Button on:click={change_pass} label="Reset Password" />
            {/if}
            <HRule />
            {#if sent}
                <Button
                    on:click={function () {
                        sent = false;
                    }}
                    label="Go Back"
                />
            {/if}
            <Button on:click={redirect_login} label="Go Back / Log In" />
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
