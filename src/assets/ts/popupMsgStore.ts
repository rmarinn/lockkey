import { writable } from "svelte/store";

export enum MsgType {
	Error,
	Success,
	Warning
}

export class PopupMsg {
	public type: MsgType;
	public msg: string;
	public id: number;

	private static idCounter: number = 0;

	constructor(type: MsgType, msg: string) {
		this.type = type;
		this.msg = msg;
		this.id = PopupMsg.idCounter;
		PopupMsg.idCounter += 1;
	}
}

let msgs: PopupMsg[] = [];

export const popupMsgs = writable<PopupMsg[]>([]);

export async function showPopupMsg(type: MsgType, msg: string, duration: number = 1500) {
	msgs.push(new PopupMsg(type, msg));

	popupMsgs.set(msgs);

	setTimeout(() => {
		msgs = msgs.filter(m => m.msg !== msg);
		popupMsgs.set(msgs);
	}, duration);
}
