import { invoke } from "@tauri-apps/api/tauri";

const VersionStatusDictionary = {
    GREEN: "GREEN",
    YELLOW: "YELLOW",
    RED: "RED"
} as const

type VersionStatusUnion = typeof VersionStatusDictionary[keyof typeof VersionStatusDictionary]


export class TauriCommandService {

    static async checkVersions() {
        return await invoke<[VersionStatusUnion,VersionStatusUnion]>("greet", {})
    }

}