<script lang="ts">
  import Icon from "@iconify/svelte";

  import { goto } from "$app/navigation";
  import { createEventDispatcher } from "svelte";
  import { fly, fade } from "svelte/transition";
  import { cubicOut } from "svelte/easing";
  import { MsgType, showPopupMsg } from "@ts/popupMsgStore";
  import { clipboard } from "@tauri-apps/api";

  import type { Response, Secret } from "@ts/types";
  import { invoke } from "@tauri-apps/api/tauri";
  import SmallLoader from "@components/SmallLoader.svelte";

  const dispatch = createEventDispatcher();
  export let label: string = "";
  export let kind: string = "password";
  let secretData: string = "";

  $: isPasswdVisible = secretData !== "";

  async function viewSecret() {
    if (kind === "text") {
      goto(`/view_secret?label=${label}`);
      return;
    }

    if (isPasswdVisible) {
      secretData = "";
    } else {
      // show loading
      secretData = "decrypting password...";

      // fetch data from the backend
      let resp = await invoke<Response<Secret | undefined>>("get_secret", {
        label: label,
      });

      // show data
      if (resp.success && resp.body !== undefined) {
        secretData = resp.body?.data || "";
      }
    }
  }

  interface GetSecretResponse {
    data: string;
    kind: string;
    label: string;
  }

  async function copyToClipboard() {
    let data: string | undefined = secretData;
    if (data === undefined || data === "") {
      // fetch data from the backend
      let resp = await invoke<Response<GetSecretResponse | undefined>>(
        "get_secret",
        {
          label: label,
        },
      );

      data = resp.body?.data;
    }

    if (data !== undefined) {
      clipboard
        .writeText(data)
        .then(() => {
          showPopupMsg(MsgType.Success, "Text copied to clipboard");
        })
        .catch((err) => {
          showPopupMsg(MsgType.Error, `Failed to copy into clipboard: ${err}`);
        });
    }

    data = "";
  }

  async function deleteSecret() {
    let resp = await invoke<Response<string>>("delete_secret", {
      label: label,
    });

    if (resp.success) {
      dispatch("secretDeleted", { label: label });
      showPopupMsg(MsgType.Success, "Secret deleted");
    } else {
      showPopupMsg(
        MsgType.Error,
        resp.body ??
          `An unknown error has occured while trying to delete secret.`,
      );
    }
  }
</script>

<div
  class="list-item lg:w-[600px] w-full"
  in:fly|global={{ y: 300, duration: 300, easing: cubicOut }}
  out:fade={{ duration: 300, easing: cubicOut }}
>
  <div class="list-label">
    {#if kind === "password"}
      <Icon icon="mdi:key-variant" width="24px" height="24px" />
    {:else if kind === "text"}
      <Icon icon="mdi:text-long" width="24px" height="24px" />
    {:else}
      <Icon icon="mdi:file-document-alert-outline" width="24px" height="24px" />
    {/if}
    <div>
      {label}
    </div>
  </div>
  <div class="flex-grow flex justify-end">
    {#if secretData !== undefined}
      {#if secretData === "decrypting password..."}
        <div class="pr-[24px]" in:fade={{ duration: 300, easing: cubicOut }}>
          <SmallLoader />
        </div>
      {:else}
        <div
          class="italic pr-[24px]"
          in:fly={{ x: -200, duration: 300, easing: cubicOut }}
          out:fade={{ duration: 300, easing: cubicOut }}
        >
          {secretData}
        </div>
      {/if}
    {/if}
  </div>
  <div class="option-btns">
    <button on:click={copyToClipboard}
      ><Icon icon="mdi:content-copy" width="24px" height="24px" /></button
    >
    <button on:click={viewSecret}>
      {#if isPasswdVisible}
        <Icon icon="mdi:eye-outline" width="24px" height="24px" />
      {:else}
        <Icon icon="mdi:eye-off-outline" width="24px" height="24px" />
      {/if}
    </button>
    <button on:click={() => goto(`/edit_secret?label=${label}`)}
      ><Icon
        icon="mdi:square-edit-outline"
        width="24px"
        height="24px"
      /></button
    >
    <button on:click={deleteSecret}
      ><Icon icon="mdi:trash-outline" width="24px" height="24px" /></button
    >
  </div>
</div>

<style lang="scss">
  @import "@assets/scss/variables";

  .list-item {
    padding: 32px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-direction: row;
    border-radius: 30px;
    box-shadow: 2px 2px 4px 2px rgba($primary, 0.3);
  }

  .list-label {
    display: flex;
    align-items: center;
    gap: 24px;
  }

  .option-btns {
    display: flex;
    justify-content: right;
    gap: 24px;
    min-width: 170px;

    & > button {
      filter: drop-shadow(1px 1px 1px rgba($primary, 0.6));
      transition:
        transform 0.2s ease-out,
        filter 0.3s ease-out;

      &:hover {
        filter: drop-shadow(2px 2px 4px $primary);
        transform: translateY(-2px);
        color: $primary;
      }
    }
  }
</style>
