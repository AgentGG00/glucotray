<script lang="ts">
    import { waitLocale } from "svelte-i18n";
    import WizardStep1 from "$lib/components/WizardStep1.svelte";

    let step = $state(1);
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

    function handleCancel() {
        step = 1;
        wizardData = { sensor: "", region: "", username: "", password: "" };
    }
</script>

{#await waitLocale()}
    <div class="loading">...</div>
{:then}
    {#if step === 1}
        <WizardStep1 onNext={handleStep1} onCancel={handleCancel} />
    {/if}
{/await}