<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Response } from "@types";
  import { MsgType, showMsg } from "../../assets/ts/popupMsgStore";

  const minUsrnameLen = 3;
  const maxUsrnameLen = 24;
  const minPasswdLen = 6;
  const maxPasswdLen = 24;

  let usrname = "";
  let passwd = "";
  let confirm_passwd = "";

  $: invalidInput =
    usrname.length < minUsrnameLen ||
    usrname.length > maxUsrnameLen ||
    passwd != confirm_passwd ||
    passwd.length < minPasswdLen ||
    passwd.length > maxPasswdLen;

  async function handleCreateAccount() {
    if (invalidInput) return;

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
  }
</script>

<div class="flex flex-col justify-center items-center m-auto">
  <form on:submit|preventDefault={handleCreateAccount} class="card">
    <h1 class="text-3xl text-center">Create account</h1>

    <div class="flex flex-col gap-[12px]">
      <input
        type="text"
        placeholder="username"
        bind:value={usrname}
        maxlength={maxUsrnameLen}
        spellcheck="false"
        aria-label="Username"
      />
      <input
        type="password"
        placeholder="password"
        bind:value={passwd}
        maxlength={maxPasswdLen}
        aria-label="Password"
      />
      <input
        type="password"
        placeholder="confirm password"
        bind:value={confirm_passwd}
        maxlength={maxPasswdLen}
        aria-label="Confirm password"
      />
    </div>
    <div class="flex flex-col gap-[10px] m-auto">
      <button class="btn" disabled={invalidInput} type="submit"
        >Create account</button
      >
      <button
        class="btn btn-secondary"
        on:click|preventDefault={() => goto("/login")}>Back to Login</button
      >
    </div>
  </form>
</div>
