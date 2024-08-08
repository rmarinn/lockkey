<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { fly, fade } from "svelte/transition";
  import { createEventDispatcher } from "svelte";
  import { Pulse } from "svelte-loading-spinners";
  import ContentCopy from "svelte-material-icons/ContentCopy.svelte";
  import EyeOffOutline from "svelte-material-icons/EyeOffOutline.svelte";
  import EyeOutline from "svelte-material-icons/EyeOutline.svelte";
  import TrashCanOutline from "svelte-material-icons/TrashCanOutline.svelte";
  import TextLong from "svelte-material-icons/TextLong.svelte";
  import KeyVariant from "svelte-material-icons/KeyVariant.svelte";
  import AsteriskCircleOutline from "svelte-material-icons/AsteriskCircleOutline.svelte";
  import FileDocumentAlertOutline from "svelte-material-icons/FileDocumentAlertOutline.svelte";
  import type { Response } from "@utils";

  interface Secret {
    label: string;
    kind: string;
    data?: string;
    showButtons?: boolean;
  }

  export let secrets: Secret[] = [];

  const dispatch = createEventDispatcher();

  async function showData(label: string) {
    let secret = secrets.find((x) => x.label === label);
    if (secret) {
      secret.data = "decrypting...";
      secrets = secrets;

      let resp = await invoke<Response<string | undefined>>("get_secret", {
        label: label,
      });

      if (resp.success && resp.body !== undefined) {
        secret.data = resp.body;
        secrets = secrets;
      }
    }
  }

  function hideData(label: string) {
    let secret = secrets.find((x) => x.label === label);
    if (secret) {
      secret.data = undefined;
      secrets = secrets;
    }
  }

  function setHoverState(label: Secret, state: boolean) {
    label.showButtons = state;
    secrets = secrets;
  }

  async function handleDelete(label: string) {
    dispatch("secretDeleted", { label: label });
  }

  async function copyToClipboard(label: string) {
    let data: string | undefined = undefined;

    // check if the data is already in the secrets list
    let secret = secrets.find((x) => x.label === label);
    if (secret && secret.data !== undefined) {
      data = secret.data;
    } else {
      // if not, fetch the data
      data = await invoke<string | undefined>("get_secret", {
        label: label,
      });
    }

    // then copy the data to the clipboard
    if (data !== undefined) {
      navigator.clipboard
        .writeText(data)
        .then(() => {
          alert("Text copied to clipboard");
        })
        .catch((err) => {
          console.log("Failed to copy text: ", err);
        });
    }
  }
</script>

<h1 class="mb-4 text-3xl text-center">Secrets</h1>
<div class="flex flex-col gap-3">
  {#each secrets as secret (secret.label)}
    <div
      class="list-item p-2"
      on:mouseenter={() => setHoverState(secret, true)}
      on:mouseleave={() => setHoverState(secret, false)}
      in:fly|global={{ y: 300, duration: 200, delay: 175 }}
      out:fade|global={{ duration: 150 }}
      role="region"
    >
      <div class="item-icon pr-2">
        {#if secret.kind === "password"}
          <KeyVariant />
        {:else if secret.kind === "text"}
          <TextLong />
        {:else}
          <FileDocumentAlertOutline />
        {/if}
      </div>
      <div class="item-label">
        {secret.label}
      </div>
      <div class="item-secret">
        {#if secret.data === "decrypting..."}
          <div in:fade={{ duration: 100, delay: 125 }}>
            <Pulse size="1.5" unit="rem" color="#f6f6f6" duration="0.5s" />
          </div>
        {:else if secret.data !== undefined}
          <div in:fade={{ duration: 100, delay: 125 }}>
            <p>{secret.data}</p>
          </div>
        {:else}
          {#each [...Array(6).keys()] as _}
            <div transition:fade|global={{ duration: 100 }}>
              <AsteriskCircleOutline size="0.9rem" />
            </div>
          {/each}
        {/if}
      </div>
      <div class="buttons-container">
        {#if secret.showButtons}
          <div class="item-buttons" transition:fly={{ x: 300, duration: 300 }}>
            <button
              class="item-button"
              on:click={async () => copyToClipboard(secret.label)}
              ><ContentCopy /></button
            >
            {#if secret.data !== undefined}
              <button
                class="item-button"
                on:click={() => hideData(secret.label)}><EyeOutline /></button
              >
            {:else}
              <button
                class="item-button"
                on:click={async () => showData(secret.label)}
                ><EyeOffOutline /></button
              >
            {/if}
            <button
              class="item-button"
              on:click={async () => handleDelete(secret.label)}
              ><TrashCanOutline /></button
            >
          </div>
        {/if}
      </div>
    </div>
  {/each}
</div>

<style>
  .list-item {
    display: grid;
    grid-template-areas: "icon label secret buttons";
    grid-template-columns: 1fr 0.5fr 0.5fr 1fr;
    border-radius: 0.5rem;
  }

  .list-item:hover {
    box-shadow: 0px 0px 0px 2px whitesmoke;
  }

  .item-icon {
    grid-area: icon;
    display: flex;
    justify-content: end;
    align-items: center;
  }

  .item-label {
    grid-area: label;
    display: flex;
    justify-content: center;
    padding-right: 2rem;
    align-content: center;
    margin-top: auto;
    margin-bottom: auto;
  }

  .item-secret {
    grid-area: secret;
    display: flex;
    justify-content: center;
    padding-left: 2rem;
    align-content: center;
    margin-top: auto;
    margin-bottom: auto;
  }

  .buttons-container {
    grid-area: buttons;
    position: relative;
    overflow: hidden;
    padding: 1.2rem;
  }

  .item-buttons {
    display: flex;
    gap: 0.3rem;
    align-items: center;
    position: absolute;
    top: 50%;
    right: 2rem;
    transform: translateY(-50%);
  }

  .item-button {
    border: none;
  }
</style>
