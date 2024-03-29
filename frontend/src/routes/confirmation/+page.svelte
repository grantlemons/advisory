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

    // variable used for feedback & errors
    // for instance, password needs to fit X requirements
    let error_text = '';

    // variables bound to input boxes
    let form = {
        email_value: '',
        code: '',
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

    // completes the confirmation process with Cognito
    // requires the confirmation code from the user's email
    function confirm() {
        if (form.email_value == '' || form.code == '') {
            return;
        }
        cognito_user = new CognitoUser({
            Username: form.email_value,
            Pool: user_pool,
        });
        cognito_user.confirmRegistration(
            form.code,
            true,
            function (err, result) {
                if (result != undefined) {
                    redirect_login();
                }
                if (err != undefined) {
                    failure(err);
                }
            }
        );
    }

    // sends a new confirmation code through Cognito
    function resend() {
        if (form.email_value == '' || form.code == '') {
            return;
        }
        cognito_user = new CognitoUser({
            Username: form.email_value,
            Pool: user_pool,
        });
        cognito_user.resendConfirmationCode(function (err, result) {
            if (err != undefined) {
                failure(err);
            }
        });
    }

    // general callback called when either confirming or resending fails
    function failure(err: Error) {
        error_text = err.message || JSON.stringify(err);
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
            <Input
                bind:value={form.code}
                {error_text}
                label="Confirmation Code"
            />
        </div>

        <div class="buttons flex vert_center hori_center">
            <Button on:click={confirm} label="Submit Confirmation Code" />
            <HRule />
            <Button on:click={resend} label="Resend Confirmation Code" />
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
