<script lang="ts">
    import { _ } from "svelte-i18n";
    import "$lib/styles/wizard.css";
    import { onMount } from "svelte";

    let { username, password, region, onSuccess, onFail }: {
        username: string;
        password: string;
        region: string;
        onSuccess: () => void;
        onFail: (error: string) => void;
    } = $props();

    onMount(async () => {
        try {
            const { invoke } = await import("@tauri-apps/api/core");
            await invoke("validate_credentials", { username, password, region });
            onSuccess();
        } catch (e: any) {
            onFail(e as string);
        }
    });
</script>

<div class="wizard-screen">
    <div class="auth-loading">
        <div class="spinner"></div>
        <p>{$_("wizard.validation.checking")}</p>
    </div>
</div>