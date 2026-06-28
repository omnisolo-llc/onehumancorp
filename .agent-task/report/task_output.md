issue_title: "Zero-Click Generation: Agentic Autonomous Onboarding & Store Creation Flow"
issue_description: |
  ## Problem Statement
  SMB owners (like Maya the Baker or Carlos the Handyman) suffer from "Setup Paralysis" when adopting new software. While traditional platforms like Shopify or Wix provide powerful tools, the initial configuration—from designing the storefront, setting up the database schemas, to writing initial copy—is terrifying and time-consuming for non-technical users. Our research indicates that 73% of non-technical users abandon complex setups. The current state requires users to manually piece together disparate tools (the "App Tax") and wait for advice rather than autonomous execution.

  ## Research Report
  Our competitive analysis in `agentic_autonomous_website_builders_smb_platform_gap_analysis.md` reveals:
  - **Traditional Builders:** Shopify, Wix, and Squarespace demand heavy manual setup and rely on reactive AI chatbots (like Shopify Sidekick) that only advise users on how to perform actions.
  - **AI Builders:** Platforms like Durable and 10Web offer rapid AI generation but lack the deep, native e-commerce and booking integration essential for businesses like Maya's or Leo's.
  - **The OHC Opportunity:** OHC can capture this market by offering a "Zero-Click Generation" flow where a single conversational prompt (e.g., "I'm a baker in Austin") autonomously generates a mobile-first storefront, configures the necessary database schemas (products, variants, inventory), and generates initial marketing copy.

  ## Design Doc
  ### Architecture
  The system will leverage a new `Onboarding Agent` that coordinates with the `Operations Agent` and `Marketing Agent`.

  ```mermaid
  sequenceDiagram
      participant User
      participant OnboardingAgent
      participant DB(PostgreSQL)
      participant MarketingAgent
      participant OperationsAgent

      User->>OnboardingAgent: Prompt: "I'm a baker in Austin"
      OnboardingAgent->>OperationsAgent: Request Schema & Catalog Structure
      OperationsAgent->>DB: Provision initial products & booking schema (Tenant Isolated)
      OperationsAgent-->>OnboardingAgent: Schema Ready
      OnboardingAgent->>MarketingAgent: Request Storefront Copy & Theme
      MarketingAgent-->>OnboardingAgent: Generated Copy & UI Tokens
      OnboardingAgent-->>User: Present fully functional mobile storefront for review
  ```

  ### Mobile UX Flow (375px)
  1. **Landing Screen:** A minimalist screen with a single, large input field: "Tell us about your business..."
  2. **Generation State:** While the AI agents work (target < 30 seconds), a skeleton loader or dynamic progress indicator explains the steps ("Creating your menu...", "Designing the storefront...").
  3. **Review Screen:** The user is presented with a fully functional, mobile-optimized storefront preview.
  4. **Action:** A sticky bottom button (≥ 44x44px touch target) labeled "Launch My Store" or "Edit Details".

  ### AI Agent Integration Points
  - **Onboarding Agent:** Acts as the orchestrator.
  - **Operations Agent:** Responsible for backend setup, including Stripe integration stubs, inventory tracking schemas, and service booking modules.
  - **Marketing Agent:** Uses LLM (Gemini Pro) to generate localized, persona-specific SEO copy and visual themes based on OHC Premium Tokens.

  ## Implementation Prompt
  **Feature Name:** OHC Zero-Click Autonomous Store Generation

  **Target Persona:** Maya the Home Baker

  **Outcome:** A single natural language prompt automatically provisions a functional, mobile-first storefront with a product catalog, booking integration, and initial marketing copy without requiring manual configuration from the user.

  **Next Actions for Engineering:**
  1. Implement the `OnboardingAgent` orchestration layer in the backend to receive the initial prompt.
  2. Develop the AI pipeline integrating Gemini to generate JSON structured output representing the initial DB state (catalog, services, copy).
  3. Create the backend services to securely provision this state into the PostgreSQL central ledger with strict multi-tenant isolation.
  4. Build the Flutter frontend flow: a single-input conversational UI, a generation loading state, and the final interactive storefront preview (optimized for 375px).
  5. Ensure E2E tests verify the flow from prompt input to successful DB population and storefront rendering.

  **Acceptance Criteria:**
  - Must function flawlessly on a 375px viewport (no horizontal scroll).
  - Must handle edge cases where the prompt is ambiguous.
  - Must include E2E Playwright tests verifying the generation flow.
  - Zero mock data; the generated state must be persisted in the real backend.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []