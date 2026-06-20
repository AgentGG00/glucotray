<script lang="ts">
    import { waitLocale } from "svelte-i18n";
    import WizardStep0 from "$lib/components/WizardStep0.svelte";
    import WizardStep1 from "$lib/components/WizardStep1.svelte";
    import WizardStep2 from "$lib/components/WizardStep2.svelte";
    import WizardStep3 from "$lib/components/WizardStep3.svelte";
    import WizardStep4 from "$lib/components/WizardStep4.svelte";
    import WizardStep5 from "$lib/components/WizardStep5.svelte";
    import Settings from "$lib/components/Settings.svelte";
    import PrivacyPolicy from "$lib/components/PrivacyPolicy.svelte";
    import TermsOfUse from "$lib/components/TermsOfUse.svelte";
    import Disclaimer from "$lib/components/Disclaimer.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    type Mode =
        | "loading"
        | "wizard"
        | "settings"
        | "settings-edit-credentials"
        | "settings-validate-credentials"
        | "settings-privacy"
        | "settings-terms"
        | "settings-disclaimer";

    let mode = $state<Mode>("loading");
    let step = $state(0);
    let authError = $state("");
    let settingsRegion = $state("");
    let settingsCredentials = $state({ username: "", password: "" });

    let wizardData = $state({
        sensor: "",
        region: "",
        username: "",
        password: "",
        unit: "" as "mmol" | "mgdl" | "",
        thresholdLowMgdl: 0,
        thresholdHighMgdl: 0,
        autostart: false,
        colors: {
            criticalLow: "#C62828",
            low:         "#EF6C00",
            normal:      "#2E7D32",
            high:        "#F9A825",
            veryHigh:    "#D84315",
        },
    });

    onMount(async () => {
        const wizardDone = await invoke<boolean>("get_wizard_status");
        mode = wizardDone ? "settings" : "wizard";
    });

    function handleStep0() {
        step = 1;
    }

    function handleStep1(data: { sensor: string; region: string }) {
        wizardData.sensor = data.sensor;
        wizardData.region = data.region;
        step = 2;
    }

    function handleStep2(data: { username: string; password: string }) {
        wizardData.username = data.username;
        wizardData.password = data.password;
        authError = "";
        step = 3;
    }

    function handleStep3Success() { step = 4; }

    function handleStep3Fail(error: string) {
        authError = error;
        step = 2;
    }

    function handleStep4(data: {
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
    }) {
        wizardData.unit = data.unit;
        wizardData.thresholdLowMgdl = data.thresholdLowMgdl;
        wizardData.thresholdHighMgdl = data.thresholdHighMgdl;
        wizardData.autostart = data.autostart;
        wizardData.colors = data.colors;
        step = 5;
    }

    async function handleFinish() {
        await invoke("restart_app");
    }

    function handleCancel() {
        step = 1;
        authError = "";
        wizardData = {
            sensor: "",
            region: "",
            username: "",
            password: "",
            unit: "",
            thresholdLowMgdl: 0,
            thresholdHighMgdl: 0,
            autostart: false,
            colors: {
                criticalLow: "#C62828",
                low:         "#EF6C00",
                normal:      "#2E7D32",
                high:        "#F9A825",
                veryHigh:    "#D84315",
            },
        };
    }

    function handleChangeCredentials(region: string) {
        settingsRegion = region;
        authError = "";
        mode = "settings-edit-credentials";
    }

    function handleSettingsCredentialsNext(data: { username: string; password: string }) {
        settingsCredentials = data;
        authError = "";
        mode = "settings-validate-credentials";
    }

    function handleSettingsCredentialsSuccess() {
        mode = "settings";
    }

    function handleSettingsCredentialsFail(error: string) {
        authError = error;
        mode = "settings-edit-credentials";
    }

    function handleSettingsCredentialsBack() {
        authError = "";
        mode = "settings";
    }

    function handleShowPrivacy() {
        mode = "settings-privacy";
    }

    function handleShowTerms() {
        mode = "settings-terms";
    }

    function handleShowDisclaimer() {
        mode = "settings-disclaimer";
    }

    function handleBackToSettings() {
        mode = "settings";
    }
</script>

{#await waitLocale()}
    <div class="loading">...</div>
{:then}
    {#if mode === "loading"}
        <div class="loading">...</div>
    {:else if mode === "wizard"}
        {#if step === 0}
            <WizardStep0 onNext={handleStep0} />
        {:else if step === 1}
            <WizardStep1 onNext={handleStep1} onCancel={handleCancel} />
        {:else if step === 2}
            <WizardStep2
                onNext={handleStep2}
                onBack={() => step = 1}
                region={wizardData.region}
                externalError={authError}
            />
        {:else if step === 3}
            <WizardStep3
                username={wizardData.username}
                password={wizardData.password}
                region={wizardData.region}
                onSuccess={handleStep3Success}
                onFail={handleStep3Fail}
            />
        {:else if step === 4}
            <WizardStep4
                onNext={handleStep4}
                onBack={() => step = 3}
            />
        {:else if step === 5}
            <WizardStep5
                sensor={wizardData.sensor}
                region={wizardData.region}
                username={wizardData.username}
                unit={wizardData.unit as "mmol" | "mgdl"}
                thresholdLowMgdl={wizardData.thresholdLowMgdl}
                thresholdHighMgdl={wizardData.thresholdHighMgdl}
                autostart={wizardData.autostart}
                colors={wizardData.colors}
                onBack={() => step = 4}
                onFinish={handleFinish}
            />
        {/if}
    {:else if mode === "settings"}
        <Settings
            onChangeCredentials={handleChangeCredentials}
            onShowPrivacy={handleShowPrivacy}
            onShowTerms={handleShowTerms}
            onShowDisclaimer={handleShowDisclaimer}
        />
    {:else if mode === "settings-edit-credentials"}
        <WizardStep2
            onNext={handleSettingsCredentialsNext}
            onBack={handleSettingsCredentialsBack}
            region={settingsRegion}
            externalError={authError}
        />
    {:else if mode === "settings-validate-credentials"}
        <WizardStep3
            username={settingsCredentials.username}
            password={settingsCredentials.password}
            region={settingsRegion}
            onSuccess={handleSettingsCredentialsSuccess}
            onFail={handleSettingsCredentialsFail}
        />
    {:else if mode === "settings-privacy"}
        <PrivacyPolicy onBack={handleBackToSettings} />
    {:else if mode === "settings-terms"}
        <TermsOfUse onBack={handleBackToSettings} />
    {:else if mode === "settings-disclaimer"}
        <Disclaimer onBack={handleBackToSettings} />
    {/if}
{/await}