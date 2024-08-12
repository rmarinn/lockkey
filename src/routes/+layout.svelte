<script lang="ts">
  import { fade } from "svelte/transition";
  import { cubicOut } from "svelte/easing";

  interface Data {
    url: string;
  }

  export let data: Data;
</script>

{#key data.url}
  <div
    class="w-screen h-screen flex flex-col justify-start items-start p-8"
    in:fade={{ duration: 150, delay: 175, easing: cubicOut }}
    out:fade={{ duration: 150 }}
  >
    <slot />
  </div>
{/key}
