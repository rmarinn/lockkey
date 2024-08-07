<script lang="ts">
  import { goto } from "$app/navigation";
  import { fade } from "svelte/transition";
  import { invoke } from "@tauri-apps/api/tauri";

  const minUsrnameLen = 3;
  const maxUsrnameLen = 24;
  const minPasswdLen = 6;
  const maxPasswdLen = 24;

  let usrname = "";
  let passwd = "";
  let confirm_passwd = "";
  let success: boolean | undefined = undefined;

  async function handleCreateAccount() {
    success = undefined;

    console.log(`func called`);
    if (
      usrname.length < minUsrnameLen ||
      usrname.length > maxUsrnameLen ||
      passwd != confirm_passwd ||
      passwd.length < minPasswdLen ||
      passwd.length > maxPasswdLen
    ) {
      return;
    }

    success = await invoke<boolean>("create_user", {
      usrname: usrname,
      passwd: passwd,
    });

    console.log(`success: ${success}`);

    if (success === true) {
      goto("/login");
    } else {
      success = true;
    }
  }
</script>

<div
  class="flex flex-col h-full w-full p-4"
  in:fade={{ duration: 150, delay: 175 }}
>
  {#if success === false}
    <p class="italic text-center text-red-900 text-lg">
      failed to create a new account
    </p>
  {/if}

  <form
    on:submit|preventDefault={handleCreateAccount}
    class="flex flex-col justify-center gap-4 h-full mx-auto"
  >
    <h1 class="text-3xl text-center mb-4">Create a new account</h1>

    <input
      type="text"
      placeholder="username"
      class="text-black text-center"
      bind:value={usrname}
      maxlength={maxUsrnameLen}
      spellcheck="false"
    />
    {#if usrname !== "" && usrname.length < minUsrnameLen}
      <p class="italic text-sm">
        username must be at least {minUsrnameLen} characters long
      </p>
    {/if}
    <input
      type="password"
      placeholder="password"
      class="text-black text-center"
      bind:value={passwd}
      maxlength={maxPasswdLen}
    />
    {#if passwd !== "" && passwd.length < minPasswdLen}
      <p class="italic text-sm">
        password must be at least {minPasswdLen} characters long
      </p>
    {/if}
    <input
      type="password"
      placeholder="confirm password"
      class="text-black text-center"
      bind:value={confirm_passwd}
      maxlength={maxPasswdLen}
    />
    {#if confirm_passwd !== "" && passwd !== confirm_passwd}
      <p class="italic text-sm text-center">passwords do not match</p>
    {/if}
    <div class="mt-4 mx-auto">
      <button style="width: 200px;">Create account</button>
    </div>
    <div class="text-center mt-8">
      <p>alread have an account?</p>
      <a href="/login">go to login</a>
    </div>
  </form>
</div>
