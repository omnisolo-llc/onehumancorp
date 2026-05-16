### Title
[Strategy] Geographic Expansion: Multilingual AI Teammates

**Problem Statement:**
Fatima (Food Cart owner) speaks limited English. She needs a tool that doesn't just translate the UI, but *thinks* and *communicates* in her native language. Most tools are English-first and translation-second.

**Research Report:**
- There are ~27.1M non-employer businesses in the US; a significant portion are owned by non-native English speakers.
- Global expansion candidates: Spanish (LATAM), Portuguese (Brazil), Arabic (MENA).
- OHC's Rust/Tauri stack allows for native localization, but the *AI Agents* must support multi-lingual context.

**Design Doc:**
- **High-Level Architecture:**
    - **Entity Types:** `LocaleContext`, `LocalizedPrompt`, `UserPreference`.
    - **Key Relationships:** `UserPreference` defines the active `LocaleContext`; `LocalizedPrompt` overrides base system prompts based on locale.
    - **Integration Points:** I18n resource files, LLM System Prompt Injection.
- **Mobile UX Flow (375px First):**
    1. **Onboarding:** "Select your language" screen (Flags + Language names).
    2. **Transformation:** Entire UI and agent greeting ("Hola, soy tu asistente") switch instantly.
    3. **Consistency:** All notifications and briefings arrive in the selected language.
- **AI Agent Integration Points:** `ContextManager` detects `UserLocale` and injects language-specific formatting instructions into all LLM calls.

**Implementation Prompt:**
Develop a roadmap and initial implementation for "Multilingual AI Teammates." Ensure that the LLM system prompts for core agents (The Ambassador, The Manager) are localized. Update the `ContextManager` to handle language-specific instructions. Create a test suite for 1-tap onboarding in Spanish.

**Priority:** P2
**Estimated Scope:** Medium
