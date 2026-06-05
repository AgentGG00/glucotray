import { register, init, getLocaleFromNavigator } from "svelte-i18n";

export function setupI18n(): void {
    register("de", () => import("./de.json"));
    register("en", () => import("./en.json"));
    register("jp", () => import("./jp.json"));

    const locale = detectLocale();

    init({
        fallbackLocale: "en",
        initialLocale: locale,
    });
}

function detectLocale(): string {
    const nav = getLocaleFromNavigator() ?? "en";
    const lang = nav.split("-")[0].toLowerCase();

    if (lang === "de") return "de";
    if (lang === "ja" || lang === "jp") return "jp";
    return "en";
}