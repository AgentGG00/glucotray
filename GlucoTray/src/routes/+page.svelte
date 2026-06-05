<script lang="ts">
    import { waitLocale } from "svelte-i18n";
    import WizardStep1 from "$lib/components/WizardStep1.svelte";
    import WizardStep2 from "$lib/components/WizardStep2.svelte";
    import WizardStep3 from "$lib/components/WizardStep3.svelte";

    let step = $state(1);
    let authError = $state("");
    let wizardData = $state({
        sensor: "",
        region: "",
        username: "",
        password: "",
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

    function handleCancel() {
        step = 1;
        authError = "";
        wizardData = { sensor: "", region: "", username: "", password: "" };
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
        <div>Settings – coming soon</div>
    {/if}
{/await}