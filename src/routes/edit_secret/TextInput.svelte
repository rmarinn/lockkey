<script lang="ts">
  import Icon from "@iconify/svelte";
  import { cubicOut } from "svelte/easing";
  import { fly, slide } from "svelte/transition";

  export let minLblLen = 3;
  export let maxLblLen = 32;
  export let minTextLen = 1;
  export let maxTextLen = 32;
  export let label: string = "";
  export let text: string = "";

  $: labelIsTooShort = label !== "" && label.length < minLblLen;
  $: labelIsTooLong = label.length > maxLblLen;
  $: textIsTooShort = text.length < minTextLen;
  $: textIsTooLong = text.length > maxTextLen;
</script>

<div
  class="card flex flex-col gap-2 w-full flex-grow items-center"
  in:fly={{ x: 300, duration: 150, delay: 150 }}
>
  <div class="relative">
    <div class="lbl-icon">
      <Icon icon="mdi:text-long" />
    </div>
    <div class="flex flex-col gap-2 w-[250px] text-nowrap overflow-visible">
      <input
        type="text"
        name="text-label-input"
        placeholder="label"
        class="p-1 flex-grow h-10"
        bind:value={label}
        autocomplete="off"
        aria-label="Label for the secret"
        min={minLblLen}
        max={maxLblLen}
      />
      <div class="flex flex-col items-center gap-2">
        {#if labelIsTooShort}
          <div
            class="italic text-red-900"
            transition:slide={{ duration: 300, axis: "y", easing: cubicOut }}
          >
            Label must be longer than {minLblLen} characters
          </div>
        {:else if labelIsTooLong}
          <div
            class="italic text-red-900"
            transition:slide={{ duration: 300, axis: "y", easing: cubicOut }}
          >
            Label cannot be longer than {maxLblLen} characters
          </div>
        {/if}
      </div>
    </div>
  </div>

  <div
    class="flex flex-col gap-2 w-full flex-grow text-nowrap overflow-visible"
  >
    <textarea
      name="text-input"
      id="text-input"
      placeholder="place your text here..."
      class="p-1 mt-2 resize-none h-full flex-grow w-full mx-0"
      bind:value={text}
      autocomplete="off"
      aria-label="Text area"
    />
    <div class="flex flex-col items-center gap-2">
      {#if textIsTooShort}
        <div
          class="italic text-red-900"
          transition:slide={{ duration: 300, axis: "y", easing: cubicOut }}
        >
          Text must be longer than {minTextLen} characters
        </div>
      {:else if textIsTooLong}
        <div
          class="italic text-red-900"
          transition:slide={{ duration: 300, axis: "y", easing: cubicOut }}
        >
          Text cannot be longer than {maxTextLen} characters
        </div>
      {/if}
    </div>
    <div class="italic" aria-label="Character count">
      character count: {text.length}
    </div>
  </div>
</div>

<style lang="scss">
  @import "@assets/scss/variables";

  .card {
    padding: 1.5rem;
    gap: 0.5rem;
    min-width: 32rem;
  }

  .lbl-icon {
    position: absolute;
    left: -2rem;
    top: 1.25rem;
    transform: translateY(-50%);
  }
</style>
