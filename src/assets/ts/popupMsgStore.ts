import { writable } from "svelte/store";

export enum MsgType {
	Error,
	Success,
}

export interface PopupMsg {
	type: string;
	msg: string;
}

let msgs: PopupMsg[] = [];
export const popupMsgs = writable<PopupMsg[]>([]);

export async function showMsg(type: MsgType, msg: string, duration: number = 1500) {
	msgs.push({ type: type.toString(), msg: msg });
	popupMsgs.set(msgs);

	setTimeout(() => {
		msgs = msgs.filter(m => m.msg !== msg);
		popupMsgs.set(msgs);
	}, duration);
}
