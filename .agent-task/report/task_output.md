issue_title: "Implement 'Zero-Click Generation' Mobile-First Onboarding Flow"
issue_description: |
  **Problem Statement**
  Small business owners (like Maya the Baker or Carlos the Handyman) experience setup paralysis when facing the blank canvas of traditional commerce platforms like Shopify or Wix. They abandon complex setups 73% of the time. They need an AI that acts and executes, rather than just advising.

  **Research Report**
  Traditional giants (Shopify, Wix) require a desktop-first approach with complex plugins. Emerging tools offer AI generation but remain superficial. The OHC Mobile-First Operations paradigm mandates that complex actions—from initial setup to daily execution—can be done on a 375px mobile screen. OHC must deploy a "Zero-Click Generation" flow where a single prompt (e.g., "I'm a baker in Austin") autonomously generates the DB schema, product catalog, and storefront layout.

  **Design Doc**
  - Architecture: A new `ZeroClickOnboardingService` that interfaces with the LLM via Minimax to parse the prompt, generate structured schema and content, and persist this to the DB via `AgentFeedService` action payloads or direct CRUD.
  - Mobile UX Flow: Start with a single text input area taking up the top half of the 375px screen. Submit button must be full-width (44px min touch target). While loading, show translucent glass loading states. Once complete, show a success card with a large 'Approve & Launch' button.
  - AI Agent Integration: The onboarding flow triggers a specialized 'Onboarding Agent' which acts as a departmental worker to populate the initial catalog and settings.

  **Implementation Prompt**
  Implement the user-facing "Zero-Click Generation" onboarding flow for mobile.
  - CUJ: A non-technical user opens the app, types "I'm a baker in Austin", and taps generate. The system creates a functional business setup (menu items, basic settings) in the backend.
  - Acceptance Criteria: The screen layout must work perfectly on a 375px viewport with no horizontal scrolling. Touch targets must be >=44x44px. Must use Translucent Glass components. Include tests covering the LLM prompt submission and successful processing of the payload.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
