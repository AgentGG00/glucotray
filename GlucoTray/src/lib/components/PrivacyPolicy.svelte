<script lang="ts">
    import { _, locale } from "svelte-i18n";
    import "$lib/styles/wizard.css";
    import { marked } from "marked";

    let { onBack }: {
        onBack: () => void;
    } = $props();

    let content = $state("");
    let html = $derived(marked.parse(content) as string);
    let isLoading = $state(true);
    let error = $state("");

    async function loadDocument() {
        isLoading = true;
        error = "";
        try {
            const { invoke } = await import("@tauri-apps/api/core");
            content = await invoke("read_legal_document", {
                document: "privacy-policy",
                lang: $locale ?? "en",
            });
        } catch (e) {
            error = e as string;
        } finally {
            isLoading = false;
        }
    }

    $effect(() => {
        loadDocument();
    });
</script>

<div class="wizard-screen">
    <div class="section">
        <h2>{$_("wizard.legal.privacy_title")}</h2>

        {#if isLoading}
            <div class="auth-loading">
                <div class="spinner"></div>
            </div>
        {:else if error}
            <p class="error-text">{error}</p>
        {:else}
            <div class="legal-document">
                {@html html}
            </div>
        {/if}
    </div>

    <div class="actions">
        <button class="btn-back" onclick={onBack}>
            {$_("wizard.buttons.back")}
        </button>
    </div>
</div>