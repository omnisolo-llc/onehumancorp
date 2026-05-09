# 1-Tap Mobile Onboarding & Store Generation

## Problem Statement
Small business owners (like Maya the baker or Carlos the handyman) are intimidated by complex setups. They abandon platforms that ask for DNS configurations, shipping zones, and tax settings before they can even see what their store will look like. The current market standard (Shopify) takes hours to configure.

## Research Report
*   **Competitor Audit:** Durable generates a website in 30 seconds but lacks business operations depth. Shopify requires significant setup time and technical knowledge.
*   **User Pain Point:** Setup complexity is the #1 pain point (73% frequency in our audit). Users want to see the value immediately.
*   **OHC Advantage:** By leveraging the Agent Department architecture, we can generate a fully functional, personalized storefront based on a simple conversational prompt or a single form.

## Design Doc
*   **UX Flow (375px Mobile First):**
    1.  **Splash:** "Describe your business in one sentence." (e.g., "I sell vegan cupcakes in Austin.")
    2.  **Processing State:** Engaging animation ("Agent Architect is building your store...").
    3.  **Reveal:** Live preview of the generated storefront with placeholder products, images, and copy tailored to the prompt.
    4.  **Action:** "Looks good, let's launch" or "Tweak the vibe."
*   **Architecture (High Level):**
    *   Input: User string.
    *   Routing: LLM Gateway to generate business profile (name, category, vibe).
    *   Agent Action: Builtin AutoDream agent triggers a workflow to create foundational database records (Tenant, Storefront, Sample Products) based on the LLM output.
    *   Output: Hydrated UI state.

## Implementation Prompt
Implement a conversational onboarding flow that takes a single user prompt and uses the AI agent system to generate a complete, previewable storefront within 30 seconds.
*   **Critical User Journey:** User enters a description -> System generates store -> User previews store on mobile UI -> User accepts and enters the dashboard.
*   **Acceptance Criteria:**
    *   Flow must be mobile-first and pass the Grandmother Test.
    *   Must use OHC Premium Design Standards (Glassmorphism, correct typography).
    *   Generation must complete in < 30s.

## Priority
P0

## Estimated Scope
Large
