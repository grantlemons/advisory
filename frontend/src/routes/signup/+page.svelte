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

    let poolData = {
        UserPoolId: 'us-east-1_Ye96rGbqV',
        ClientId: '5c6eva8nctpb3aug8l0teak36v',
    };
    let userPool = new CognitoUserPool(poolData);
    let cognitoUser: CognitoUser;

    let form = {
        email_value: '',
        name: '',
        password: '',
        pass_verify: '',
    };

    email.subscribe((value) => {
        form.email_value = value;
    });

    function redirect_login() {
        goto('/');
    }
    function redirect_confirm() {
        goto('/confirmation');
    }

    function sign_up() {
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

    function success(result: ISignUpResult) {
        cognitoUser = result.user;
        alert(`User name is ${cognitoUser.getUsername()}`);
        console.log(`User name is ${cognitoUser.getUsername()}`);

        redirect_confirm();
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
            <Input bind:value={form.name} label="Name" />
            <Input bind:value={form.password} label="Password" />
            <Input bind:value={form.pass_verify} label="Repeat Password" />
        </div>

        <div class="buttons flex vert_center hori_center">
            <Button on:click={sign_up} label="Sign Up" />
            <HRule />
            <div style="height: 36px;" />
            <div style="height: 20%;" />
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
        row-gap: 20px;
        min-width: 20%;
        max-width: 80%;
        max-height: fit-content;
        /* min-height: 70vh; */
        position: absolute;
        top: 25vh;
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
