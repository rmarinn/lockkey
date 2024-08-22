import { readTextFile, writeTextFile } from "@tauri-apps/api/fs";
import { appConfigDir } from "@tauri-apps/api/path";
import { showPopupMsg, MsgType } from "./popupMsgStore";
import { writable } from "svelte/store";

class UserPrefs {
	public usrname: string | undefined = undefined;

	public passwdGen: {
		passwdLength: number;
		useLetters: boolean;
		useNumbers: boolean;
		useSymbols: boolean;
		useUppercase: boolean;
		excludedChars: string;
	}

	constructor() {
		this.passwdGen = {
			passwdLength: 12,
			useLetters: true,
			useNumbers: true,
			useSymbols: true,
			useUppercase: true,
			excludedChars: "",
		};
	}

	public static async load(usrname: string): Promise<UserPrefs> {
		const appConfigDirPath: string = await appConfigDir();
		const filePath = `${appConfigDirPath}${usrname}.prefs.json`;

		try {
			const jsonString = await readTextFile(filePath);
			const configObj = JSON.parse(jsonString) as unknown as UserPrefs;
			const prefs = new UserPrefs();
			Object.assign(prefs, configObj);
			prefs.usrname = usrname;
			return prefs;
		} catch (error) {
			if (error instanceof Error) {
				const errMsg = error.message;
				if (errMsg.includes("The system cannot find the file specified")) {
					console.log("Creating new user prefs");
				} else {
					showPopupMsg(MsgType.Error, `Failed to load user preferences: ${error}`);
				}
			} else {
				if (typeof error === "string" && error.includes("The system cannot find the file specified")) {
					console.log("Creating new user prefs");
				} else {
					showPopupMsg(MsgType.Error, `Failed to load user preferences: ${error}`);
				}
			}
			return new UserPrefs();
		}
	}

	public async save(): Promise<void> {
		if (this.usrname === undefined) return;

		const appConfigDirPath: string = await appConfigDir();
		const filePath = `${appConfigDirPath}${this.usrname}.prefs.json`;

		try {
			const tomlString = JSON.stringify({ passwdGen: this.passwdGen });
			await writeTextFile(filePath, tomlString);
			console.log(`saved user prefs to ${filePath}`);
		} catch (error) {
			showPopupMsg(MsgType.Error, `Failed to save user preferences: ${error}`);
		}
	}
}

function createUserPrefsStore() {
	const { subscribe, set, update } = writable<UserPrefs>(new UserPrefs());

	return {
		subscribe,
		async load(usrname: string) {
			let prefs = await UserPrefs.load(usrname);
			prefs.usrname = usrname;
			set(prefs);
		},
		async save() {
			let prefs: UserPrefs = new UserPrefs();

			update((currentPrefs: UserPrefs) => {
				prefs = currentPrefs;
				return currentPrefs;
			});

			if (prefs instanceof UserPrefs) {
				await prefs.save();
			} else {
				showPopupMsg(MsgType.Error, `Failed to save user preferences`);
			}
		},
		updatePrefs(updateFn: (prefs: UserPrefs) => void) {
			update(currentPrefs => {
				updateFn(currentPrefs);
				return currentPrefs;
			});
		},
		reset() {
			set(new UserPrefs());
		}
	}
}

export const userPrefs = createUserPrefsStore();
