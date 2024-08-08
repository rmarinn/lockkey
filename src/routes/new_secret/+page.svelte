<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import { fly, fade } from "svelte/transition";
  import { Pulse } from "svelte-loading-spinners";
  import type { Response } from "@utils";

  const INPUT_TYPES: string[] = ["password", "text"];
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
  let loadingBtnHovered = false;
  let err_msg: string | undefined = undefined;
  $: loadingBtnColor = loadingBtnHovered ? "#6875F5" : "#F6F6F6";

  // clears data when selectedType changes
  $: {
    selectedType;
    data = "";
  }

  async function handleSubmit() {
    submitting = true;

    // label length check
    if (label.length < MIN_LABEL_LEN || label.length > MAX_LABEL_LEN) return;

    // password length check
    if (
      (selectedType === "password" && data.length < MIN_PASSWD_LEN) ||
      data.length > MAX_PASSWD_LEN
    )
      return;

    // text length check
    if (
      (selectedType === "text" && data.length < MIN_TEXT_LEN) ||
      data.length > MAX_TEXT_LEN
    )
      return;

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
</script>

<div
  class="flex flex-col h-full w-full p-4"
  in:fade={{ duration: 150, delay: 175 }}
>
  <div class="flex justify-end">
    <button on:click={() => goto("/")}>Back</button>
  </div>

  {#if err_msg !== undefined}
    <div class="italic text-lg text-red-900 text-center">{err_msg}</div>
  {/if}

  <form
    on:submit|preventDefault={handleSubmit}
    class="flex flex-col content-center gap-4 h-full"
  >
    <h1 class="text-3xl text-center">New Secret</h1>
    <div class="flex gap-3 mx-auto">
      <label for="input-type" class="mr-2">Type:</label>

      {#each INPUT_TYPES as type}
        <div>
          <input
            type="radio"
            name="input-type"
            id={"input-type-" + type}
            value={type}
            bind:group={selectedType}
          />
          {type}
        </div>
      {/each}
    </div>

    <div class="flex flex-col gap-3 overflow-hidden p-2">
      <div class="flex flex-col gap-2 mx-auto">
        <input
          type="text"
          name="password-label-input"
          placeholder="label"
          class="text-center p-1 text-black"
          bind:value={label}
          autocomplete="off"
          aria-label="Label for the secret"
        />
        {#if label.length > 0 && label.length < MIN_LABEL_LEN}
          <div class="italic text-sm">
            the label must be at least {MIN_LABEL_LEN} characters long
          </div>
        {:else if label.length > MAX_LABEL_LEN}
          <div class="italic text-sm">
            the label cannot be longer than {MAX_LABEL_LEN} characters long
          </div>
        {/if}
      </div>
      {#if selectedType === "password"}
        <div
          class="flex flex-col items-center mx-auto gap-2"
          in:fly={{ x: 300, duration: 150, delay: 150 }}
          out:fly={{ x: -300, duration: 150 }}
        >
          <input
            type="password"
            name="password-input"
            id="password-input"
            placeholder="password"
            class="text-center p-1 text-black"
            bind:value={data}
            autocomplete="off"
            aria-label="Password"
          />
          {#if data.length > 0 && data.length < MIN_PASSWD_LEN}
            <div class="italic text-sm">
              the password must be at least {MIN_PASSWD_LEN} characters long
            </div>
          {:else if data.length > MAX_PASSWD_LEN}
            <div class="italic text-sm">
              the password cannot be longer than {MAX_PASSWD_LEN} characters long
            </div>
          {/if}
        </div>
      {:else if selectedType === "text"}
        <textarea
          name="text-input"
          id="text-input"
          placeholder="place your text here..."
          rows="5"
          class="p-1 text-black"
          bind:value={data}
          autocomplete="off"
          in:fly={{ x: 300, duration: 150, delay: 150 }}
          out:fly={{ x: -300, duration: 150 }}
        ></textarea>
      {:else}
        <p>Unknown type</p>
      {/if}
    </div>

    <div class="flex justify-center justify-self-end mt-auto">
      <button
        style="width: 200px; height: 2.8rem;"
        disabled={submitting}
        class="text-center"
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
          Save
        {/if}
      </button>
    </div>
  </form>

  <div></div>
</div>
