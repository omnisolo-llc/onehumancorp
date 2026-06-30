issue_title: "Implement Zero-Click Agentic Onboarding Protocol"
issue_description: |
  # Research Report: Zero-Click Agentic Onboarding Protocol

  ## 1. Problem Statement
  Small business owners face significant setup paralysis when adopting new platforms. Traditional SaaS platforms (like Shopify or Wix) present users with an empty dashboard and demand extensive manual configuration (menus, settings, integrations, schemas). Maya (the Home Baker) and Carlos (the Handyman) do not have the time or technical confidence to configure multi-tenant databases, Stripe payment keys, or DNS settings. The onboarding process must transition from a "build it yourself" model to an "assistant builds it for you" model.

  ## 2. Research Report
  - **Market Context:** Competitive analysis reveals that AI-native tools like Durable.co generate websites in under 30 seconds from a single prompt. However, these tools often lack deep operational logic (inventory, bookings, multi-channel POS) and are treated as "AI Toys."
  - **The OHC Opportunity:** By leveraging the existing KAIROS orchestration engine and our strict multi-tenant PostgreSQL schema, OHC can combine the rapid, prompt-based generation of a Durable with the robust backend operations of a Shopify.
  - **Competitor Gaps:**
    - *Shopify:* Setup takes hours/days. "Sidekick" merely advises the user on how to navigate menus rather than taking autonomous action.
    - *Durable:* Excellent onboarding, but shallow operational capabilities.
    - *Squarespace:* Guided blueprint onboarding but still requires manual design assembly.

  ## 3. Design Doc (Architecture & UX)

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD;
      User[Owner (Mobile/Web)] -->|Natural Language Prompt| Triage[Onboarding Interceptor];
      Triage --> Orchestrator[KAIROS Orchestrator];
      Orchestrator --> OpsAgent[Operations Agent];
      Orchestrator --> SalesAgent[Sales Agent];
      Orchestrator --> DesignAgent[Design Agent];
      OpsAgent --> DB[PostgreSQL: Provision Services & Inventory];
      SalesAgent --> Stripe[Stripe: Setup Payments & Deposit Limits];
      DesignAgent --> UI[Flutter: Generate View Configs];
      Orchestrator --> User[Deliver fully configured Work Assistant Feed];
  ```

  ### Data Model & Multi-Tenancy Invariants
  - **Tenant Provisioning:** The onboarding prompt triggers a background job to allocate a new `tenant_id` and apply Row-Level Security (RLS) constraints.
  - **Agent State Graph:** A new `OnboardingSession` entity tracks the conversational state, mapped to the user and provisional tenant ID, ensuring that dropped connections can resume exactly where they left off.

  ### Mobile UX Flow (375px)
  1. **Landing (The "One Question" UI):** User opens the OHC app. Instead of a complex signup form, a clean, translucent glass screen asks: "What kind of work do you do?" with a native mobile keyboard pre-focused.
  2. **Conversational Refinement:** The AI Agent responds, e.g., "Great, a custom cake bakery! Do you take deposits for orders?" (Yes/No buttons).
  3. **Generative Loading State:** A visually premium pulse animation indicating: "Creating your product catalog... Setting up your booking calendar... Connecting payment structures."
  4. **The Assistant Feed:** The user is dropped directly into their active daily feed with 3 sample (AI-generated) inbound requests that demonstrate how to use the app.

  ### AI Agent Integration Points
  - **Onboarding Coordinator Agent:** Interprets the initial prompt and delegates schema creation tasks to departmental agents.
  - **Operations Agent (Setup Mode):** Translates the business type into a default inventory list or booking schedule (e.g., standard business hours).
  - **Sales Agent (Setup Mode):** Configures default pricing heurisitcs and deposit policies.

  ## 4. Implementation Prompt
  **Feature Name:** OHC Zero-Click Agentic Onboarding Flow
  **Target Persona:** Maya the Home Baker
  **Outcome:** Maya types "I bake custom wedding cakes from my home in Austin" and within 3 minutes, the system autonomously provisions her tenant, creates a basic cake catalog, sets up a deposit-based quoting flow, and drops her into the Assistant Feed.

  **Acceptance Criteria / Next Actions:**
  1. **Schema & State:** Implement an `OnboardingSession` table with strict tenant isolation to manage the multi-step generative flow.
  2. **API Layer:** Create a gRPC/REST endpoint `InitiateAgenticOnboarding` that accepts a natural language string and streams back status updates via server-sent events or WebSockets.
  3. **Agent Orchestration:** Extend the `KAIROS` engine to parse the onboarding prompt and execute parallel background tasks (catalog creation, setting default business hours).
  4. **Mobile UI:** Build the Flutter/Next.js 375px conversational entry screen, utilizing OHC Premium translucent materials and typography. Ensure zero placeholder mock data is used; the UI must render the real generated tenant data.

  ## 5. Priority & Scope
  **Priority:** P0 (Critical for Growth & Activation)
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
