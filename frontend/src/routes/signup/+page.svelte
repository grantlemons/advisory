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
        type ISignUpResult,
    } from 'amazon-cognito-identity-js';

    // variables used for Cognito
    const poolData = {
        UserPoolId: 'us-east-1_Ye96rGbqV',
        ClientId: '5c6eva8nctpb3aug8l0teak36v',
    };
    const userPool = new CognitoUserPool(poolData);
    let cognitoUser: CognitoUser;

    // variable used for feedback & errors
    // for instance, password needs to fit X requirements
    let error_text = '';

    // variables bound to input boxes
    let form = {
        email_value: '',
        name: '',
        password: '',
        pass_verify: '',
    };

    // stores email in state in order to keep between pages
    email.subscribe((value) => {
        form.email_value = value;
    });

    // functions to redirect to other internal pages
    // these are functions so they can be used by other code as well as elements on the page
    function redirect_login() {
        goto('/');
    }
    function redirect_confirm() {
        goto('/confirmation');
    }

    // function to create an account in Cognito user pool
    function sign_up() {
        if (!verify_input()) return;

        let attributeEmail = new CognitoUserAttribute({
            Name: 'email',
            Value: form.email_value,
        });
        let attributeName = new CognitoUserAttribute({
            Name: 'name',
            Value: form.name,
        });

        userPool.signUp(
            form.email_value,
            form.password,
            [attributeEmail, attributeName],
            [],
            function (err, result) {
                if (result != undefined) {
                    success(result);
                }
                if (err != undefined) {
                    failure(err);
                }
            }
        );
    }

    // callback called if auth is successful
    function success(result: ISignUpResult) {
        cognitoUser = result.user;
        error_text = `User name is ${cognitoUser.getUsername()}`;
        console.log(`User name is ${cognitoUser.getUsername()}`);

        redirect_confirm();
    }

    // callback called if auth fails
    function failure(err: Error) {
        error_text = err.message || JSON.stringify(err);
    }

    // function to verify input meets standards
    function verify_input(): boolean {
        // flag
        let match = true;

        // check if two password inputs match
        if (form.password !== form.pass_verify) {
            error_text = 'Password inputs do not match!';
            match = false;
        } else {
            error_text = '';
        }
        return (
            form.email_value != '' &&
            form.password != '' &&
            form.pass_verify != '' &&
            match
        );
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
            <Input bind:value={form.name} label="Name" />
            <Input bind:value={form.password} password label="Password" />
            <Input
                bind:value={form.pass_verify}
                {error_text}
                password
                label="Repeat Password"
            />
        </div>

        <div class="buttons flex vert_center hori_center">
            <Button on:click={sign_up} label="Sign Up" />
            <HRule />
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
