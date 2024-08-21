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
  import { fly } from "svelte/transition";
  import { MsgType, showPopupMsg } from "../../assets/ts/popupMsgStore";
  import Loader from "../Loader.svelte";

  const MIN_LABEL_LEN = 3;
  const MAX_LABEL_LEN = 32;
  const MIN_PASSWD_LEN = 6;
  const MAX_PASSWD_LEN = 24;
  const MIN_TEXT_LEN = 1;
  const MAX_TEXT_LEN = 5000;
  let selectedType: string = "password";

  let loadedLabel: string | undefined = undefined;
  let label: string = "";
  let data: string = "";
  let submitting: boolean = false;
  let isEditingSecret: boolean = true;
  let isFetchingData: boolean = true;

  // validation
  $: labelIsNotValid =
    label.length < MIN_LABEL_LEN || label.length > MAX_LABEL_LEN;
  $: passwdIsNotValid =
    selectedType === "password" &&
    (data.length < MIN_PASSWD_LEN || data.length > MAX_PASSWD_LEN);
  $: textIsNotValid =
    selectedType === "text" &&
    (data.length < MIN_TEXT_LEN || data.length > MAX_TEXT_LEN);
  $: canSubmit =
    !labelIsNotValid &&
    ((selectedType === "password" && !passwdIsNotValid) ||
      (selectedType === "text" && !textIsNotValid));

  async function handleSubmit() {
    submitting = true;

    if (labelIsNotValid) {
      showPopupMsg(MsgType.Error, "Invalid label");
      submitting = false;
      return;
    }

    if (passwdIsNotValid) {
      showPopupMsg(MsgType.Error, "Invalid password");
      submitting = false;
      return;
    }

    if (textIsNotValid) {
      showPopupMsg(MsgType.Error, "Invalid text");
      submitting = false;
      return;
    }

    // try to save secret
    let resp: Response<string>;
    if (isEditingSecret) {
      resp = await invoke<Response<string>>("edit_secret", {
        label: loadedLabel,
        newLabel: label,
        newData: data,
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
      showPopupMsg(
        MsgType.Success,
        `Secret ${isEditingSecret ? "edited" : "created"}`,
      );
      return;
    } else {
      showPopupMsg(MsgType.Error, resp.body ?? "An unknown error has occured.");
    }

    submitting = false;
  }

  async function fetchData(label: string) {
    data = "";

    let resp = await invoke<Response<Secret | undefined>>("get_secret", {
      label: label,
    });

    if (resp.success) {
      selectedType = resp.body?.kind || "password";
      data = resp.body?.data || "";
    } else {
      showPopupMsg(MsgType.Error, "Error retrieving the secret");
    }

    isFetchingData = false;
  }

  onMount(async () => {
    loadedLabel =
      decodeURIComponent($page.url.searchParams.get("label") || "") ||
      undefined;

    if (loadedLabel === undefined) {
      isEditingSecret = false;
      isFetchingData = false;
    } else {
      isFetchingData = true;
      label = loadedLabel;
      data = "loading...";
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
  <div
    class="content text-center flex-grow flex justify-center items-center ml-[4rem] pb-8 text-nowrap"
  >
    <Loader loadingMsg="Loading secret..." />
  </div>
{:else}
  <div
    class="flex flex-col flex-grow p-8 ml-[4rem] gap-4 content-center content"
  >
    <div class="flex justify-between items-center gap-[24px]">
      {#if isEditingSecret}
        <h1 class="text-xl">Edit Secret</h1>
      {:else}
        <h1 class="text-xl">New Secret</h1>
        <TypeSelection bind:selectedType />
      {/if}

      <button
        class="btn btn-primary flex gap-4 item-center"
        on:click={handleSubmit}
        disabled={!canSubmit}
        aria-label="Save secret"
      >
        <Icon icon="mdi:content-save-outline" class="my-auto" />
        Save
      </button>
    </div>

    <div class="flex flex-col flex-grow">
      {#if selectedType === "password"}
        <div in:fly={{ x: 300, duration: 300 }}>
          <PasswordInput
            bind:label
            bind:passwd={data}
            minLblLen={MIN_LABEL_LEN}
            maxLblLen={MAX_LABEL_LEN}
            minPasswdLen={MIN_PASSWD_LEN}
            maxPasswdLen={MAX_PASSWD_LEN}
          />
        </div>
      {:else if selectedType === "text"}
        <div in:fly={{ x: 300, duration: 300 }} class="flex-grow flex flex-col">
          <TextInput
            bind:label
            bind:text={data}
            minLblLen={MIN_LABEL_LEN}
            maxLblLen={MAX_LABEL_LEN}
            minTextLen={MIN_TEXT_LEN}
            maxTextLen={MAX_TEXT_LEN}
          />
        </div>
      {:else}
        <div in:fly={{ x: 300, duration: 300 }}>
          <p>Unknown type</p>
        </div>
      {/if}
    </div>
  </div>
{/if}
