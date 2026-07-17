issue_title: "[Research] AI-Native Storefront Generation for SMBs"
issue_description: |
  ## Issue Title: AI-Native Zero-Click Storefront Generation for SMBs

  ## Problem Statement
  Small business owners (SMBs) like Maya (Baker) and Carlos (Handyman) face significant friction when trying to establish an online presence. Existing tools like Shopify, Wix, and Squarespace require heavy manual configuration, theme selection, app integration, and content creation. The "blank canvas" problem causes high drop-off rates during onboarding. The current OHC offering lacks a rapid, AI-native way to instantly generate a fully functional, mobile-ready storefront with integrated services and commerce.

  ## Research Report
  - **Market Context**: Platforms like Durable, 10Web, and Hocoos are capitalizing on AI generation, but they often produce shallow landing pages rather than deep functional commerce tools. Shopify's "Sidekick" is mostly advisory, not executional.
  - **User Pain Points**: SMBs suffer from "App Tax" fatigue (paying for multiple disparate apps) and setup paralysis. They need a system that *does* the work based on natural language, rather than just telling them how to do it.
  - **The Opportunity**: OHC can differentiate by offering a "Zero-Click Generation" flow. A single prompt (e.g., "I'm a dog walker in Seattle") should autonomously generate the DB schema, product/service catalog, scheduling rules, and the actual storefront UI, ready for the user to review and launch from their phone.

  ## Design Doc
  - **Architecture Diagram (Mental Model)**:
    - User Input (Prompt) -> Prompt Processing Service -> AI Orchestrator
    - AI Orchestrator -> (Parallel Agents):
      1. Schema Generation Agent (Defines models)
      2. Content Agent (Writes copy, selects/generates images)
      3. UX/UI Agent (Selects templates, layouts based on OHC design system)
    - Output -> Unified JSON/Proto Representation of the Storefront State
    - Application Layer -> Renders the state directly to the user (Flutter/Web).
  - **Mobile UX Flow (375px)**:
    1. **Onboarding Screen**: A clean, single-input field (e.g., "Describe your business in one sentence").
    2. **Loading/Generation Screen**: Translucent glass overlay showing real-time agent activity ("Generating schema...", "Writing copy...").
    3. **Preview Screen**: A scrollable, fully functional preview of the generated storefront, utilizing OHC Premium Tokens (clean typography, unified cards).
    4. **Action Bar**: "Launch Now" or "Tweak" (which opens an AI chat for adjustments).
  - **AI Agent Integration**:
    - Rely on a strong structured prompt using `OHC_LLM_PROVIDER` (Gemini Pro/MiniMax).
    - The output must be strictly typed (JSON schema) to ensure deterministic rendering by the UI.
    - Integration with the existing `TenantRegistry` to instantly provision the new tenant's data structure based on the generated output.
  - **Key Decisions**:
    - Use AI to generate *data* and *configuration*, not raw HTML/CSS. The frontend renders OHC-native components based on this data.
    - Focus strictly on mobile-first interaction; the generation process and review must be flawless on a 375px viewport.

  ## Implementation Prompt
  **Goal**: Implement the backend service and API endpoint for the "Zero-Click Generation" feature, and the corresponding mobile-first UI for the onboarding flow.

  **CUJ (Critical User Journey)**:
  1. A new user lands on the OHC onboarding page on their mobile device.
  2. They enter a prompt: "I run a mobile car detailing service in Miami."
  3. They click "Generate".
  4. The system presents a loading screen showing agent progress.
  5. Within 30 seconds, a fully rendered preview of their storefront appears, including a generated catalog of services (e.g., Basic Wash, Premium Detail) and a booking component.
  6. The user clicks "Launch", and the tenant is provisioned with this real data.

  **Acceptance Criteria**:
  - A new gRPC/REST endpoint accepts the user prompt.
  - The backend uses the configured LLM provider to return a structured JSON response containing the generated business profile, services, and UI layout preferences.
  - A new Flutter/Web view is implemented that takes this JSON and renders the OHC native components.
  - The UI must adhere to the macOS Translucent Glass and UniFi layout design system, looking perfect on a 375px width screen.
  - E2E Playwright tests must be added to cover this specific generation flow, using the AI judge helper if necessary to validate the quality of the generated output.
  - The feature must be entirely usable without ever accessing a desktop browser.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
