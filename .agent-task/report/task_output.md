issue_title: "Zero-Click AI Storefront & Operations Generation Protocol"
issue_description: |
  # Mission Queue Protocol: Zero-Click AI Storefront Generation

  ## Problem Statement
  Small business owners (e.g., Carlos the Handyman, Fatima the Food Cart Operator) experience massive drop-off during platform onboarding because they are overwhelmed by a blank canvas. Traditional platforms like Shopify or Wix require hours of manual configuration, theme selection, and schema setup before any real value is realized. The SMB user needs a solution that transitions from an idea (e.g., "I sell custom cakes") to a fully configured, operationally ready platform with zero clicks.

  ## Research Report
  - **Shopify & Wix**: They offer AI chatbots (like Shopify Sidekick) or AI questionnaire-based setup, but the user is still required to manually refine settings, connect apps, and orchestrate their operations.
  - **AI-Native Builders (Durable, Mixo)**: They generate simple landing pages quickly but lack deep operational backends (no real inventory, booking, or POS sync).
  - **OHC Opportunity**: OHC can differentiate by not just generating a landing page, but autonomously configuring the backend PostgreSQL schema, Agent workflows (Sales, Operations), payment intent endpoints, and storefront UI in a single, prompt-driven action.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner (Mobile)
      participant OHC API
      participant Prompt Orchestrator
      participant DB Generator Agent
      participant UI Config Agent

      Owner (Mobile)->>OHC API: Submit prompt ("I run a mobile dog grooming service in Austin")
      OHC API->>Prompt Orchestrator: Init Zero-Click Flow
      Prompt Orchestrator->>DB Generator Agent: Generate Service DB, Availabilities, & Prices
      Prompt Orchestrator->>UI Config Agent: Generate 375px native UI layout & theme
      DB Generator Agent-->>OHC API: Configured Postgres schema & sample data
      UI Config Agent-->>OHC API: Storefront configuration data
      OHC API-->>Owner (Mobile): Render live, operationally ready storefront
  ```

  ### Mobile UX Flow (375px first)
  1. **Onboarding Screen**: A clean, single-input prompt window: "Describe your business in a few words."
  2. **Generation State**: A translucent glass loading overlay displaying agent activity ("Configuring services...", "Setting up booking calendar...").
  3. **Live Storefront View**: The fully generated app shell opens in "Owner Mode", showcasing a pre-filled booking calendar, a drafted welcome email, and a tap-to-pay active terminal ready for the first customer.

  ### AI Agent Integration
  - **DB Generator Agent (Gemini/Claude)**: Parses the user's business intent to automatically populate `Services`, `Pricing`, and `Availability` tables.
  - **UI Config Agent**: Applies the OHC Premium Token library (Apple/Ubiquiti-style hierarchy) based on the business domain (e.g., playful for dog grooming, sleek for a boutique).

  ### Zero Trust & Multi-Tenancy
  - The entire generation process occurs within a strictly isolated PostgreSQL tenant context (`tenant_id`).
  - Agent requests are authenticated via SPIFFE/SPIRE to prevent cross-tenant data leakage during the initial setup phase.

  ## Implementation Prompt
  **Feature Name**: Zero-Click AI Storefront Generation Protocol
  **Target Persona**: Carlos the Handyman (no technical skills, wants to book clients immediately).

  **Outcome**: Implement an onboarding API and mobile-first UI flow where a single natural language prompt results in a fully seeded, tenant-isolated operational backend (PostgreSQL) and a functional storefront UI, bypassing all manual configuration screens.

  **Critical User Journey (CUJ) & Acceptance Criteria**:
  1. Provide a single text input in the mobile UI (375px) for the user's business description.
  2. The backend must orchestrate an LLM call to structure the business domain into a JSON schema (Services, Pricing, Policies).
  3. The `DB Generator` must parse this schema and perform authenticated database insertions within the new tenant's isolation scope.
  4. The UI must instantly render the generated storefront and owner dashboard without any page reloads or manual theme tweaking.
  5. Playwright E2E tests must verify that a submitted prompt successfully creates the tenant database records and renders the correct UI elements.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
