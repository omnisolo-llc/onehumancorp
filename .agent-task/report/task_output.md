issue_title: "Implement Zero-Click Onboarding Agent for Rapid Storefront Generation"
issue_description: |
  ## Problem Statement
  Small business owners (like Maya the Baker) abandon complex setup flows on traditional platforms (like Shopify or Wix) due to the overwhelming number of menus, DNS configurations, and initial data entry. Our competitive analysis shows 34% drop-off due to technical complexity. OHC must provide a "Zero-Click" onboarding experience where a single natural language prompt ("I am a baker in Austin selling custom cakes") automatically generates the database schema, product catalog, and a mobile-first storefront in under 10 minutes.

  ## Research Report
  - **Competitor Insights**: AI-native builders like Durable generate sites in 30 seconds but lack deep commerce integration. Shopify Sidekick advises but does not execute the initial store creation autonomously.
  - **OHC Gap**: We currently rely on manual Next.js/Tauri wizard flows (as seen in `src/ui/next/src/app/onboarding/page.tsx`). We need an agentic approach that bypasses forms.
  - **Persona Focus**: Maya (Baker, 28) needs a beautiful storefront and deposit-based ordering without reading a manual.

  ## Design Doc
  - **Architecture Diagram (Mermaid.js)**:
    ```mermaid
    sequenceDiagram
      participant Owner as Maya (Mobile App)
      participant Gateway as OHC API Layer
      participant Agent as Onboarding Agent
      participant LLM as Gemini Provider
      participant DB as Postgres (Tenant Schema)

      Owner->>Gateway: "I'm a baker in Austin..."
      Gateway->>Agent: Route prompt
      Agent->>LLM: Generate business model (categories, products, prices)
      LLM-->>Agent: JSON Blueprint
      Agent->>DB: Provision Tenant, insert products
      Agent->>Gateway: Return generated storefront URL
      Gateway-->>Owner: Display mobile-first preview
    ```
  - **Mobile UX Flow**:
    1. 375px view with a single chat interface.
    2. User inputs their business concept.
    3. Loading state showing agent progress ("Provisioning database...", "Drafting product copy...").
    4. Transition to a live preview of the generated storefront (using Premium Token glassmorphism).
  - **AI Agent Integration Points**: Create a new `ZeroClickOnboarding` capability in the Rust backend that utilizes `minimax.reason()` or Gemini to output a structured JSON blueprint (tenant details, products, initial policies) and directly persists it to PostgreSQL using `save_onboarding_state`.

  ## Implementation Prompt
  **Goal**: Build the "Zero-Click Onboarding" flow.
  **User Journey**: The owner downloads the app, types one sentence describing their business, and within 30 seconds is presented with a fully populated storefront (with sample products and generated copy) that they can immediately publish.
  **Acceptance Criteria**:
  - Introduce a new chat-based onboarding screen in Tauri (mobile layout).
  - Implement the backend agent capable of translating a free-text prompt into a complete tenant database state (products, services).
  - The UI must use the new translucent glass design tokens.
  - Fully covered by Playwright E2E tests validating the end-to-end generation flow.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
