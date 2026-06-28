issue_title: "Implement Universal Zero-Click Agentic Mobile-First Onboarding Flow"
issue_description: |
  **Title:** Implement Universal Zero-Click Agentic Mobile-First Onboarding Flow

  **Problem Statement:**
  Non-technical business owners (like Maya the baker or Carlos the handyman) experience "setup paralysis" when confronting a blank canvas or complex configurations. Our market research (`agentic_autonomous_website_builders_smb_platform_gap_analysis.md`) shows that 73% of non-technical users abandon complex setups. Although OHC's current onboarding wizard in `onboarding_agent.rs` handles some automated setup, it relies on multiple steps, caching syncs, and manual navigation. A true "Zero-Click" generation flow is needed—a flow where a single sentence prompt ("I'm a baker in Austin") autonomously generates the DB schema, product catalog, and storefront layout on a native 375px mobile screen.

  **Research Report:**
  According to our UX analysis (`ux_analysis_onboarding.md`), the current onboarding uses a Next.js stepper that, while meeting the OHC Premium Token design standards structurally, is part of a legacy transition. Furthermore, our platform gap analysis reveals that traditional monoliths (like Shopify or Wix) impose an "App Tax" and overwhelming configurations. In contrast, emerging AI native builders generate sites quickly but lack our deep operational capabilities (e.g., booking, physical catalog).

  OHC must differentiate itself by providing an onboarding flow that is purely conversational, seamlessly integrated with our core agents (Operations, Marketing, Finance, Legal, Advisory), and mobile-first.

  **Design Doc:**

  **1. Architecture Diagram (Mermaid.js):**
  ```mermaid
  sequenceDiagram
      actor Owner
      participant MobileUI as OHC Mobile UI (375px)
      participant OnboardingAgent as Onboarding Agent
      participant Orchestrator as Orchestration Layer
      participant DB as PostgreSQL (Multi-tenant)

      Owner->>MobileUI: Submits single text prompt (e.g., "I'm a baker in Austin")
      MobileUI->>OnboardingAgent: POST /api/onboarding/start_zero_click
      OnboardingAgent->>Orchestrator: Parse persona, location, business type
      Orchestrator->>DB: Provision Tenant & Schema
      Orchestrator->>Orchestrator: Generate Initial Products/Services
      Orchestrator->>Orchestrator: Provision AI Departments
      Orchestrator-->>MobileUI: Return Success & Live Storefront URL
      MobileUI-->>Owner: Displays Success Screen & Auto-login
  ```

  **2. Mobile UX Flow:**
  - **Screen 1 (Input):** A 375px mobile view featuring a single vibrant, translucent glass card (macOS style). It contains one large text area and a prominent "Build My Business" button.
  - **Screen 2 (Loading):** A mesmerizing loading state showing the AI agents working. Text updates: "Analyzing business type...", "Generating product catalog...", "Setting up booking system...".
  - **Screen 3 (Success):** A clean success screen using OHC Premium Tokens (`#34C759` checkmark), presenting the live URL and a 1-Tap Launch button.

  **3. AI Agent Integration Points:**
  - The `OnboardingAgent` uses the `MinimaxClient` to extract structured `IntakeData` (business_name, location, target_audience, business_type, initial_products, etc.) from the unstructured single-sentence prompt.
  - The `Orchestration Layer` coordinates across `agent_clone_product` and `agent_clone_store` to populate the `TenantRegistry`.

  **Implementation Prompt:**
  **User-Facing Outcome:** A non-technical owner can open the app on their phone, type "I run a mobile dog grooming service in Seattle," and within 30-60 seconds, have a fully deployed storefront with initial booking services, an AI auto-responder enabled, and a multi-tenant DB schema provisioned.

  **CUJ (Critical User Journey):**
  1. User navigates to `/onboarding/zero-click`.
  2. User enters a brief description of their business.
  3. User taps "Generate".
  4. User is automatically redirected to the dashboard of their newly created, fully populated OHC workspace.

  **Acceptance Criteria:**
  - Implement a new `zero-click` API endpoint in `server/services/onboarding/` that bypasses the multi-step stepper.
  - Build the corresponding `375px` mobile-first UI using Tauri v2/Next.js adhering to `.glassmorphism` and OHC Premium Token standards.
  - 100% unit test coverage on the new API endpoints.
  - Playwright E2E test covering the exact CUJ described above.
  - No mock data: all provisioning must hit the real DB and Minimax/LLM provider (or test-mode adapters).

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
