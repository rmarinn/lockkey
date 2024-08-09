<script lang="ts">
  import { goto } from "$app/navigation";
  import { fade } from "svelte/transition";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Response } from "@types";

  const minUsrnameLen = 3;
  const maxUsrnameLen = 24;
  const minPasswdLen = 6;
  const maxPasswdLen = 24;

  let usrname = "";
  let passwd = "";
  let confirm_passwd = "";
  let err_msg: string | undefined = undefined;

  $: invalidInput =
    usrname.length < minUsrnameLen ||
    usrname.length > maxUsrnameLen ||
    passwd != confirm_passwd ||
    passwd.length < minPasswdLen ||
    passwd.length > maxPasswdLen;

  async function handleCreateAccount() {
    err_msg = undefined;

    if (invalidInput) return;

    let resp = await invoke<Response<string>>("new_user", {
      usrname: usrname,
      passwd: passwd,
    });

    if (resp.success === true) {
      goto("/login");
    } else {
      err_msg =
        resp.body ??
        "An error has occured whilte trying to create a new account";
    }
  }
</script>

<div
  class="flex flex-col h-full w-full p-4"
  in:fade={{ duration: 150, delay: 175 }}
>
  {#if err_msg !== undefined}
    <div class="italic text-center text-red-900 text-lg">
      {err_msg}
    </div>
  {/if}

  <form
    on:submit|preventDefault={handleCreateAccount}
    class="flex flex-col justify-center gap-4 h-full mx-auto"
  >
    <h1 class="text-3xl text-center mb-4">Create a new account</h1>

    <input
      type="text"
      placeholder="username"
      bind:value={usrname}
      maxlength={maxUsrnameLen}
      spellcheck="false"
      aria-label="Username"
    />
    {#if usrname !== "" && usrname.length < minUsrnameLen}
      <p class="italic text-sm">
        username must be at least {minUsrnameLen} characters long
      </p>
    {/if}
    <input
      type="password"
      placeholder="password"
      bind:value={passwd}
      maxlength={maxPasswdLen}
      aria-label="Password"
    />
    {#if passwd !== "" && passwd.length < minPasswdLen}
      <p class="italic text-sm">
        password must be at least {minPasswdLen} characters long
      </p>
    {/if}
    <input
      type="password"
      placeholder="confirm password"
      bind:value={confirm_passwd}
      maxlength={maxPasswdLen}
      aria-label="Confirm password"
    />
    {#if confirm_passwd !== "" && passwd !== confirm_passwd}
      <p class="italic text-sm text-center">passwords do not match</p>
    {/if}
    <div class="mt-4 mx-auto">
      <button style="width: 200px;" disabled={invalidInput}
        >Create account</button
      >
    </div>
    <div class="text-center mt-8">
      <p>alread have an account?</p>
      <a href="/login" aria-label="Go to login">go to login</a>
    </div>
  </form>
</div>
