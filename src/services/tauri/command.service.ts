import { invoke } from "@tauri-apps/api/tauri";

const VersionStatusDictionary = {
    GREEN: "GREEN",
    YELLOW: "YELLOW",
    RED: "RED"
} as const

const TauriCommands = {
    GREET: "greet",
    BEGIN_INSTALLATION: "begin_installation"
}

type VersionStatusUnion = typeof VersionStatusDictionary[keyof typeof VersionStatusDictionary]


export class TauriCommandService {

    static async checkVersions() {
        return await invoke<[VersionStatusUnion, VersionStatusUnion, VersionStatusUnion]>(TauriCommands.GREET, {})
    }

    static async BeginInstallation() {
        return await invoke(TauriCommands.BEGIN_INSTALLATION, {})
    }

}