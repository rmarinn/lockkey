<script lang="ts">
  import { logOut } from "@utils";
  import Icon from "@iconify/svelte";
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Response } from "@types";
  import PasswordInput from "./PasswordInput.svelte";
  import TypeSelection from "./TypeSelection.svelte";
  import TextInput from "./TextInput.svelte";

  const MIN_LABEL_LEN = 3;
  const MAX_LABEL_LEN = 32;
  const MIN_PASSWD_LEN = 6;
  const MAX_PASSWD_LEN = 24;
  const MIN_TEXT_LEN = 1;
  const MAX_TEXT_LEN = 3000;
  let selectedType: string = "text";

  let label: string = "";
  let data: string = "";
  let submitting: boolean = false;
  let err_msg: string | undefined = undefined;

  // clears data when selectedType changes
  $: {
    selectedType;
    data = "";
  }

  $: labelIsNotValid =
    label.length < MIN_LABEL_LEN || label.length > MAX_LABEL_LEN;
  $: passwdIsNotValid =
    selectedType === "password" &&
    (data.length < MIN_PASSWD_LEN || data.length > MAX_PASSWD_LEN);
  $: textIsNotValid =
    selectedType === "text" &&
    (data.length < MIN_TEXT_LEN || data.length > MAX_TEXT_LEN);

  async function handleSubmit() {
    submitting = true;

    if (labelIsNotValid) {
      err_msg = "invalid label";
      submitting = false;
      return;
    }

    if (passwdIsNotValid) {
      err_msg = "invalid password";
      submitting = false;
      return;
    }

    if (textIsNotValid) {
      err_msg = "invalid text";
      submitting = false;
      return;
    }

    // try to save secret
    let resp = await invoke<Response<string>>("new_secret", {
      kind: selectedType,
      label: label,
      data: data,
    });

    if (resp.success) {
      goto("/");
      return;
    } else {
      err_msg = resp.body;
    }

    submitting = false;
  }

  interface PasswordConfig {
    length: number;
    useLetters: boolean;
    useUppercase: boolean;
    useNumbers: boolean;
    useSymbols: boolean;
    excludedChars: string;
  }

  let passwdCfg: PasswordConfig = {
    length: 12,
    useLetters: true,
    useUppercase: true,
    useNumbers: true,
    useSymbols: true,
    excludedChars: "",
  };

  function shuffleString(str: string): string {
    const array = str.split("");
    for (let i = array.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [array[i], array[j]] = [array[j], array[i]]; // Swap elements
    }
    return array.join("");
  }

  function generateRandomPasswd(): string {
    const lowercaseLetters = "abcdefghijklmnopqrstuvwxyz";
    const uppercaseLetters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const numbers = "0123456789";
    const symbols = "!@#$%^&*()-_=+[]{}|;:'\",.<>?/\\~`";

    let charSet = "";
    if (passwdCfg.useLetters) charSet += lowercaseLetters;
    if (passwdCfg.useUppercase) charSet += uppercaseLetters;
    if (passwdCfg.useNumbers) charSet += numbers;
    if (passwdCfg.useSymbols) charSet += symbols;
    if (charSet === "") return "";

    const excludedChars = new Set(passwdCfg.excludedChars);
    charSet = charSet
      .split("")
      .filter((char) => !excludedChars.has(char))
      .join("");
    charSet = shuffleString(charSet);

    let passwd = "";

    for (let i = 0; i < passwdCfg.length; i++) {
      passwd += charSet[Math.floor(Math.random() * charSet.length)];
    }

    return passwd;
  }
</script>

<nav class="navbar">
  <div class="flex-grow">
    <button class="nav-btn" on:click={() => goto("/secrets")}
      ><Icon icon="mdi:arrow-back" width="32px" height="32px" /></button
    >
  </div>
  <button class="nav-btn" on:click={logOut} aria-label="Log out"
    ><Icon icon="mdi:logout-variant" width="32px" height="32px" /></button
  >
</nav>

<form
  on:submit|preventDefault={handleSubmit}
  class="flex flex-col content-center gap-4 flex-grow w-full p-0 pl-[44px]"
>
  <div class="flex justify-between items-center gap-[24px]">
    <h1 class="text-xl">New Secret</h1>

    <TypeSelection bind:selectedType />

    <button class="btn btn-primary flex gap-4 item-center">
      <Icon icon="mdi:content-save-outline" class="my-auto" />
      Save
    </button>
  </div>

  <div class="flex flex-col flex-grow">
    {#if selectedType === "password"}
      <PasswordInput
        bind:label
        bind:passwd={data}
        minLblLen={MIN_LABEL_LEN}
        maxLblLen={MAX_LABEL_LEN}
        minPasswdLen={MIN_PASSWD_LEN}
        maxPasswdLen={MAX_PASSWD_LEN}
      />
    {:else if selectedType === "text"}
      <TextInput
        bind:label
        bind:text={data}
        minLblLen={MIN_LABEL_LEN}
        maxLblLen={MAX_LABEL_LEN}
        minTextLen={MIN_TEXT_LEN}
        maxTextLen={MAX_TEXT_LEN}
      />
    {:else}
      <p>Unknown type</p>
    {/if}
  </div>
</form>
