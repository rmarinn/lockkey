import { goto } from "$app/navigation";
import { invoke } from "@tauri-apps/api/tauri";
import { userPrefs } from "./userPrefs";

export function adjustTextAreaHeight(textArea: HTMLTextAreaElement) {
  if (textArea === null) return;

  textArea.style.height = "auto";
  textArea.style.height = `${textArea.scrollHeight}px`;
}

export async function logOut() {
  if (await invoke<boolean>("logout")) {
    goto("/login");
    userPrefs.reset();
  }
}
