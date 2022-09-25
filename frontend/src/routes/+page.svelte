<script lang="ts">
    import Button, { Label } from '@smui/button';
    import Textfield from '@smui/textfield';

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

<div class="left">
    <h1>Image Here</h1>
</div>

<div class="right">
    <div class="content">
        <div class="input">
            <Textfield
                style="width: 100%"
                variant="outlined"
                bind:value={email}
                label="Email"
            />
            <Textfield
                style="width: 100%"
                variant="outlined"
                bind:value={password}
                label="Password"
            />
        </div>

        <div class="buttons">
            <div class="button signin">
                <Button
                    color="secondary"
                    on:click={sign_in}
                    variant="unelevated"
                >
                    <Label>Sign In</Label>
                </Button>
            </div>

            <HRule color="#939393" thickness={2} />

            <div class="button google">
                <Button
                    color="secondary"
                    on:click={sign_w_google}
                    variant="unelevated"
                >
                    <Label>Sign In With Google</Label>
                </Button>
            </div>

            <!-- LARGER SPACE -->

            <div class="button signup">
                <Button
                    color="secondary"
                    on:click={sign_up}
                    variant="unelevated"
                >
                    <Label>Sign Up</Label>
                </Button>
            </div>
        </div>
    </div>
</div>

<style>
    .left {
        position: absolute;
        height: 100vh;
        width: 50vw;
        background-color: #939393;

        text-align: center;
        line-height: 90vh;
        font-family: Roboto;
        color: #595959;
    }
    .right {
        position: absolute;
        display: flex;
        justify-content: center;

        align-items: center;
        margin-left: 50%;
        height: 100vh;
        width: 50vw;
    }
    .content {
        display: flex;
        flex-direction: column;
        justify-content: center;
        row-gap: 20px;

        align-items: center;
        width: 30%;
    }
    .input {
        display: flex;
        flex-direction: column;
        justify-content: center;
        row-gap: 10px;

        align-items: center;
        width: 100%;
    }
    .buttons {
        display: flex;
        flex-direction: column;
        justify-content: center;
        row-gap: 10px;

        align-items: center;
        width: 100%;
    }
    .button {
        display: flex;
        flex-direction: column;
        justify-content: center;
        row-gap: 10px;

        width: 100%;
    }
    .signup {
        margin-top: 30px;
    }
</style>
