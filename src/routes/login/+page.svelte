<script lang="ts">
  import { fade } from "svelte/transition";
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";

  let usrname = "";
  let passwd = "";
  let success: boolean | undefined = undefined;

  async function handleLogin() {
    success = undefined;

    if (usrname.length === 0 || passwd.length === 0) {
      return;
    }

    console.log("submitting2");

    success = await invoke<boolean>("login", {
      usrname: usrname,
      passwd: passwd,
    });

    console.log(success);

    if (success) {
      goto("/secrets");
    }
  }
</script>

<div
  class="flex flex-col h-full w-full p-4"
  in:fade={{ duration: 150, delay: 175 }}
>
  <form
    on:submit|preventDefault={handleLogin}
    class="flex flex-col justify-center gap-4 h-full mx-auto"
  >
    {#if success === false}
      <p class="text-lg italic text-red-900 text-center">login failed</p>
    {/if}
    <h1 class="text-3xl text-center mb-4">Lockkey</h1>
    <input
      type="text"
      placeholder="username"
      class="text-black text-center"
      bind:value={usrname}
    />
    <input
      type="password"
      placeholder="password"
      class="text-black text-center"
      bind:value={passwd}
    />
    <div class="mt-4 mx-auto">
      <button style="width: 200px;">Login</button>
    </div>
    <div class="text-center mt-8">
      <p>Don't have an account?</p>
      <a href="/new_account">create an account</a>
    </div>
  </form>

  <div></div>
</div>
