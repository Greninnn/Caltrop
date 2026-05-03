import { invoke } from "@tauri-apps/api/core";
import type { Image } from "../types/image";

export async function getImageUrl(id: string): Promise<string> {
    const image = await invoke<string>("get_image_for_id", { id });
    return image;
}

export async function setSteamGridDbApiKey(apiKey: string) {
    await invoke("set_steamgrid_key", { apiKey });
}

export async function getSteamGridDbApiKey(): Promise<string> {
    return await invoke("get_steamgrid_key");
}