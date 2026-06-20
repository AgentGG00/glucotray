<script lang="ts">
    import { _, locale } from "svelte-i18n";
    import { cycleLanguage, flags } from "$lib/app";
    import "$lib/styles/wizard.css";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    type Unit = "mmol" | "mgdl";

    let { onChangeCredentials, onShowPrivacy, onShowTerms, onShowDisclaimer }: {
        onChangeCredentials: (region: string) => void;
        onShowPrivacy: () => void;
        onShowTerms: () => void;
        onShowDisclaimer: () => void;
    } = $props();

    const MMOL_TO_MGDL = 18.0182;

    function mmolToMgdl(val: number): number {
        return Math.round(val * MMOL_TO_MGDL);
    }

    function mgdlToMmol(val: number): number {
        return val / MMOL_TO_MGDL;
    }

    function closestValue(target: number, values: number[]): number {
        return values.reduce((closest, v) =>
            Math.abs(v - target) < Math.abs(closest - target) ? v : closest
        );
    }

    const MIN_VALUES_MMOL = [2.8, 3.0, 3.2, 3.5, 3.8, 4.0, 4.2, 4.5];
    const MAX_VALUES_MMOL = [
        8.0, 8.2, 8.5, 8.8,
        9.0, 9.2, 9.5, 9.8,
        10.0, 10.2, 10.5, 10.8,
        11.0, 11.2, 11.5, 11.8,
        12.0, 12.2, 12.5, 12.8,
        13.0
    ];

    function formatValue(mmol: number, unit: Unit): string {
        if (unit === "mgdl") return `${mmolToMgdl(mmol)} mg/dL`;
        return `${mmol.toFixed(1)} mmol/L`;
    }

    let isLoading = $state(true);
    let isSaving = $state(false);
    let error = $state("");
    let showRestartNotice = $state(false);

    let username = $state("");
    let region = $state("");
    let originalUnit = $state<Unit>("mgdl");

    let unit       = $state<Unit>("mgdl");
    let minMmol    = $state(4.0);
    let maxMmol    = $state(10.0);
    let autostart  = $state(false);
    let colors     = $state({
        criticalLow: "#C62828",
        low:         "#EF6C00",
        normal:      "#2E7D32",
        high:        "#F9A825",
        veryHigh:    "#D84315",
    });

    let availableMaxValues = $derived(
        MAX_VALUES_MMOL.filter(v => v > minMmol)
    );

    let hasChanges = $state(false);

    function markChanged() {
        hasChanges = true;
    }

    async function loadSettings() {
        isLoading = true;
        error = "";
        try {
            const data = await invoke<{
                username: string;
                region: string;
                unit: string;
                thresholdLowMgdl: number;
                thresholdHighMgdl: number;
                autostart: boolean;
                colorCriticalLow: string;
                colorLow: string;
                colorNormal: string;
                colorHigh: string;
                colorVeryHigh: string;
            }>("get_settings");

            username = data.username;
            region = data.region;
            unit = data.unit === "mmol" ? "mmol" : "mgdl";
            originalUnit = unit;

            minMmol = closestValue(mgdlToMmol(data.thresholdLowMgdl), MIN_VALUES_MMOL);
            maxMmol = closestValue(mgdlToMmol(data.thresholdHighMgdl), MAX_VALUES_MMOL);
            autostart = data.autostart;
            colors = {
                criticalLow: data.colorCriticalLow,
                low:         data.colorLow,
                normal:      data.colorNormal,
                high:        data.colorHigh,
                veryHigh:    data.colorVeryHigh,
            };
        } catch (e) {
            error = e as string;
        } finally {
            isLoading = false;
        }
    }

    onMount(() => {
        loadSettings();
    });

    function handleMinChange(e: Event) {
        const val = parseFloat((e.target as HTMLSelectElement).value);
        minMmol = val;
        if (maxMmol <= minMmol) {
            maxMmol = availableMaxValues[0] ?? maxMmol;
        }
        markChanged();
    }

    function handleMaxChange(e: Event) {
        maxMmol = parseFloat((e.target as HTMLSelectElement).value);
        markChanged();
    }

    async function handleSave() {
        isSaving = true;
        error = "";
        try {
            await invoke("save_settings", {
                unit,
                thresholdLowMgdl: mmolToMgdl(minMmol),
                thresholdHighMgdl: mmolToMgdl(maxMmol),
                autostart,
                colorCriticalLow: colors.criticalLow,
                colorLow: colors.low,
                colorNormal: colors.normal,
                colorHigh: colors.high,
                colorVeryHigh: colors.veryHigh,
            });

            hasChanges = false;

            if (unit !== originalUnit) {
                showRestartNotice = true;
            }
        } catch (e) {
            error = e as string;
        } finally {
            isSaving = false;
        }
    }

    async function handleRestart() {
        await invoke("restart_app");
    }

    async function handleUpdateCheck() {
        // TODO: Update-Logik folgt im nächsten Schritt
        console.log("Update check triggered");
    }

    function handleCredentialsClick() {
        onChangeCredentials(region);
    }
</script>

<div class="wizard-screen">
    <button class="flag-btn" onclick={() => cycleLanguage($locale ?? "en", locale.set)}>
        {flags[$locale ?? "en"]}
    </button>

    {#if isLoading}
        <div class="auth-loading">
            <div class="spinner"></div>
        </div>
    {:else}
        <div class="section">
            <h2>{$_("wizard.credentials.title")}</h2>
            <div class="field-group">
                <label class="field-label">{$_("wizard.credentials.username")}</label>
                <input class="text-input" value={username} disabled />
            </div>
            <div class="field-group">
                <label class="field-label">{$_("wizard.credentials.password")}</label>
                <input class="text-input" value="**********" disabled />
            </div>
            <button class="btn-back" onclick={handleCredentialsClick}>
                {$_("wizard.settings.change_credentials")}
            </button>
        </div>

        <div class="section">
            <h2>{$_("wizard.settings.unit")}</h2>
            <div class="option-group">
                <label class="option" class:selected={unit === "mgdl"}>
                    <input type="radio" name="unit" value="mgdl" checked={unit === "mgdl"} onchange={() => { unit = "mgdl"; markChanged(); }} />
                    {$_("wizard.settings.unit_mgdl")}
                </label>
                <label class="option" class:selected={unit === "mmol"}>
                    <input type="radio" name="unit" value="mmol" checked={unit === "mmol"} onchange={() => { unit = "mmol"; markChanged(); }} />
                    {$_("wizard.settings.unit_mmol")}
                </label>
            </div>
        </div>

        <div class="section">
            <h2>{$_("wizard.settings.threshold_low")}</h2>
            <select class="select-input" value={minMmol} onchange={handleMinChange}>
                {#each MIN_VALUES_MMOL as val}
                    <option value={val} selected={minMmol === val}>
                        {formatValue(val, unit)}
                    </option>
                {/each}
            </select>
        </div>

        <div class="section">
            <h2>{$_("wizard.settings.threshold_high")}</h2>
            <select class="select-input" value={maxMmol} onchange={handleMaxChange}>
                {#each availableMaxValues as val}
                    <option value={val} selected={maxMmol === val}>
                        {formatValue(val, unit)}
                    </option>
                {/each}
            </select>
        </div>

        <div class="section">
            <h2>{$_("wizard.settings.color_scheme")}</h2>
            <p class="hint-text">{$_("wizard.settings.color_hint")}</p>
            <div class="color-grid">
                <div class="color-row">
                    <label class="color-label" for="color-very-high">{$_("wizard.settings.zone_very_high")}</label>
                    <div class="color-input-wrapper">
                        <div class="color-swatch" style="background-color: {colors.veryHigh}"></div>
                        <input id="color-very-high" type="color" bind:value={colors.veryHigh} class="color-picker" oninput={markChanged} />
                        <span class="color-hex">{colors.veryHigh}</span>
                    </div>
                </div>
                <div class="color-row">
                    <label class="color-label" for="color-high">{$_("wizard.settings.zone_high")}</label>
                    <div class="color-input-wrapper">
                        <div class="color-swatch" style="background-color: {colors.high}"></div>
                        <input id="color-high" type="color" bind:value={colors.high} class="color-picker" oninput={markChanged} />
                        <span class="color-hex">{colors.high}</span>
                    </div>
                </div>
                <div class="color-row">
                    <label class="color-label" for="color-normal">{$_("wizard.settings.zone_normal")}</label>
                    <div class="color-input-wrapper">
                        <div class="color-swatch" style="background-color: {colors.normal}"></div>
                        <input id="color-normal" type="color" bind:value={colors.normal} class="color-picker" oninput={markChanged} />
                        <span class="color-hex">{colors.normal}</span>
                    </div>
                </div>
                <div class="color-row">
                    <label class="color-label" for="color-low">{$_("wizard.settings.zone_low")}</label>
                    <div class="color-input-wrapper">
                        <div class="color-swatch" style="background-color: {colors.low}"></div>
                        <input id="color-low" type="color" bind:value={colors.low} class="color-picker" oninput={markChanged} />
                        <span class="color-hex">{colors.low}</span>
                    </div>
                </div>
                <div class="color-row">
                    <label class="color-label" for="color-critical-low">{$_("wizard.settings.zone_critical_low")}</label>
                    <div class="color-input-wrapper">
                        <div class="color-swatch" style="background-color: {colors.criticalLow}"></div>
                        <input id="color-critical-low" type="color" bind:value={colors.criticalLow} class="color-picker" oninput={markChanged} />
                        <span class="color-hex">{colors.criticalLow}</span>
                    </div>
                </div>
            </div>
        </div>

        <div class="section">
            <label class="option">
                <input type="checkbox" bind:checked={autostart} onchange={markChanged} />
                {$_("wizard.settings.autostart_label")}
            </label>
        </div>

        {#if error}
            <p class="error-text">{error}</p>
        {/if}

        {#if showRestartNotice}
            <div class="section">
                <p class="hint-text">{$_("wizard.settings.restart_required")}</p>
                <button class="btn-next" onclick={handleRestart}>
                    {$_("wizard.settings.restart_now")}
                </button>
            </div>
        {/if}

        <div class="actions">
            <button class="btn-back" onclick={handleRestart}>
                {$_("wizard.settings.restart")}
            </button>
            <button class="btn-back" onclick={handleUpdateCheck}>
                {$_("wizard.settings.update_check")}
            </button>
            <button class="btn-submit" disabled={!hasChanges || isSaving} onclick={handleSave}>
                {#if isSaving}
                    {$_("wizard.legal.saving")}
                {:else}
                    {$_("wizard.buttons.submit")}
                {/if}
            </button>
        </div>

        <div class="footer-links">
            <button class="footer-link" onclick={onShowPrivacy}>{$_("wizard.legal.privacy_title")}</button>
            <span class="footer-separator">·</span>
            <button class="footer-link" onclick={onShowTerms}>{$_("wizard.legal.terms_title")}</button>
            <span class="footer-separator">·</span>
            <button class="footer-link" onclick={onShowDisclaimer}>{$_("wizard.legal.disclaimer_title")}</button>
            <span class="footer-separator">·</span>
            <a class="footer-link" href="https://glucotray.github.io/contact" target="_blank" rel="noopener">{$_("wizard.settings.contact")}</a>
            <span class="footer-separator">·</span>
            <a class="footer-link" href="https://github.com/AgentGG00/glucotray" target="_blank" rel="noopener">{$_("wizard.settings.repo")}</a>
        </div>
    {/if}
</div>