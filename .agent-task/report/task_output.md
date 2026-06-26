issue_title: "Implement Zero-Click Agentic Onboarding Engine"
issue_description: |
  # Zero-Click Agentic Onboarding Engine

  ## Problem Statement
  Small business owners and non-technical operators (like Maya the Baker or Carlos the Handyman) experience extreme "Setup Paralysis" when adopting new platforms. Traditional e-commerce tools (e.g., Shopify) require days of manual configuration for shipping zones, tax settings, domain routing, and inventory structures before they can capture their first sale. AI-native rivals like Durable have proven the market appetite for sub-60-second setup. OHC needs a Zero-Click Onboarding flow that uses autonomous agents to provision business infrastructure directly from an initial conversation or social profile link, entirely bypassing traditional complex forms and settings dashboards. The owner needs to go from "unclear work" to "ready to take deposits" in under 10 minutes from their mobile phone.

  ## Research Report
  - **Market Landscape**: Tools like Durable and 10Web are capturing the micro-SME market by generating business websites and CRM setups from natural language prompts in under a minute.
  - **Competitor Gap**: Shopify's "Sidekick" is a reactive assistant inside a complex admin panel. It helps, but it doesn't solve the core Day 1 structural onboarding friction.
  - **OHC Opportunity**: OHC's unique value is its integration of Operations, Finance, and CS agents. A true Zero-Click onboarding shouldn't just generate a static website; it should configure the required backend services (booking, quoting, POS) based on the business type inferred by the AI.
  - **User Sentiment**: Research indicates 34% of small business owners abandon e-commerce setup due to technical complexity (Reddit/SmallBiz). E-commerce platform drop-off in the first hour is a critical failure point.

  ## Design Doc
  ### Mobile UX Flow (375px First)
  1. **Landing/Greeting Screen**: "Hi, I'm your OHC Assistant. What kind of business are you starting, or paste a link to your Instagram/Facebook?"
  2. **Agentic Processing (Vibe Coding/Spinner)**: A dynamic card showing background tasks: "Analyzing Instagram profile...", "Creating custom cake product catalog...", "Setting up Stripe deposit links..."
  3. **The Reveal (Preview Screen)**: "Here is your new storefront and booking system. Want to change anything before we publish?"
  4. **Approval**: User taps "Looks Good! Publish".
  5. **Post-Onboarding Hub**: User lands directly on the Assistant-First Shell feed, not a settings dashboard.

  ### AI Agent Integration Points
  - **Intake Agent**: Uses Gemini Pro to parse natural language or scrape social profile data to determine business category (e.g., 'Physical Products' vs 'Services & Bookings').
  - **Provisioning Agent**: Calls internal APIs to establish PostgreSQL tenant schemas, set up default catalog items, and prepare Stripe integration configurations.
  - **Copy/Design Agent**: Generates tailored copy, selects design tokens, and builds the initial PWA storefront schema.

  ### System Architecture (Mermaid)
  ```mermaid
  sequenceDiagram
      actor User
      participant MobileUI as OHC Mobile UI
      participant OnboardOrchestrator as Onboarding Orchestrator (KAIROS)
      participant LLMAgent as LLM Intake/Copy Agent
      participant Provisioning as Core Provisioning Service

      User->>MobileUI: "I sell custom cakes on IG (@mayascakes)"
      MobileUI->>OnboardOrchestrator: Submit Intake Request
      OnboardOrchestrator->>LLMAgent: Analyze request/profile, extract business entity
      LLMAgent-->>OnboardOrchestrator: Return Structured Business Profile (Type: Food/Bev, Needs: Deposits, Catalog)
      OnboardOrchestrator->>Provisioning: Create Tenant, Seed Catalog, Configure Stripe intent
      Provisioning-->>OnboardOrchestrator: Tenant Provisioned & Configured
      OnboardOrchestrator->>LLMAgent: Generate Storefront UI config
      LLMAgent-->>OnboardOrchestrator: Return UI Schema
      OnboardOrchestrator-->>MobileUI: Push "Ready to Publish" state
      User->>MobileUI: Taps "Publish"
      MobileUI->>Provisioning: Finalize & Go Live
  ```

  ### Key Design Decisions
  - **Zero Trust/Tenant Isolation**: The Provisioning Service must guarantee row-level security (`tenant_id`) setup before any LLM-generated data is persisted.
  - **Mobile-First Constraints**: All configuration interactions must fit on a 375px screen without requiring the user to tap into complex nested menus. Use conversational UI for configuration gathering.
  - **Asynchronous Architecture**: The onboarding orchestration should leverage the AI Job Queue (PostgreSQL SKIP LOCKED) to handle potentially long-running LLM calls without blocking the UI, utilizing WebSockets or polling for status updates to the mobile client.

  ## Implementation Prompt
  Implement the "Zero-Click Agentic Onboarding Engine".
  - **Outcome**: A user can arrive at the OHC app, describe their business in one sentence (or provide a social handle), and the system will automatically configure a tenant, generate a basic product/service catalog, and present a functional mobile storefront preview.
  - **CUJ**:
    1. User lands on onboarding screen and types "I am Carlos, I run a handyman service in Austin."
    2. System processes request, identifies business type as "Services & Bookings".
    3. System automatically creates a tenant, generates default service items (e.g., "General Repair Quote", "Plumbing Assessment"), and sets up a booking calendar layout.
    4. User sees the generated storefront preview on their phone and clicks "Publish".
  - **Acceptance Criteria**:
    - The onboarding flow is fully functional and responsive on a 375px viewport.
    - E2E Playwright tests must verify the entire flow from initial prompt to the successfully provisioned and displayed storefront preview, without mocking the internal provisioning APIs.
    - Backend must correctly assign `tenant_id` and maintain strict data isolation for the generated assets.

  ## Priority
  P0

  ## Estimated Scope
  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
