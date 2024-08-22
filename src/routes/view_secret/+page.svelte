<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { fly } from "svelte/transition";
  import { page } from "$app/stores";
  import type { Response, Secret } from "@ts/types";
  import { invoke } from "@tauri-apps/api/tauri";
  import Icon from "@iconify/svelte";
  import { logOut } from "@ts/utils";
  import { cubicOut } from "svelte/easing";
  import Loader from "@components/Loader.svelte";
  import { MsgType, showPopupMsg } from "@ts/popupMsgStore";

  let label: string | undefined;
  let data: string | undefined;

  async function fetchData(label: string) {
    data = undefined;

    let resp = await invoke<Response<Secret | undefined>>("get_secret", {
      label: label,
    });
    if (resp.success) {
      data = resp.body?.data;
    } else {
      showPopupMsg(MsgType.Error, "Error retrieving the secret");
    }
  }

  onMount(async () => {
    label =
      decodeURIComponent($page.url.searchParams.get("label") || "") ||
      undefined;

    if (label !== undefined) {
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

{#if data === undefined}
  <div
    class="content text-center flex-grow flex justify-center items-center ml-[4rem] pb-8 text-nowrap"
  >
    <Loader loadingMsg="Loading secret..." />
  </div>
{:else}
  <div
    class="flex flex-col flex-grow w-full p-8 ml-[4rem] content gap-4"
    in:fly={{ x: 300, duration: 150, easing: cubicOut }}
  >
    <h1 class="text-3xl text-center" aria-label="The secret's label">
      {label}
    </h1>
    <div class="secret flex-grow">
      {data}
    </div>
  </div>
{/if}

<style lang="scss">
  @import "@assets/scss/variables";

  .secret {
    background-color: $background;
    color: $text-light;
    box-shadow: inset 1px 1px 4px 2px rgba($primary, 0.2);
    border-radius: 0.5rem;
    padding: 1rem;
  }
</style>
