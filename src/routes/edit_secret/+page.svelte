<script lang="ts">
  import { logOut } from "@utils";
  import Icon from "@iconify/svelte";
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Response, Secret } from "@types";
  import PasswordInput from "./PasswordInput.svelte";
  import TypeSelection from "./TypeSelection.svelte";
  import TextInput from "./TextInput.svelte";
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";

  const MIN_LABEL_LEN = 3;
  const MAX_LABEL_LEN = 32;
  const MIN_PASSWD_LEN = 6;
  const MAX_PASSWD_LEN = 24;
  const MIN_TEXT_LEN = 1;
  const MAX_TEXT_LEN = 3000;
  let selectedType: string = "password";

  let label: string = "";
  let data: string = "";
  let submitting: boolean = false;
  let err_msg: string | undefined = undefined;
  let editing: boolean = true;
  $: isFetchingData = editing && data === "";

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
    let resp: Response<string>;
    if (editing) {
      resp = await invoke<Response<string>>("edit_secret", {
        label: label,
        data: data,
      });
    } else {
      resp = await invoke<Response<string>>("new_secret", {
        kind: selectedType,
        label: label,
        data: data,
      });
    }

    if (resp.success) {
      goto("/secrets");
      return;
    } else {
      err_msg = resp.body;
    }

    submitting = false;
  }

  async function fetchData(label: string) {
    err_msg = undefined;
    data = "";

    let resp = await invoke<Response<Secret | undefined>>("get_secret", {
      label: label,
    });

    if (resp.success) {
      selectedType = resp.body?.kind || "password";
      data = resp.body?.data || "";
    } else {
      err_msg = "error retrieving the secret";
    }
  }

  onMount(async () => {
    label = decodeURIComponent($page.url.searchParams.get("label") || "");

    if (label === "") {
      editing = false;
      isFetchingData = false;
    } else {
      editing = true;
      isFetchingData = true;
      await fetchData(label);
    }
  });
</script>

<aside>
  <button class="nav-btn" on:click={() => goto("/secrets")}
    ><Icon icon="mdi:arrow-back" width="2rem" height="2rem" /></button
  >
  <div class="flex-grow flex items-end">
    <button class="nav-btn" on:click={logOut} aria-label="Log out"
      ><Icon icon="mdi:logout-variant" width="2rem" height="2rem" /></button
    >
  </div>
</aside>

{#if isFetchingData}
  <div class="content text-center text-3xl flex-grow">
    Loading {label}...
  </div>
{:else}
  <div
    class="flex flex-col flex-grow p-8 ml-[4rem] gap-4 content-center content"
  >
    <div class="flex justify-between items-center gap-[24px]">
      {#if editing}
        <h1 class="text-xl">Edit Secret</h1>
      {:else}
        <h1 class="text-xl">New Secret</h1>
        <TypeSelection bind:selectedType />
      {/if}

      <button
        class="btn btn-primary flex gap-4 item-center"
        on:click={handleSubmit}
      >
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
  </div>
{/if}
