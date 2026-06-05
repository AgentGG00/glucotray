export type Theme = "light" | "dark" | "grey";

export function detectTheme(): Theme {
    if (typeof window === "undefined") return "dark";

    const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;

    if (!prefersDark) return "light";

    const bgColor = window.getComputedStyle(document.documentElement)
        .getPropertyValue("--system-bg")
        .trim();

    if (bgColor === "grey") return "grey";

    return "dark";
}

export function applyTheme(theme: Theme): void {
    document.documentElement.setAttribute("data-theme", theme);
}

export function initTheme(): void {
    const theme = detectTheme();
    applyTheme(theme);

    window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", () => {
        const updated = detectTheme();
        applyTheme(updated);
    });
}