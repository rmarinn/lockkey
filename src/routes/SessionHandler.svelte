<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import type { UnlistenFn } from "@tauri-apps/api/event";

  import { showAlertMsg } from "../assets/ts/alertMsgStore";
  import { logOut } from "@utils";

  let unlisten: UnlistenFn;

  async function resetSessionTimer() {
    await invoke("update_last_activity");
  }

  function handleActivity() {
    resetSessionTimer();
  }

  function startTrackingUserActivities() {
    window.addEventListener("mousemove", handleActivity);
    window.addEventListener("keydown", handleActivity);
    window.addEventListener("click", handleActivity);
  }

  function stopTrackingUserActivities() {
    window.removeEventListener("mousemove", handleActivity);
    window.removeEventListener("keydown", handleActivity);
    window.removeEventListener("click", handleActivity);
  }

  onMount(async () => {
    startTrackingUserActivities();

    // listen for session timeout form backend
    unlisten = await listen("session_timeout", (_) => {
      console.log("session timed out");

      // implement additional actions here
      logOut();
      showAlertMsg("Logged out due to inactivity");
    });
  });

  onDestroy(() => {
    if (unlisten) {
      unlisten();
    }
    stopTrackingUserActivities();
  });
</script>
