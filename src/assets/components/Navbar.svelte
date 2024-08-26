<script lang="ts">
  import Icon from "@iconify/svelte";
  import { logOut } from "@ts/utils";
  import { goto } from "$app/navigation";

  export let backBtn: boolean = false;
  export let newSecretBtn: boolean = true;
  export let settingsBtn: boolean = false;
  export let logOutBtn: boolean = true;
</script>

<aside>
  <div class="flex flex-col gap-4 items-center">
    {#if backBtn}
      <button class="nav-btn" on:click={() => goto("/secrets")}
        ><Icon icon="mdi:arrow-back" width="2rem" height="2rem" /></button
      >
    {/if}
    {#if newSecretBtn}
      <button
        class="nav-btn"
        on:click={() => goto("/edit_secret")}
        aria-label="Create a new secret"
        ><Icon
          icon="mdi:plus-circle-outline"
          width="2rem"
          height="2rem"
        /></button
      >
    {/if}
  </div>
  <div class="flex-grow flex flex-col gap-4 justify-end items-center">
    {#if settingsBtn}
      <button
        class="nav-btn"
        on:click={() => goto("/settings")}
        aria-label="Log out"
        ><Icon icon="mdi:cog-outline" width="2rem" height="2rem" /></button
      >
    {/if}
    {#if logOutBtn}
      <button class="nav-btn" on:click={logOut} aria-label="Log out"
        ><Icon icon="mdi:logout-variant" width="2rem" height="2rem" /></button
      >
    {/if}
  </div>
</aside>
