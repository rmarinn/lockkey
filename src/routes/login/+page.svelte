<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Response } from "@types";

  let usrname: string = "";
  let passwd: string = "";
  let err_msg: string | undefined = undefined;
  let submitting: boolean = false;

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

<div class="flex flex-col justify-center items-center m-auto">
  <form class="card" on:submit|preventDefault={handleLogin}>
    <h1 class="app-title text-3xl text-center">Lockkey</h1>
    <div class="flex flex-col gap-2">
      <input
        type="text"
        placeholder="username"
        bind:value={usrname}
        aria-label="Username"
      />
      <input
        type="password"
        placeholder="password"
        bind:value={passwd}
        aria-label="Password"
      />
    </div>

    <div class="flex flex-col gap-[12px]">
      <button class="btn" disabled={submitting || inputIsInvalid} type="submit">
        Log in
      </button>
      <button
        class="btn btn-secondary"
        on:click|preventDefault={() => goto("/new_account")}
      >
        Create account
      </button>
    </div>
  </form>
</div>
