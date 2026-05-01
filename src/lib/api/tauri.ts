import { invoke } from "@tauri-apps/api/core";
import type { Image } from "../types/image";

export async function getImageUrl(id: string): Promise<Image[]> {
    const images = await invoke<Image[]>("get_image_for_id", { id });
    if (!images.length) throw new Error("No image found");
    return images;
}

export async function setSteamGridDbApiKey(apiKey: string) {
    await invoke("set_steamgrid_key", { apiKey });
}