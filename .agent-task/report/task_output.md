issue_title: "[Architecture] Zero-Click AI Storefront Generation & Mobile-First Onboarding"
issue_description: |
  # Research Report: Zero-Click AI Storefront Generation & Mobile-First Onboarding

  ## Track 1: Architectural Gap & Scaling Discovery

  **Observation & Audit:**
  During our market mapping and codebase audit (reviewing competitor AI assistants like Shopify Sidekick and Wix Studio AI vs AI-native upstarts like Durable), a significant gap emerged. Shopify requires 30-60 minutes to onboard and configure themes/plugins. AI-native tools like Durable can generate a site in under a minute but lack deep operations/backend logic. OHC currently relies a manual setup process to create organizations, add users, and set up products.

  **The Core Gap:** Non-technical operators (like Maya the baker or Carlos the handyman) suffer from "setup paralysis". They abandon platforms that show them blank canvases or complex configuration dashboards. OHC is missing an automated, agent-driven "Zero-Click Generation" onboarding flow that translates a single natural language input (e.g., "I make custom vegan cakes in Austin") into a fully configured tenant schema, product catalog, localized storefront, and initial booking/deposit configuration.

  ## Track 2: Selected Architecture Deep Dive

  **Business Journey Mapping:**
  - **Persona:** Maya (Baker, 28) wanting to move away from messy Instagram DMs.
  - **Journey:** Maya opens the OHC mobile app. The first screen asks a simple question: "What do you do?" She types or uses voice: "I sell custom vegan cakes in Austin and need a $50 deposit for orders."
  - **The Magic:** The OHC Autodream Agent takes this prompt and triggers the `Storefront Generation Pipeline`. In < 30 seconds:
    - Creates a Multi-tenant `Tenant` context.
    - Generates a lightweight Mobile-First Storefront UI (Premium Translucent Glass).
    - Configures a `Product Catalog` with AI-generated cake placeholders and variants.
    - Configures a Stripe connect or basic payment profile with a $50 deposit requirement.
    - Presents the result: "Here is your store. We generated 3 cake styles. You can replace the photos. Should we turn on bookings?"

  **Data Model & Invariants:**
  - **Entities:** `Tenant`, `AgentFeed`, `StorefrontConfig`, `Product`, `BookingRule`.
  - **Multi-tenant Isolation:** Strict RLS enforcement on all generated entities using `tenant_id`. All generation requests run within a temporary sandbox tenant until the owner confirms ownership and creates credentials.
  - **Agent Department Coordination:**
    - *Onboarding Agent:* Captures the prompt and orchestrates.
    - *Design Agent:* Chooses layout components, color tokens, and typographies based on the business type.
    - *Operations Agent:* Sets up booking and payment configuration rules.

  ```mermaid
  erDiagram
      TENANT ||--o{ STOREFRONT_CONFIG : "owns"
      TENANT ||--o{ PRODUCT : "has"
      TENANT ||--o{ BOOKING_RULE : "enforces"
      STOREFRONT_CONFIG {
          uuid id
          uuid tenant_id
          jsonb theme_tokens
          jsonb layout_structure
      }
      PRODUCT {
          uuid id
          string name
          boolean requires_deposit
          decimal deposit_amount
      }
  ```

  ## Track 3: Technical Integrity & Mobile-First Review

  - **Mobile-First UX Flow:**
    - Single, prominent input field on a 375px mobile screen. Large, friendly typography.
    - Translucent glass loading state displaying what the agents are currently building ("Designing storefront...", "Configuring payments...").
    - A stack of generated Unifi-style modular cards that Maya can tap to accept or edit. No desktop-only complex nav menus.
  - **Performance & Offline Targets:**
    - Target payload generation < 15 seconds.
    - Generated site must be statically exportable or edge-cached for instant loading.
    - Offline fallback: the setup intent is queued locally and submitted when the network restores.
  - **Zero Trust & Security:**
    - SPIFFE/SPIRE identity utilized by the onboarding agent when performing tenant creation. The user receives a magic link to claim the newly created `tenant_id`.

  ## Track 4: Strategic Feature Issue Dispatch

  **Implementation Prompt for Implementer Agent:**
  **User-Facing Outcome:** Build the "Autodream" zero-click generation mobile interface and backend pipeline. The user provides a single natural language description of their business, and the backend orchestrates the AI to create their initial tenant, storefront layout, and product catalog.
  **CUJ (Critical User Journey):**
  1. Open app -> See "Describe your business" prompt.
  2. Enter text -> See real-time generation steps.
  3. View the generated storefront, products, and deposit rules as modular cards.
  4. Tap "Launch" to finalize the tenant creation.

  **Acceptance Criteria:**
  - Must include a Flutter/Mobile-friendly setup screen (375px width optimized).
  - Must use AI (via configured LLM) to return a structured JSON response defining the products and storefront configuration.
  - Must enforce `tenant_id` RLS when persisting the generated data.
  - Must include full Playwright E2E test verifying the flow from the initial prompt to the final rendered storefront.

  **Priority:** P0 (Critical for Activation)
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
