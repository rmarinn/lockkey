<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Response } from "@types";
  import { MsgType, showMsg } from "../../assets/ts/popupMsgStore";
  import SmallLoader from "../SmallLoader.svelte";
  import { slide } from "svelte/transition";
  import { cubicOut } from "svelte/easing";

  const MIN_USERNAME_LEN: number = 3;
  const MAX_USERNAME_LEN: number = 24;
  const MIN_PASSWORD_LEN: number = 6;
  const MAX_PASSWORD_LEN: number = 24;

  let usrname: string = "";
  let passwd: string = "";
  let confirm_passwd: string = "";
  let submitting: boolean = false;

  $: usrnameTooShort = usrname.length > 0 && usrname.length < MIN_USERNAME_LEN;
  $: usrnameTooLong = usrname.length > MAX_USERNAME_LEN;
  $: passwdTooShort = passwd.length > 0 && passwd.length < MIN_PASSWORD_LEN;
  $: passwdTooLong = passwd.length > MAX_PASSWORD_LEN;
  $: passwdDontMatch = passwd !== confirm_passwd;

  $: invalidInput =
    usrname.length < MIN_USERNAME_LEN ||
    usrname.length > MAX_USERNAME_LEN ||
    passwd !== confirm_passwd ||
    passwd.length < MIN_PASSWORD_LEN ||
    passwd.length > MAX_PASSWORD_LEN;

  async function handleCreateAccount() {
    if (invalidInput) {
      submitting = false;
      showMsg(MsgType.Error, "Invalid username or password");
      return;
    }

    submitting = true;

    let resp = await invoke<Response<string>>("new_user", {
      usrname: usrname,
      passwd: passwd,
    });

    if (resp.success === true) {
      goto("/login");
      showMsg(MsgType.Success, "Account created");
    } else {
      showMsg(
        MsgType.Error,
        resp.body ??
          "An error has occured whilte trying to create a new account",
      );
    }

    submitting = false;
  }
</script>

<div class="flex flex-col justify-center items-center m-auto">
  <form on:submit|preventDefault={handleCreateAccount} class="card">
    <h1 class="text-3xl text-center">Create account</h1>

    <div class="flex flex-col gap-4">
      <div class="flex flex-col gap-2">
        <input
          type="text"
          placeholder="username"
          bind:value={usrname}
          spellcheck="false"
          aria-label="Username"
        />
        {#if usrnameTooShort}
          <div
            class="text-sm italic input-msg"
            transition:slide|global={{ duration: 300, easing: cubicOut }}
          >
            Username must be at least {MIN_USERNAME_LEN} characters long
          </div>
        {:else if usrnameTooLong}
          <div
            class="text-sm italic input-msg"
            transition:slide|global={{ duration: 300, easing: cubicOut }}
          >
            Username cannot be longer than {MAX_USERNAME_LEN} characters long
          </div>
        {/if}
        {#if usrnameTooShort || usrnameTooLong}
          <div
            class="text-sm italic input-msg"
            transition:slide|global={{ duration: 300, easing: cubicOut }}
          >
            Current length: <b>{usrname.length}</b> characters
          </div>
        {/if}
      </div>
      <div class="flex flex-col gap-2">
        <input
          type="password"
          placeholder="password"
          bind:value={passwd}
          aria-label="Password"
        />
        {#if passwdTooShort}
          <div
            class="text-sm italic input-msg"
            transition:slide|global={{ duration: 300, easing: cubicOut }}
          >
            Password must be at least {MIN_PASSWORD_LEN} characters long
          </div>
        {:else if usrnameTooLong}
          <div
            class="text-sm italic input-msg"
            transition:slide|global={{ duration: 300, easing: cubicOut }}
          >
            Password cannot be longer than {MAX_PASSWORD_LEN} characters long
          </div>
        {/if}
        {#if passwdTooShort || passwdTooLong}
          <div
            class="text-sm italic input-msg"
            transition:slide|global={{ duration: 300, easing: cubicOut }}
          >
            Current length: <b>{passwd.length}</b> characters
          </div>
        {/if}
      </div>
      <div class="flex flex-col gap-2">
        <input
          type="password"
          placeholder="confirm password"
          bind:value={confirm_passwd}
          aria-label="Confirm password"
        />
        {#if confirm_passwd.length > 0 && passwdDontMatch}
          <div
            class="text-sm italic input-msg"
            transition:slide|global={{ duration: 300, easing: cubicOut }}
          >
            Passwords do not match
          </div>
        {/if}
      </div>
    </div>
    <div class="flex flex-col gap-[10px] m-auto">
      <button class="btn" disabled={invalidInput || submitting} type="submit">
        {#if submitting}
          <SmallLoader />
        {:else}
          Create account
        {/if}
      </button>
      <button
        class="btn btn-secondary"
        on:click|preventDefault={() => goto("/login")}>Back to Login</button
      >
    </div>
  </form>
</div>

<style lang="scss">
  @import "../../assets/scss/variables";

  input {
    min-width: 18rem;
  }

  .input-msg {
    color: $error;
    text-align: center;
  }
</style>
