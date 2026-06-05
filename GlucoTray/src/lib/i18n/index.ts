import { register, init, getLocaleFromNavigator } from "svelte-i18n";
import { locale as osLocale } from "@tauri-apps/plugin-os";

export async function setupI18n(): Promise<void> {
    register("de", () => import("./de.json"));
    register("en", () => import("./en.json"));
    register("jp", () => import("./jp.json"));

    const detectedLocale = await detectLocale();

    init({
        fallbackLocale: "en",
        initialLocale: detectedLocale,
    });
}

async function detectLocale(): Promise<string> {
    try {
        const raw = await osLocale();
        if (!raw) throw new Error("no locale");
        const lang = raw.split("-")[0].toLowerCase();
        if (lang === "de") return "de";
        if (lang === "ja") return "jp";
        return "en";
    } catch {
        const nav = getLocaleFromNavigator() ?? "en";
        const lang = nav.split("-")[0].toLowerCase();
        if (lang === "de") return "de";
        if (lang === "ja") return "jp";
        return "en";
    }
}