<script lang="ts">
    import { _, locale, getLocaleFromNavigator } from "svelte-i18n";
    import { cycleLanguage, flags } from "$lib/app";
    import "$lib/styles/wizard.css";
    import { parsePhoneNumberFromString, AsYouType, getCountryCallingCode } from "libphonenumber-js";
    import { locale as osLocale } from "@tauri-apps/plugin-os";
    import { onMount } from "svelte";

    let loginType: "email" | "phone" = $state("email");
    let username = $state("");
    let password = $state("");
    let isLoading = $state(false);
    let error = $state("");
    let currentCountry = $state("US");

    let passwordVisible = $state(false);
    let revealedIndex = $state<number | null>(null);
    let revealTimer: ReturnType<typeof setTimeout> | null = null;

    let passwordDisplay = $derived(
        passwordVisible
            ? password
            : password
                .split("")
                .map((_, i) => (i === revealedIndex ? password[i] : "•"))
                .join("")
    );

    function handlePasswordInput(e: Event) {
        const target = e.target as HTMLInputElement;
        const displayed = target.value;

        if (displayed.length > password.length) {
            const newChar = displayed[displayed.length - 1];
            password = password + newChar;
        } else if (displayed.length < password.length) {
            const diff = password.length - displayed.length;
            password = password.slice(0, password.length - diff);
        }

        if (!passwordVisible && password.length > 0) {
            revealedIndex = password.length - 1;
            if (revealTimer) clearTimeout(revealTimer);
            revealTimer = setTimeout(() => {
                revealedIndex = null;
            }, 3000);
        }
    }

    function handlePasswordPaste(e: ClipboardEvent) {
        e.preventDefault();
        const pasted = e.clipboardData?.getData("text") ?? "";
        if (!pasted) return;
        password = password + pasted;
        revealedIndex = null;
        if (revealTimer) clearTimeout(revealTimer);
    }

    function togglePasswordVisibility() {
        passwordVisible = !passwordVisible;
        if (!passwordVisible) {
            revealedIndex = null;
            if (revealTimer) clearTimeout(revealTimer);
        }
    }

    const COMMON_DOMAINS = [
        "gmail.com", "googlemail.com",
        "outlook.com", "hotmail.com", "hotmail.de", "live.com", "live.de", "msn.com",
        "yahoo.com", "yahoo.de",
        "icloud.com", "me.com", "mac.com",
        "gmx.de", "gmx.net", "gmx.at", "gmx.ch",
        "web.de", "t-online.de", "freenet.de", "arcor.de",
        "protonmail.com", "proton.me",
        "aol.com",
    ];

    function levenshtein(a: string, b: string): number {
        const dp = Array.from({ length: a.length + 1 }, (_, i) =>
            Array.from({ length: b.length + 1 }, (_, j) => (i === 0 ? j : j === 0 ? i : 0))
        );
        for (let i = 1; i <= a.length; i++) {
            for (let j = 1; j <= b.length; j++) {
                dp[i][j] = a[i - 1] === b[j - 1]
                    ? dp[i - 1][j - 1]
                    : 1 + Math.min(dp[i - 1][j], dp[i][j - 1], dp[i - 1][j - 1]);
            }
        }
        return dp[a.length][b.length];
    }

    function getSuggestedDomain(domain: string): string | null {
        if (COMMON_DOMAINS.includes(domain)) return null;
        let best: string | null = null;
        let bestDist = Infinity;
        for (const known of COMMON_DOMAINS) {
            const dist = levenshtein(domain, known);
            if (dist < bestDist) { bestDist = dist; best = known; }
        }
        return bestDist <= 2 ? best : null;
    }

    function extractCountry(raw: string): string {
        const parts = raw.split("-");
        if (parts.length >= 2) return parts[1].toUpperCase();
        const lang = parts[0].toLowerCase();
        const langToCountry: Record<string, string> = {
            de: "DE", ja: "JP", en: "US", fr: "FR", es: "ES",
            it: "IT", pt: "PT", nl: "NL", pl: "PL", ru: "RU",
            zh: "CN", ko: "KR",
        };
        return langToCountry[lang] ?? "US";
    }

    onMount(async () => {
        try {
            const raw = await osLocale();
            if (!raw) throw new Error("no locale");
            currentCountry = extractCountry(raw);
        } catch {
            const nav = getLocaleFromNavigator() ?? "en-US";
            currentCountry = extractCountry(nav);
        }
    });

    function getPlaceholder(): string {
        try {
            const code = getCountryCallingCode(currentCountry as any);
            return `+${code} ...`;
        } catch {
            return "+1 234 567 8900";
        }
    }

    let phoneValidation = $derived.by(() => {
        if (loginType !== "phone" || username.trim() === "") return null;
        const parsed = parsePhoneNumberFromString(username, currentCountry as any);
        if (!parsed) return { valid: false, message: $_("wizard.credentials.phone_invalid"), formatted: null };
        if (!parsed.isValid()) return { valid: false, message: $_("wizard.credentials.phone_invalid"), formatted: null };
        return { valid: true, message: $_("wizard.credentials.phone_valid"), formatted: parsed.format("E.164") };
    });

    let emailValidation = $derived.by(() => {
        if (loginType !== "email" || username.trim() === "") return null;
        const emailRegex = /^[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}$/;
        if (!emailRegex.test(username)) {
            return { valid: false, message: $_("wizard.credentials.email_invalid"), suggestion: null };
        }
        const domain = username.split("@")[1].toLowerCase();
        const suggestion = getSuggestedDomain(domain);
        return {
            valid: true,
            message: $_("wizard.credentials.email_valid"),
            suggestion: suggestion
                ? `${$_("wizard.credentials.email_suggestion")} ${username.split("@")[0]}@${suggestion}?`
                : null,
        };
    });

    let canProceed = $derived(
        password.trim() !== "" &&
        (loginType === "email" ? emailValidation?.valid === true : phoneValidation?.valid === true)
    );

    let { onNext, onBack, region, externalError = "" }: {
        onNext: (data: { username: string; password: string }) => void;
        onBack: () => void;
        region: string;
        externalError?: string;
    } = $props();

    function applySuggestion() {
        if (!emailValidation?.suggestion) return;
        const local = username.split("@")[0];
        const domain = emailValidation.suggestion.split("@")[1].replace("?", "");
        username = `${local}@${domain}`;
    }

    function handlePhoneInput(e: Event) {
        const target = e.target as HTMLInputElement;
        let filtered = target.value.replace(/[^\d+\s\-().]/g, "");
        if (filtered.startsWith("0") && !filtered.startsWith("00")) {
            const countryCallingCode = getCountryCallingCode(currentCountry as any);
            filtered = `+${countryCallingCode}${filtered.slice(1)}`;
        }
        if (target.value !== filtered) target.value = filtered;
        const formatter = new AsYouType(currentCountry as any);
        username = formatter.input(filtered);
    }

    function handlePhoneKeydown(e: KeyboardEvent) {
        if (e.key === "Enter") {
            e.preventDefault();
            document.getElementById("password-display")?.focus();
            return;
        }
        const allowed = /[\d+\s\-().\b]/;
        if (!allowed.test(e.key) && !["Backspace", "Delete", "ArrowLeft", "ArrowRight", "Tab"].includes(e.key)) {
            e.preventDefault();
        }
    }

    function handleUsernameKeydown(e: KeyboardEvent) {
        if (e.key === "Enter") {
            e.preventDefault();
            document.getElementById("password-display")?.focus();
        }
    }

    function handleNext() {
        if (!canProceed) return;
        const finalUsername = loginType === "phone" && phoneValidation?.formatted
            ? phoneValidation.formatted
            : username;
        onNext({ username: finalUsername, password });
    }
</script>

<div class="wizard-screen">
    <button class="flag-btn" onclick={() => cycleLanguage($locale ?? "en", locale.set)}>
        {flags[$locale ?? "en"]}
    </button>

    <div class="section">
        <h2>{$_("wizard.credentials.title")}</h2>

        <div class="option-group">
            <label class="option" class:selected={loginType === "email"}>
                <input type="radio" name="loginType" value="email" bind:group={loginType} />
                {$_("wizard.credentials.type_email")}
            </label>
            <label class="option" class:selected={loginType === "phone"}>
                <input type="radio" name="loginType" value="phone" bind:group={loginType} />
                {$_("wizard.credentials.type_phone")}
            </label>
        </div>

        <div class="field-group">
            <label class="field-label" for="username">
                {loginType === "email" ? $_("wizard.credentials.email") : $_("wizard.credentials.phone")}
            </label>

            {#if loginType === "phone"}
                <input
                    id="username"
                    type="tel"
                    class="text-input"
                    class:input-valid={phoneValidation?.valid === true}
                    class:input-invalid={phoneValidation?.valid === false}
                    value={username}
                    oninput={handlePhoneInput}
                    onkeydown={handlePhoneKeydown}
                    disabled={isLoading}
                    placeholder={getPlaceholder()}
                />
            {:else}
                <input
                    id="username"
                    type="email"
                    class="text-input"
                    class:input-valid={emailValidation?.valid === true}
                    class:input-invalid={emailValidation?.valid === false}
                    bind:value={username}
                    onkeydown={handleUsernameKeydown}
                    disabled={isLoading}
                    autocomplete="email"
                    placeholder="user@example.com"
                />
            {/if}

            {#if loginType === "phone" && phoneValidation}
                <span class="validation-text" class:valid={phoneValidation.valid} class:invalid={!phoneValidation.valid}>
                    {phoneValidation.message}
                </span>
            {/if}
            {#if loginType === "email" && emailValidation}
                <span class="validation-text" class:valid={emailValidation.valid} class:invalid={!emailValidation.valid}>
                    {emailValidation.message}
                </span>
                {#if emailValidation.suggestion}
                    <button class="suggestion-btn" onclick={applySuggestion}>
                        {emailValidation.suggestion}
                    </button>
                {/if}
            {/if}
        </div>

        <div class="field-group">
            <label class="field-label" for="password-display">{$_("wizard.credentials.password")}</label>
            <div class="input-wrapper">
                <input
                    type="password"
                    class="password-manager-input"
                    bind:value={password}
                    autocomplete="current-password"
                    tabindex="-1"
                />
                {#if passwordVisible}
                    <input
                        id="password-display"
                        type="text"
                        class="text-input password-display"
                        bind:value={password}
                        disabled={isLoading}
                        autocomplete="off"
                        spellcheck={false}
                    />
                {:else}
                    <input
                        id="password-display"
                        type="text"
                        class="text-input password-display"
                        value={passwordDisplay}
                        oninput={handlePasswordInput}
                        onpaste={handlePasswordPaste}
                        disabled={isLoading}
                        autocomplete="off"
                        spellcheck={false}
                    />
                {/if}
                <button
                    type="button"
                    class="eye-btn"
                    onclick={togglePasswordVisibility}
                    tabindex="-1"
                    disabled={isLoading}
                >
                    {#if passwordVisible}
                        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/>
                            <circle cx="12" cy="12" r="3"/>
                        </svg>
                    {:else}
                        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94"/>
                            <path d="M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19"/>
                            <line x1="1" y1="1" x2="23" y2="23"/>
                        </svg>
                    {/if}
                </button>
            </div>
        </div>
    </div>

    {#if error}
        <p class="error-text">{error}</p>
    {:else if externalError}
        <p class="error-text">{$_(`wizard.errors.${externalError}`)}</p>
    {/if}

    <div class="actions">
        <button class="btn-back" onclick={onBack} disabled={isLoading}>
            {$_("wizard.buttons.back")}
        </button>
        <button class="btn-next" disabled={!canProceed || isLoading} onclick={handleNext}>
            {$_("wizard.buttons.next")}
        </button>
    </div>
</div>