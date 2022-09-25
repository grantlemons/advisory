<script lang="ts">
    import Button from '../lib/Button.svelte';
    import Input from '../lib/Input.svelte';
    import HRule from '../lib/Horizontal-Rule.svelte';

    let email = '';
    let password = '';

    function sign_in() {
        alert(`email: ${email}\npassword: ${password}`);
    }
    function sign_up() {
        alert(`email: ${email}\npassword: ${password}`);
    }
    function sign_w_google() {
        alert(`email: ${email}\npassword: ${password}`);
    }
</script>

<div class="half left centered_contents">
    <h1 style="margin-top: 0;margin-bottom: 0;">Image Here</h1>
</div>

<div class="half right centered_contents">
    <div class="content flex centered_contents">
        <div class="input flex centered_contents">
            <Input bind:value={email} label="Email" />
            <Input bind:value={password} label="Password" />
        </div>

        <div class="buttons flex centered_contents">
            <Button on:click={sign_in} label="Sign In" />
            <HRule />
            <Button on:click={sign_w_google} label="Sign In With Google" />
            <div style="height: 20%;" />
            <Button on:click={sign_up} label="Sign Up" />
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
    .centered_contents {
        justify-content: center;
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
