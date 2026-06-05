<script lang="ts">
    import "$lib/styles/app.css";
    import { onMount } from "svelte";
    import { initTheme } from "$lib/theme";
    import { setupI18n } from "$lib/i18n";

    const i18nReady = setupI18n();

    onMount(() => {
        initTheme();
    });
</script>

{#await i18nReady}
    <div class="loading">...</div>
{:then}
    <slot />
{/await}