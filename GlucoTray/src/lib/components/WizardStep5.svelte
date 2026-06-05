<script lang="ts">
    import { _, locale } from "svelte-i18n";
    import { cycleLanguage, flags } from "$lib/app";
    import "$lib/styles/wizard.css";
    import { onMount } from "svelte";

    const MMOL_TO_MGDL = 18.0182;

    function mmolToMgdl(val: number): number {
        return Math.round(val * MMOL_TO_MGDL);
    }

    let {
        sensor,
        region,
        username,
        unit,
        minMmol,
        maxMmol,
        autostart,
        colors,
        onBack,
        onFinish,
    }: {
        sensor: string;
        region: string;
        username: string;
        unit: "mmol" | "mgdl";
        minMmol: number;
        maxMmol: number;
        autostart: boolean;
        colors: {
            criticalLow: string;
            low: string;
            normal: string;
            high: string;
            veryHigh: string;
        };
        onBack: () => void;
        onFinish: () => void;
    } = $props();

    let isSaving = $state(false);
    let saveError = $state("");

    function formatUnit(u: "mmol" | "mgdl"): string {
        return u === "mmol" ? "mmol/L" : "mg/dL";
    }

    function formatMin(): string {
        if (unit === "mgdl") return `${mmolToMgdl(minMmol)} mg/dL`;
        return `${minMmol.toFixed(1)} mmol/L`;
    }

    function formatMax(): string {
        if (unit === "mgdl") return `${mmolToMgdl(maxMmol)} mg/dL`;
        return `${maxMmol.toFixed(1)} mmol/L`;
    }

    function regionLabel(r: string): string {
        switch (r) {
            case "us":  return $_("wizard.completion.region_us");
            case "ous": return $_("wizard.completion.region_ous");
            case "jp":  return $_("wizard.completion.region_jp");
            default:    return r;
        }
    }

    onMount(async () => {
        isSaving = true;
        saveError = "";
        try {
            const { invoke } = await import("@tauri-apps/api/core");
            await invoke("save_wizard_data", {
                username,
                region,
                sensor,
                unit,
                thresholdLow:     unit === "mgdl" ? mmolToMgdl(minMmol) : Math.round(minMmol * 10),
                thresholdHigh:    unit === "mgdl" ? mmolToMgdl(maxMmol) : Math.round(maxMmol * 10),
                autostart,
                colorCriticalLow: colors.criticalLow,
                colorLow:         colors.low,
                colorNormal:      colors.normal,
                colorHigh:        colors.high,
                colorVeryHigh:    colors.veryHigh,
            });
        } catch (e) {
            saveError = e as string;
        } finally {
            isSaving = false;
        }
    });
</script>

<div class="wizard-screen">
    <button class="flag-btn" onclick={() => cycleLanguage($locale ?? "en", locale.set)}>
        {flags[$locale ?? "en"]}
    </button>

    <div class="section">
        <h2>{$_("wizard.completion.title")}</h2>
        <p class="hint-text">{$_("wizard.completion.subtitle")}</p>
    </div>

    <!-- Zusammenfassung -->
    <div class="section">
        <div class="summary-grid">

            <div class="summary-row">
                <span class="summary-label">{$_("wizard.completion.summary_sensor")}</span>
                <span class="summary-value">{sensor.toUpperCase()}</span>
            </div>

            <div class="summary-row">
                <span class="summary-label">{$_("wizard.completion.summary_region")}</span>
                <span class="summary-value">{regionLabel(region)}</span>
            </div>

            <div class="summary-row">
                <span class="summary-label">{$_("wizard.completion.summary_username")}</span>
                <span class="summary-value">{username}</span>
            </div>

            <div class="summary-row">
                <span class="summary-label">{$_("wizard.completion.summary_unit")}</span>
                <span class="summary-value">{formatUnit(unit)}</span>
            </div>

            <div class="summary-row">
                <span class="summary-label">{$_("wizard.completion.summary_threshold_low")}</span>
                <span class="summary-value">{formatMin()}</span>
            </div>

            <div class="summary-row">
                <span class="summary-label">{$_("wizard.completion.summary_threshold_high")}</span>
                <span class="summary-value">{formatMax()}</span>
            </div>

            <div class="summary-row">
                <span class="summary-label">{$_("wizard.completion.summary_autostart")}</span>
                <span class="summary-value">
                    {autostart ? $_("wizard.completion.autostart_on") : $_("wizard.completion.autostart_off")}
                </span>
            </div>

            <div class="summary-row summary-row-colors">
                <span class="summary-label">{$_("wizard.completion.summary_colors")}</span>
                <div class="summary-color-row">
                    <div class="summary-swatch" style="background-color: {colors.veryHigh}" title={$_("wizard.settings.zone_very_high")}></div>
                    <div class="summary-swatch" style="background-color: {colors.high}" title={$_("wizard.settings.zone_high")}></div>
                    <div class="summary-swatch" style="background-color: {colors.normal}" title={$_("wizard.settings.zone_normal")}></div>
                    <div class="summary-swatch" style="background-color: {colors.low}" title={$_("wizard.settings.zone_low")}></div>
                    <div class="summary-swatch" style="background-color: {colors.criticalLow}" title={$_("wizard.settings.zone_critical_low")}></div>
                </div>
            </div>

        </div>
    </div>

    {#if isSaving}
        <p class="hint-text">{$_("wizard.completion.saving")}</p>
    {/if}

    {#if saveError}
        <p class="error-text">{$_("wizard.completion.error")}: {saveError}</p>
    {/if}

    <div class="actions">
        <button class="btn-back" onclick={onBack} disabled={isSaving}>
            {$_("wizard.buttons.back")}
        </button>
        <button class="btn-submit" disabled={isSaving || !!saveError} onclick={onFinish}>
            {$_("wizard.completion.finish")}
        </button>
    </div>
</div>