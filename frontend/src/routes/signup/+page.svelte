<script lang="ts">
    import Button from '$lib/Button.svelte';
    import Input from '$lib/Input.svelte';
    import HRule from '$lib/Horizontal-Rule.svelte';
    import Logo from '$lib/Logo.svelte';
    import { email } from '$lib/auth_store';

    let email_value = '';
    let password = '';
    let password2 = '';

    email.subscribe((value) => {
        email_value = value;
    });

    function sign_up() {
        if (password !== password2) {
            alert('Password inputs do not match!');
            password = '';
            password2 = '';
        } else {
            alert(`email: ${email_value}\npassword: ${password}`);
        }
    }
</script>

<div class="half left vert_center hori_center">
    <h1 style="margin-top: 0;margin-bottom: 0;">Image Here</h1>
</div>

<div class="half right vert_center hori_center">
    <div class="content flex vert_center hori_center">
        <div class="logo flex vert_center hori_center">
            <Logo name={true} />
        </div>
        <div class="input flex vert_center hori_center">
            <Input bind:value={$email} label="Email" />
            <Input bind:value={password} label="Password" />
            <Input bind:value={password2} label="Repeat Password" />
        </div>

        <div class="buttons flex vert_center hori_center">
            <Button on:click={sign_up} label="Sign Up" />
            <HRule />
            <div style="height: 36px;" />
            <div style="height: 20%;" />
            <a href="/" style="all: inherit;">
                <Button label="Go Back / Log In" />
            </a>
        </div>
    </div>
</div>

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
