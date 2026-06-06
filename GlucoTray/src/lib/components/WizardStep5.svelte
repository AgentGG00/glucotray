<script lang="ts">
    import { _, locale } from "svelte-i18n";
    import { cycleLanguage, flags } from "$lib/app";
    import "$lib/styles/wizard.css";
    import { onMount } from "svelte";

    const MGDL_TO_MMOL = 18.0182;

    function mgdlToMmol(val: number): string {
        return (val / MGDL_TO_MMOL).toFixed(1);
    }

    let {
        sensor,
        region,
        username,
        unit,
        thresholdLowMgdl,
        thresholdHighMgdl,
        autostart,
        colors,
        onBack,
        onFinish,
    }: {
        sensor: string;
        region: string;
        username: string;
        unit: "mmol" | "mgdl";
        thresholdLowMgdl: number;
        thresholdHighMgdl: number;
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
        if (unit === "mgdl") return `${thresholdLowMgdl} mg/dL`;
        return `${mgdlToMmol(thresholdLowMgdl)} mmol/L`;
    }

    function formatMax(): string {
        if (unit === "mgdl") return `${thresholdHighMgdl} mg/dL`;
        return `${mgdlToMmol(thresholdHighMgdl)} mmol/L`;
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
                thresholdLowMgdl,
                thresholdHighMgdl,
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