issue_title: "Implement Autonomous Zero-Click Mobile-First Storefront Generation Flow"
issue_description: |
  # Mission Queue Protocol: Autonomous Zero-Click Mobile-First Storefront Generation

  ## Problem Statement
  Non-technical small business owners (like Maya the baker and Carlos the handyman) face significant friction ("Setup Paralysis") during onboarding. Current flows require manual configuration of settings, products, and domains, taking upwards of 60 minutes. 34% of these users abandon setup due to technical complexity. We need an autonomous, mobile-first experience that takes a user from a single natural language prompt to a fully generated, functional storefront and booking page in under 60 seconds.

  ## Research Report
  Our competitive research (`agentic_autonomous_website_builders_smb_platform_gap_analysis.md`, `ohc_owner_work_assistant_competitive_research.md`) maps OHC against tools like Shopify (Sidekick) and Durable.co:
  - **Shopify/Wix:** High power, but complex, manual desktop-first setups.
  - **Durable:** Generates sites in 30 seconds but lacks deep operations and customizability.
  - **OHC Gap:** We need to combine Durable's "Zero-to-One" speed with OHC's "Agentic Operations." A user should type "I'm a baker in Austin" and the Onboarding/Marketing Agent should instantly provision a mobile-first UI, configure Stripe, and seed a multi-tenant isolated database schema.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as Mobile User (Maya)
      participant UI as OHC Frontend (Flutter/Next)
      participant KAIROS as KAIROS Orchestrator
      participant Agent as Onboarding/Marketing Agent
      participant DB as PostgreSQL (Multi-Tenant RLS)

      Owner->>UI: "I'm a custom baker in Austin"
      UI->>KAIROS: Submit natural language prompt
      KAIROS->>Agent: Trigger Zero-Click Generation
      Agent->>DB: Provision Tenant & Schema (RLS)
      Agent->>DB: Seed initial catalog/services
      Agent-->>KAIROS: Generation Complete
      KAIROS-->>UI: Return Storefront URL
      UI-->>Owner: Display Mobile-First Storefront
  ```

  ### UI Flow (375px Mobile First)
  1. **Splash/Login:** Clean, fast entry point.
  2. **The Prompt Screen:** A single text input (with native mobile keyboard) and microphone button: "What kind of business are you running?"
  3. **Generation State:** A premium Translucent Glass skeleton loader (Ubiquiti UniFi style) showing real-time agent progress ("Designing layout...", "Adding products...", "Setting up bookings...").
  4. **The "Aha!" Moment:** The user is dropped directly into the live, 375px-optimized storefront.

  ### Key Design Decisions
  - **Zero Trust/Multi-Tenant:** The Agent must use SPIFFE/SPIRE identity to securely provision data under a strict `tenant_id` context with PostgreSQL RLS.
  - **Optimistic UI:** The frontend must feel responsive immediately, relying on KAIROS for background syncing.

  ## Implementation Prompt
  Implementer Agent: Your task is to build the "Autonomous Zero-Click Mobile-First Storefront Generation" flow. The CUJ begins at the home page after login. Present a single chat/prompt interface. When the owner submits their business description (e.g., "I'm a handyman"), trigger an agentic workflow that creates their tenant context, provisions sample services/products, and immediately transitions them to a generated mobile storefront.
  - **Acceptance Criteria:**
    - The flow must be fully functional on a 375px viewport.
    - Zero mock data in the final UI; the generated storefront must reflect real data persisted by the agent.
    - Implement a Playwright E2E test covering the prompt submission to the final generated storefront view.
    - Ensure strict multi-tenant isolation is maintained during generation.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
