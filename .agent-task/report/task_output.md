issue_title: "Architectural Gap: Zero-Click Onboarding Agent for Setup Paralysis"
issue_description: |
  # OHC Architectural Gap: Zero-Click Onboarding Agent

  ## Problem Statement
  Currently, OneHumanCorp (OHC) has a powerful multi-tenant backend (KAIROS orchestration, specialized services). However, it lacks a critical capability identified in competitive research (Shopify Sidekick, Durable AI): **Autonomous, AI-driven zero-to-one onboarding**.

  Our target persona, **Maya (the home baker)**, faces setup paralysis. 34% of small business owners abandon platforms due to "technical complexity" (e.g. configuring DNS, setting up Stripe deposits, mapping products). Maya wants to sell cakes immediately from an Instagram-like interaction, not read technical manuals. While our competitors (like Durable) can generate a site in < 1 minute, OHC currently requires a manual, service-first setup taking ~1 hour.

  To solve this, we must build the **"Zero-Click Onboarding Agent"** architecture.

  ## Research Report
  - **Shopify & Apps**: The "Shopify Tax" forces merchants to install 5-10 apps to get going. Shopify's Sidekick tries to mitigate this, but core setup (like shipping zones) remains manual.
  - **Durable AI**: Excels at a "30-Second Setup" by automatically generating a site, CRM, and invoicing.
  - **OHC Current State**: OHC is assistant-first, but the initial barrier to entry is too high.
  - **The Gap**: We lack an autonomous `Setup Agent` capable of interacting with the new user in natural language to instantly provision tenant resources, populate initial catalog/services from images or brief descriptions, and activate a localized dynamic storefront.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      User[User (Maya)] -->|Natural Language Prompt / Image| Shell[Flutter Assistant Shell]
      Shell -->|gRPC/REST| Gateway[API API Layer]
      Gateway --> SetupAgent[Setup Agent Department]
      SetupAgent --> KAIROS[KAIROS Orchestrator]
      KAIROS -->|Provisioning| TenantDB[(Tenant DB Isolation)]
      KAIROS -->|Publishing| Storefront[Edge-Cached Storefront]
      KAIROS -->|Config| Integrations[Stripe / Stripe Connect]
      SetupAgent -->|Feedback| Shell
  ```

  ### Mobile UX Flow (375px First)
  1. **Greeting**: The app opens to a clean, translucent-glass chat UI. "What kind of business are you starting today?"
  2. **Intake**: Maya uploads a picture of a cake and types "Custom vegan cakes in Portland."
  3. **Agent Action**: The `Setup Agent` displays a beautiful, modular card (UniFi style) showing real-time progress: "Provisioning workspace", "Drafting product catalog", "Setting up deposit payments".
  4. **Review**: The agent presents a finalized 375px mobile storefront preview and a suggested deposit policy.
  5. **Activation**: Maya clicks one "Approve & Launch" button. The agent finalizes the DB schema, connects the Stripe sandbox, and the business is live.

  ### AI Agent Integration Points
  - **Trigger**: New unauthenticated or unconfigured tenant session.
  - **Memory**: The Setup Agent holds a temporary sandbox context.
  - **Action**: Once approved, it uses KAIROS to execute mutations across `catalog`, `pricing`, `inventory`, and `settings` services.

  ### Key Design Decisions
  - **Zero-Trust**: The Setup Agent operates in a restricted temporary scope until the user officially "Approves", at which point the tenant boundary is hardened.
  - **Visuals**: Progress states must use Apple/Ubiquiti translucent glass tokens to feel premium, never exposing terminal text or raw JSON.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Build the end-to-end "Zero-Click Onboarding Agent" flow. The CUJ begins with a new user on a mobile device (375px) interacting with a chat UI. The user uploads a product image or provides a short text description. Your implementation must capture this, route it to a new backend `Setup Agent` that interacts with the LLM to extract business details, and then autonomously populate the tenant's database with at least one product, a basic pricing/deposit policy, and a functional storefront link. The final screen must present a single "Launch" button that finalizes the setup. Ensure all new components use the premium translucent glass design tokens and strictly adhere to the row-level security multi-tenant architecture. Add at least 5 Playwright E2E tests validating this exact onboarding journey.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []