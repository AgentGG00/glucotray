<script lang="ts">
    import { waitLocale } from "svelte-i18n";
    import WizardStep1 from "$lib/components/WizardStep1.svelte";
    import WizardStep2 from "$lib/components/WizardStep2.svelte";
    import WizardStep3 from "$lib/components/WizardStep3.svelte";
    import WizardStep4 from "$lib/components/WizardStep4.svelte";
    import WizardStep5 from "$lib/components/WizardStep5.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { getCurrentWindow } from "@tauri-apps/api/window";

    let step = $state(1);
    let authError = $state("");
    let wizardData = $state({
        sensor: "",
        region: "",
        username: "",
        password: "",
        unit: "" as "mmol" | "mgdl" | "",
        minMmol: 0,
        maxMmol: 0,
        autostart: false,
        colors: {
            criticalLow: "#C62828",
            low:         "#EF6C00",
            normal:      "#2E7D32",
            high:        "#F9A825",
            veryHigh:    "#D84315",
        },
    });

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

    function handleStep3Success() {
        step = 4;
    }

    function handleStep3Fail(error: string) {
        authError = error;
        step = 2;
    }

    function handleStep4(data: {
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
    }) {
        wizardData.unit = data.unit;
        wizardData.minMmol = data.minMmol;
        wizardData.maxMmol = data.maxMmol;
        wizardData.autostart = data.autostart;
        wizardData.colors = data.colors;
        step = 5;
    }

    async function handleFinish() {
        await invoke("start_worker");
        await getCurrentWindow().close();
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
            minMmol: 0,
            maxMmol: 0,
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
</script>

{#await waitLocale()}
    <div class="loading">...</div>
{:then}
    {#if step === 1}
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
            minMmol={wizardData.minMmol}
            maxMmol={wizardData.maxMmol}
            autostart={wizardData.autostart}
            colors={wizardData.colors}
            onBack={() => step = 4}
            onFinish={handleFinish}
        />
    {/if}
{/await}