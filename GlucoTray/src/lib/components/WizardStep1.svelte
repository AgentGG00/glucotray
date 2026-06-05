<script lang="ts">
    import { _, locale } from "svelte-i18n";
    import { cycleLanguage, flags } from "$lib/app";
    import "$lib/styles/wizard.css";

    let selectedSensor: "g6" | "g7" | null = $state(null);
    let selectedRegion: "us" | "ous" | "jp" | null = $state(null);
    let requirements = $state({
        app_installed: false,
        share_active: false,
        follower_added: false,
    });

    let canProceed = $derived(
        selectedSensor !== null &&
        selectedRegion !== null &&
        Object.values(requirements).every(Boolean)
    );

    let { onNext, onCancel }: {
        onNext: (data: { sensor: string; region: string }) => void;
        onCancel: () => void;
    } = $props();
</script>

<div class="wizard-screen">
    <button class="flag-btn" onclick={() => cycleLanguage($locale ?? "en", locale.set)}>
        {flags[$locale ?? "en"]}
    </button>

    <div class="section">
        <h2>{$_("wizard.sensor.title")}</h2>
        <div class="option-group">
            <label class="option" class:selected={selectedSensor === "g6"}>
                <input type="radio" name="sensor" value="g6" bind:group={selectedSensor} />
                {$_("wizard.sensor.g6")}
            </label>
            <label class="option" class:selected={selectedSensor === "g7"}>
                <input type="radio" name="sensor" value="g7" bind:group={selectedSensor} />
                {$_("wizard.sensor.g7")}
            </label>
        </div>
    </div>

    <div class="section">
        <h2>{$_("wizard.region.title")}</h2>
        <div class="option-group">
            <label class="option" class:selected={selectedRegion === "us"}>
                <input type="radio" name="region" value="us" bind:group={selectedRegion} />
                {$_("wizard.region.us")}
            </label>
            <label class="option" class:selected={selectedRegion === "ous"}>
                <input type="radio" name="region" value="ous" bind:group={selectedRegion} />
                {$_("wizard.region.ous")}
            </label>
            <label class="option" class:selected={selectedRegion === "jp"}>
                <input type="radio" name="region" value="jp" bind:group={selectedRegion} />
                {$_("wizard.region.jp")}
            </label>
        </div>
    </div>

    <div class="section">
        <h2>{$_("wizard.requirements.title")}</h2>
        <div class="checklist">
            <label class="check-item">
                <input type="checkbox" bind:checked={requirements.app_installed} />
                {$_("wizard.requirements.app_installed")}
            </label>
            <label class="check-item">
                <input type="checkbox" bind:checked={requirements.share_active} />
                {$_("wizard.requirements.share_active")}
            </label>
            <label class="check-item">
                <input type="checkbox" bind:checked={requirements.follower_added} />
                {$_("wizard.requirements.follower_added")}
            </label>
        </div>
    </div>

    <div class="actions">
        <button class="btn-cancel" onclick={onCancel}>
            {$_("wizard.buttons.cancel")}
        </button>
        <button class="btn-next" disabled={!canProceed} onclick={() => onNext({ sensor: selectedSensor!, region: selectedRegion! })}>
            {$_("wizard.buttons.next")}
        </button>
    </div>
</div>