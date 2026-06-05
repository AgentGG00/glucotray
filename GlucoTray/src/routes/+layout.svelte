<script lang="ts">
    import "$lib/styles/app.css";
    import { onMount } from "svelte";
    import { initTheme } from "$lib/theme";
    import { setupI18n } from "$lib/i18n";

    setupI18n();

    onMount(() => {
        initTheme();
    });
</script>

<slot />