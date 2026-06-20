<script lang="ts">
    import { _, locale } from "svelte-i18n";
    import { cycleLanguage, flags } from "$lib/app";
    import "$lib/styles/wizard.css";
    import { marked } from "marked";

    const LEGAL_VERSION = "2026-06";

    type LegalDoc = "privacy-policy" | "terms-of-use" | "disclaimer";

    const DOC_ORDER: LegalDoc[] = ["privacy-policy", "terms-of-use", "disclaimer"];

    let { onNext }: {
        onNext: () => void;
    } = $props();

    let legalStep = $state(0);
    let currentDoc = $derived(DOC_ORDER[legalStep]);

    let content = $state("");
    let html = $derived(marked.parse(content) as string);
    let isLoading = $state(true);
    let error = $state("");
    let accepted = $state([false, false, false]);
    let isSaving = $state(false);

    async function loadDocument() {
        isLoading = true;
        error = "";
        try {
            const { invoke } = await import("@tauri-apps/api/core");
            content = await invoke("read_legal_document", {
                document: currentDoc,
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

    function handleAccept() {
        accepted[legalStep] = true;

        if (legalStep < DOC_ORDER.length - 1) {
            legalStep += 1;
        } else {
            finishLegal();
        }
    }

    async function finishLegal() {
        isSaving = true;
        error = "";
        try {
            const { invoke } = await import("@tauri-apps/api/core");
            await invoke("save_legal_acceptance", { legalVersion: LEGAL_VERSION });
            onNext();
        } catch (e) {
            error = e as string;
        } finally {
            isSaving = false;
        }
    }

    function docTitleKey(doc: LegalDoc): string {
        switch (doc) {
            case "privacy-policy": return "wizard.legal.privacy_title";
            case "terms-of-use": return "wizard.legal.terms_title";
            case "disclaimer": return "wizard.legal.disclaimer_title";
        }
    }
</script>

<div class="wizard-screen">
    <button class="flag-btn" onclick={() => cycleLanguage($locale ?? "en", locale.set)}>
        {flags[$locale ?? "en"]}
    </button>

    <div class="section">
        <h2>{$_(docTitleKey(currentDoc))}</h2>
        <p class="legal-progress">{legalStep + 1} / {DOC_ORDER.length}</p>

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

    {#if error}
        <p class="error-text">{error}</p>
    {/if}

    <div class="actions">
        <button
            class="btn-next"
            disabled={isLoading || isSaving || !!error}
            onclick={handleAccept}
        >
            {#if isSaving}
                {$_("wizard.legal.saving")}
            {:else if legalStep < DOC_ORDER.length - 1}
                {$_("wizard.legal.accept_continue")}
            {:else}
                {$_("wizard.legal.accept_finish")}
            {/if}
        </button>
    </div>
</div>

