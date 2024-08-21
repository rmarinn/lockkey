import { writable } from "svelte/store";

export const alertMsg = writable<string | null>(null);

export async function showAlertMsg(msg: string) {
	alertMsg.set(msg);
}

export function clearAlertMsg() {
	alertMsg.set(null);
}
