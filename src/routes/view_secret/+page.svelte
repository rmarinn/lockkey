<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { fly, fade } from "svelte/transition";
  import { page } from "$app/stores";
  import type { Response } from "@types";
  import { invoke } from "@tauri-apps/api/tauri";
  import { adjustTextAreaHeight } from "@utils";

  let label: string | undefined;
  let data: string | undefined;
  let err_msg: string | undefined;

  let textArea: HTMLTextAreaElement | null = null;

  async function fetchData(label: string) {
    err_msg = undefined;
    data = undefined;

    let resp = await invoke<Response<string | undefined>>("get_secret", {
      label: label,
    });
    if (resp.success) {
      data = resp.body;
    } else {
      err_msg = resp.body;
    }
  }

  onMount(async () => {
    label =
      decodeURIComponent($page.url.searchParams.get("label") || "") ||
      undefined;
    if (label !== undefined) {
      await fetchData(label);
    }

    if (textArea !== null) adjustTextAreaHeight(textArea);
  });
</script>

<div
  class="flex flex-col h-full w-full p-4"
  in:fade={{ duration: 150, delay: 175 }}
>
  <div class="flex justify-end">
    <button on:click={() => goto("/")}>Back</button>
  </div>

  {#if err_msg !== undefined}
    <div class="text-lg italic text-red-900 text-center">{err_msg}</div>
  {/if}
  <div class="flex flex-col content-center gap-4 h-full">
    <h1 class="text-3xl text-center" aria-label="The secret's label">
      {label}
    </h1>

    <div class="flex flex-col gap-3 overflow-hidden p-2">
      <textarea
        placeholder="loading..."
        class="p-1 resize-none"
        bind:value={data}
        autocomplete="off"
        in:fly={{ x: 300, duration: 150, delay: 150 }}
        out:fly={{ x: -300, duration: 150 }}
        bind:this={textArea}
        readonly
        aria-label="Secret text"
      ></textarea>
    </div>
  </div>
</div>
