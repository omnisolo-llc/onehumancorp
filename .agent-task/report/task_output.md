issue_title: "Zero-Click Onboarding Agent: Autonomous Business Setup via Conversational AI"
issue_description: |
  ## Problem Statement
  Small business owners face "Setup Paralysis" when adopting new software. 34% of small business owners abandon setup due to technical complexity (e.g., DNS configuration, payment gateway setup, catalog creation). Persona Maya (Home Baker) wants to sell cakes, not configure Stripe or build web pages manually.

  ## Research Report
  Our competitive analysis (Durable.co, Shopify Sidekick, Wix Studio AI) shows that AI-native platforms are winning early-stage adoption by offering <1 minute setup experiences. Durable generates a site, CRM, and invoicing module from a single prompt. Shopify Sidekick requires an existing store to be useful. OHC currently takes an estimated 1 hour of manual setup. To win the SMB market, OHC must close the gap between intent and first transaction by using an agentic onboarding flow that operates invisibly behind a chat interface.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD;
      Owner[Owner Mobile 375px] -->|Chat Prompt| ChatUI[Onboarding Chat UI];
      ChatUI --> TriageAgent[Triage Agent];
      TriageAgent -->|Entity Extraction| BizProfile[Business Profile Creation];
      TriageAgent -->|Tool Call| StripeAgent[Stripe Setup Agent];
      TriageAgent -->|Tool Call| DomainAgent[DNS/Domain Agent];
      TriageAgent -->|Tool Call| CatalogAgent[Catalog & Pricing Agent];
      BizProfile --> DB[(Tenant DB)];
      StripeAgent --> DB;
      DomainAgent --> DB;
      CatalogAgent --> DB;
      DB --> Dashboard[Owner Dashboard Generated];
  ```

  ### Mobile UX Flow (375px First)
  1. **Landing:** Single text input area: "Describe your business in one sentence." (e.g., "I sell custom vegan cakes in Austin").
  2. **Generation State:** Translucent glass spinner with Apple-style fluid animations. "Building your catalog...", "Configuring payments...", "Designing storefront...".
  3. **Interactive Review:** Agent presents a swipeable carousel of the generated site, products, and a one-click "Connect Bank" (Stripe) button.
  4. **Refinement:** Owner can type "Make the theme more pastel" and the agent updates the UI instantly.

  ### AI Agent Integration Points
  - **Triage Agent (Gemini Pro / MiniMax):** Understands the business type and orchestrates specialized agents.
  - **Stripe Setup Agent:** Uses Stripe API to create Express Connected Accounts and Payment Links.
  - **Catalog Agent:** Uses image generation (or placeholders) and LLM to draft product descriptions and reasonable default pricing.

  ### Key Design Decisions
  - **No Manual Forms:** Standard multi-step forms are completely replaced by conversation and tool-calling agents.
  - **Progressive Enhancement:** The owner can start taking orders via payment links before a full website is even published.
  - **Multi-Tenant Isolation:** All generated assets are strictly bound to the new `tenant_id` created during the first message, secured by Row Level Security (RLS).

  ## Implementation Prompt
  **To the Implementer:**
  Create the "Zero-Click Onboarding Agent" feature in the Flutter mobile-first UI and Go backend.
  - **CUJ:** A new user opens the app, types a sentence about their business, and the system automatically provisions a tenant, drafts 3 sample products, and prepares a payment link.
  - **Acceptance Criteria:**
    1. The UI must be fully functional on a 375px width screen without horizontal scrolling.
    2. The frontend must use the existing AI chat component to collect the business description.
    3. The backend must use a KAIROS agent with tool calling to create the `Tenant`, `Product`s, and `StripeAccount` placeholder.
    4. Provide at least one full E2E test proving the chat flow successfully creates the tenant and products in the database.
    5. UI must follow the premium Translucent Glass design tokens.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
