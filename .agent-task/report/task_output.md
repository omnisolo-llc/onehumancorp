issue_title: "[architecture] Instant Build Setup Wizard (Mobile-First)"
issue_description: |
  ## Problem Statement
  The onboarding friction for most ecommerce platforms is too high. Even a 10-minute setup feels like a chore for a busy founder. OHC currently implements a multi-step `SetupWizard` (e.g., asking "What's the name of your business?", "What do you sell?", "Where are you located?"). We need an "Instant Build" mode to reduce "Time to Live" for the most basic storefront to under 60 seconds by using AI to guess and fill 80% of the required fields from a single text prompt.

  ## Research Report
  - **Durable Benchmark:** Claims "Get online in 30 seconds."
  - **Wix Harmony:** Uses "vibe coding" to generate designs instantly from a single prompt.
  - **OHC Current State:** We currently have a 3-step conversational flow (`src/ui/next/src/app/onboarding/page.tsx`) mapping to an intake handler.
  - **Target:** Introduce an "Instant Build" mode parallel to or replacing the multi-step flow that accepts a single paragraph of text.

  ## Design Doc
  ### High-Level Architecture
  - **Conversational One-Pager:** A new UI state (`instant-build`) in the `SetupWizard` that presents a single textarea: "Describe your business in a sentence".
  - **Agent Handoff:** The input is sent directly to `api/onboarding/intake` or a new endpoint that invokes "The Advisor" to extrapolate all necessary metadata (business name, type, products, categories).
  - **Live Preview Generation:** "The Promoter" then immediately generates a live website draft based on the extracted metadata.

  ### Mobile UX Flow
  1. User lands on Onboarding Welcome screen.
  2. User taps "⚡ Instant Build (AI)".
  3. User enters: "I bake custom vegan cakes for weddings in Portland."
  4. Loading screen ("Designing your storefront...") while agents generate everything.
  5. User lands directly on the "You're Live!" confirmation screen.

  ## Implementation Prompt
  Implement an "Instant Build" mode in the `SetupWizard` (`src/ui/next/src/app/onboarding/page.tsx`). Add a toggle or distinct path for "Instant Build". When selected, present a single paragraph input. Send this to `/api/onboarding/intake`, parse the result, and auto-submit to `/api/onboarding/start` without requiring the user to manually review details (skip `step === 2` and `step === 3`). Update E2E tests to verify this rapid flow.

  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
