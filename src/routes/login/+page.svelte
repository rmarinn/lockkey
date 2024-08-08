<script lang="ts">
  import { fade } from "svelte/transition";
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Response } from "@utils";
  import { Pulse } from "svelte-loading-spinners";

  let usrname: string = "";
  let passwd: string = "";
  let err_msg: string | undefined = undefined;
  let submitting: boolean = false;
  let loadingBtnHovered = false;
  $: loadingBtnColor = loadingBtnHovered ? "#6875F5" : "#F6F6F6";

  $: inputIsInvalid = usrname.length === 0 || passwd.length === 0;

  async function handleLogin() {
    submitting = true;
    err_msg = undefined;

    if (inputIsInvalid) {
      return;
    }

    let resp = await invoke<Response<string>>("login", {
      usrname: usrname,
      passwd: passwd,
    });

    if (resp.success) {
      goto("/secrets");
      return;
    }

    err_msg = resp.body ?? "An error has occured while trying to log in";
    submitting = false;
  }
</script>

<div
  class="flex flex-col h-full w-full p-4"
  in:fade={{ duration: 150, delay: 175 }}
>
  {#if err_msg !== undefined}
    <div class="italic text-lg text-center text-red-900">{err_msg}</div>
  {/if}

  <form
    on:submit|preventDefault={handleLogin}
    class="flex flex-col justify-center gap-4 h-full mx-auto"
  >
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
    <button
      style="width: 200px; height: 2.8rem;"
      disabled={submitting || inputIsInvalid}
      class="text-center mt-4"
    >
      {#if submitting}
        <div
          class="w-full h-full flex justify-center items-center"
          role="none"
          on:mouseenter={() => {
            loadingBtnHovered = true;
          }}
          on:mouseleave={() => {
            loadingBtnHovered = false;
          }}
        >
          <Pulse size="1.5" unit="rem" color={loadingBtnColor} />
        </div>
      {:else}
        Log in
      {/if}
    </button>

    <div class="text-center mt-8">
      <p>Don't have an account?</p>
      <a href="/new_account">create an account</a>
    </div>
  </form>

  <div></div>
</div>
