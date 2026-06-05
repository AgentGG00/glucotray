<script lang="ts">
    import { _, locale } from "svelte-i18n";
    import { cycleLanguage, flags } from "$lib/app";
    import "$lib/styles/wizard.css";

    // --- Typen ---
    type Unit = "mmol" | "mgdl";

    interface Settings {
        unit: Unit;
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
    }

    let { onNext, onBack }: {
        onNext: (data: Settings) => void;
        onBack: () => void;
    } = $props();

    // --- Konstanten ---
    const MIN_VALUES_MMOL = [2.8, 3.0, 3.2, 3.5, 3.8, 4.0, 4.2, 4.5];
    const MAX_VALUES_MMOL = [
        8.0, 8.2, 8.5, 8.8,
        9.0, 9.2, 9.5, 9.8,
        10.0, 10.2, 10.5, 10.8,
        11.0, 11.2, 11.5, 11.8,
        12.0, 12.2, 12.5, 12.8,
        13.0
    ];

    const MMOL_TO_MGDL = 18.0182;

    function mmolToMgdl(val: number): number {
        return Math.round(val * MMOL_TO_MGDL);
    }

    function formatValue(val: number, unit: Unit): string {
        if (unit === "mgdl") return `${mmolToMgdl(val)} mg/dL`;
        return `${val.toFixed(1)} mmol/L`;
    }

    // --- Defaults ---
    const DEFAULT_COLORS = {
        criticalLow: "#C62828",
        low:         "#EF6C00",
        normal:      "#2E7D32",
        high:        "#F9A825",
        veryHigh:    "#D84315",
    };

    // --- State ---
    let unit      = $state<Unit | null>(null);
    let minMmol   = $state<number | null>(null);
    let maxMmol   = $state<number | null>(null);
    let autostart = $state(false);
    let colors    = $state({ ...DEFAULT_COLORS });

    // --- Derived ---
    let dropdownsActive = $derived(unit !== null);

    let canProceed = $derived(
        unit !== null &&
        minMmol !== null &&
        maxMmol !== null
    );

    let availableMaxValues = $derived(
        minMmol !== null
            ? MAX_VALUES_MMOL.filter(v => v > minMmol!)
            : MAX_VALUES_MMOL
    );

    // --- Handler ---
    function handleUnitChange(newUnit: Unit) {
        unit = newUnit;
    }

    function handleMinChange(e: Event) {
        const val = parseFloat((e.target as HTMLSelectElement).value);
        minMmol = isNaN(val) ? null : val;
        if (maxMmol !== null && minMmol !== null && minMmol >= maxMmol) {
            maxMmol = null;
        }
    }

    function handleMaxChange(e: Event) {
        const val = parseFloat((e.target as HTMLSelectElement).value);
        maxMmol = isNaN(val) ? null : val;
    }

    function handleNext() {
        if (!canProceed) return;
        onNext({
            unit: unit!,
            minMmol: minMmol!,
            maxMmol: maxMmol!,
            autostart,
            colors: { ...colors },
        });
    }
</script>

<div class="wizard-screen">
    <button class="flag-btn" onclick={() => cycleLanguage($locale ?? "en", locale.set)}>
        {flags[$locale ?? "en"]}
    </button>

    <!-- Einheit -->
    <div class="section">
        <h2>{$_("wizard.settings.unit")}</h2>
        <div class="option-group">
            <label class="option" class:selected={unit === "mgdl"}>
                <input
                    type="radio"
                    name="unit"
                    value="mgdl"
                    checked={unit === "mgdl"}
                    onchange={() => handleUnitChange("mgdl")}
                />
                {$_("wizard.settings.unit_mgdl")}
            </label>
            <label class="option" class:selected={unit === "mmol"}>
                <input
                    type="radio"
                    name="unit"
                    value="mmol"
                    checked={unit === "mmol"}
                    onchange={() => handleUnitChange("mmol")}
                />
                {$_("wizard.settings.unit_mmol")}
            </label>
        </div>
    </div>

    <!-- Min-Grenzwert -->
    <div class="section">
        <h2>{$_("wizard.settings.threshold_low")}</h2>
        <select
            class="select-input"
            class:select-disabled={!dropdownsActive}
            disabled={!dropdownsActive}
            value={minMmol ?? ""}
            onchange={handleMinChange}
        >
            <option value="" disabled selected={minMmol === null}>
                {dropdownsActive ? $_("wizard.settings.select_placeholder") : $_("wizard.settings.select_unit_first")}
            </option>
            {#each MIN_VALUES_MMOL as val}
                <option value={val} selected={minMmol === val}>
                    {unit ? formatValue(val, unit) : val}
                </option>
            {/each}
        </select>
    </div>

    <!-- Max-Grenzwert -->
    <div class="section">
        <h2>{$_("wizard.settings.threshold_high")}</h2>
        <select
            class="select-input"
            class:select-disabled={!dropdownsActive}
            disabled={!dropdownsActive || minMmol === null}
            value={maxMmol ?? ""}
            onchange={handleMaxChange}
        >
            <option value="" disabled selected={maxMmol === null}>
                {dropdownsActive ? $_("wizard.settings.select_placeholder") : $_("wizard.settings.select_unit_first")}
            </option>
            {#each availableMaxValues as val}
                <option value={val} selected={maxMmol === val}>
                    {unit ? formatValue(val, unit) : val}
                </option>
            {/each}
        </select>
    </div>

    <!-- Farbschema -->
    <div class="section">
        <h2>{$_("wizard.settings.color_scheme")}</h2>
        <p class="hint-text">{$_("wizard.settings.color_hint")}</p>
        <div class="color-grid">
            <div class="color-row">
                <label class="color-label" for="color-very-high">{$_("wizard.settings.zone_very_high")}</label>
                <div class="color-input-wrapper">
                    <div class="color-swatch" style="background-color: {colors.veryHigh}"></div>
                    <input id="color-very-high" type="color" bind:value={colors.veryHigh} class="color-picker" />
                    <span class="color-hex">{colors.veryHigh}</span>
                </div>
            </div>
            <div class="color-row">
                <label class="color-label" for="color-high">{$_("wizard.settings.zone_high")}</label>
                <div class="color-input-wrapper">
                    <div class="color-swatch" style="background-color: {colors.high}"></div>
                    <input id="color-high" type="color" bind:value={colors.high} class="color-picker" />
                    <span class="color-hex">{colors.high}</span>
                </div>
            </div>
            <div class="color-row">
                <label class="color-label" for="color-normal">{$_("wizard.settings.zone_normal")}</label>
                <div class="color-input-wrapper">
                    <div class="color-swatch" style="background-color: {colors.normal}"></div>
                    <input id="color-normal" type="color" bind:value={colors.normal} class="color-picker" />
                    <span class="color-hex">{colors.normal}</span>
                </div>
            </div>
            <div class="color-row">
                <label class="color-label" for="color-low">{$_("wizard.settings.zone_low")}</label>
                <div class="color-input-wrapper">
                    <div class="color-swatch" style="background-color: {colors.low}"></div>
                    <input id="color-low" type="color" bind:value={colors.low} class="color-picker" />
                    <span class="color-hex">{colors.low}</span>
                </div>
            </div>
            <div class="color-row">
                <label class="color-label" for="color-critical-low">{$_("wizard.settings.zone_critical_low")}</label>
                <div class="color-input-wrapper">
                    <div class="color-swatch" style="background-color: {colors.criticalLow}"></div>
                    <input id="color-critical-low" type="color" bind:value={colors.criticalLow} class="color-picker" />
                    <span class="color-hex">{colors.criticalLow}</span>
                </div>
            </div>
        </div>
    </div>

    <!-- Autostart -->
    <div class="section">
        <label class="check-item">
            <input type="checkbox" bind:checked={autostart} />
            {$_("wizard.settings.autostart_label")}
        </label>
    </div>

    <div class="actions">
        <button class="btn-back" onclick={onBack}>
            {$_("wizard.buttons.back")}
        </button>
        <button class="btn-next" disabled={!canProceed} onclick={handleNext}>
            {$_("wizard.buttons.next")}
        </button>
    </div>
</div>