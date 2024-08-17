<script lang="ts">
  import Icon from "@iconify/svelte";
  import { fly } from "svelte/transition";

  export let minLblLen = 3;
  export let maxLblLen = 32;
  export let minTextLen = 3;
  export let maxTextLen = 32;
  export let label: string = "";
  export let text: string = "";
</script>

<div
  class="card flex flex-col gap-2 w-full flex-grow"
  in:fly={{ x: 300, duration: 150, delay: 150 }}
>
  <div class="grid grid-cols-[0.3fr_1fr_0.3fr] gap-4">
    <div class="justify-self-end self-center">
      <Icon icon="mdi:text-long" />
    </div>
    <input
      type="text"
      name="password-label-input"
      placeholder="label"
      class="p-1 flex-grow"
      bind:value={label}
      autocomplete="off"
      aria-label="Label for the secret"
      min={minLblLen}
      max={maxLblLen}
    />
  </div>

  <textarea
    name="text-input"
    id="text-input"
    placeholder="place your text here..."
    class="p-1 mt-2 resize-none h-full flex-grow w-full mx-0"
    bind:value={text}
    autocomplete="off"
    minlength={minTextLen}
    maxlength={maxTextLen}
  />
</div>

<style lang="scss">
  @import "../../assets/scss/variables";

  .card {
    padding: 1.5rem;
    gap: 0.5rem;
    min-width: 32rem;
  }
</style>
