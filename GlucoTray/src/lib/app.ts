export function cycleLanguage(currentLocale: string, setLocale: (l: string) => void): void {
    const languages = ["de", "en", "jp"];
    const idx = languages.indexOf(currentLocale);
    const next = languages[(idx + 1) % languages.length];
    setLocale(next);
}

export const flags: Record<string, string> = {
    de: "🇩🇪",
    en: "🇬🇧",
    jp: "🇯🇵",
};