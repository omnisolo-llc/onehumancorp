issue_title: "Implement Agentic Zero-Click Onboarding Flow"
issue_description: |
  ## Problem Statement
  Small business owners face "Setup Paralysis" when adopting new platforms. Research indicates that non-technical users abandon complex setups in high numbers because the initial blank canvas is terrifying, and traditional platforms (Shopify, Wix) require them to manually configure settings, catalogs, and design. Maya (Home Baker) wants to sell cakes, not configure DNS or read manuals. OHC currently requires a manual setup process that takes an hour, lagging behind the "Zero-to-One" AI-native competitors (like Durable) that can generate sites in 30 seconds.

  ## Research Report
  ### Market Context
  - **Traditional Builders:** Shopify, Wix, Squarespace are powerful but rely on complex plugin ecosystems ("App Tax" fatigue) and desktop-first setups. Their AI tools (like Shopify Sidekick) often only act in an advisory capacity ("chatbots that advise").
  - **AI-Native Rivals:** Durable, 10Web, Mixo are gaining traction by offering near-instant setup via AI generation, removing the technical hurdle completely.
  - **OHC Pain Point:** OHC lacks the "Zero-to-One" autonomous experience. The current manual setup contradicts the core promise of leading users from "unclear work -> clear next action in minutes" via an assistant-led flow.

  ### Proposed Action
  Develop an **"Agentic Zero-Click Onboarding System"** to eliminate the manual setup hurdle. This system will allow a user to simply provide a natural language prompt (e.g., "I'm a baker in Austin selling custom cakes via IG DMs") and have an AI agent autonomously configure the tenant, generate a basic product catalog, set up an initial storefront, and configure basic Stripe deposit links. This shifts OHC from "advisory AI" to "executing AI."

  ## Design Doc
  ### Mobile UX Flow
  1.  **The Prompt:** New user opens the app (375px mobile view). The only UI is a clean, conversational chat interface ("What kind of business do you run?").
  2.  **The Wait State:** User enters "I bake custom wedding cakes in Austin." A translucent "Glass" loading card appears indicating "Agent is setting up your shop...".
  3.  **The Reveal:** The feed populates with the first Action Card: "Your shop is ready! We created 3 sample cake products and set up a deposit booking link."
  4.  **The Polish:** User taps the card to review the generated store. All UI uses the OHC Premium Token library (Apple/Ubiquiti style).

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant User (Mobile App)
      participant OHC API Gateway
      participant KAIROS Orchestrator
      participant LLM Provider (Gemini Pro)
      participant Tenant Provisioner
      participant Commerce Service

      User (Mobile App)->>OHC API Gateway: Submit NL Prompt ("Baker in Austin...")
      OHC API Gateway->>KAIROS Orchestrator: Initiate Onboarding Event
      KAIROS Orchestrator->>LLM Provider (Gemini Pro): Classify Intent & Extract Business Meta (Name, Type, Products)
      LLM Provider (Gemini Pro)-->>KAIROS Orchestrator: Structured JSON (Business Profile)
      KAIROS Orchestrator->>Tenant Provisioner: Create Tenant & Default Configs
      KAIROS Orchestrator->>Commerce Service: Generate Initial Catalog & Deposit Settings
      KAIROS Orchestrator-->>User (Mobile App): Push Action Card "Shop is Ready" via Agent Feed
  ```

  ### AI Agent Integration Points
  -   **Intent Classification Agent:** Parses the initial user prompt to determine business category (Physical, Service, Food) and extract key entities.
  -   **Content Generation Agent:** Drafts initial product descriptions, placeholder images (via external tool or smart placeholders), and basic shop policies based on the business type.
  -   **Orchestration:** KAIROS manages the distributed locks to ensure the tenant isn't accessed before the onboarding agents finish execution.

  ### Key Design Decisions
  -   **Conversational Entry:** Replacing forms with a single chat input reduces cognitive load and adheres to the "Radical Simplicity" core value.
  -   **Asynchronous Agent Work:** The generation happens asynchronously, aligning with the Agent Feed architecture where work is pushed to the owner for approval.
  -   **Strict Multi-Tenancy:** The provisioner must ensure row-level security (`tenant_id`) is properly initialized before any dummy data is inserted.

  ## Implementation Prompt
  **Mission:** Build the "Zero-Click Onboarding" flow for new users.
  **Objective:** When a new user signs up, present a single conversational prompt asking about their business. Based on their answer, orchestrate background agents to create the `tenant` record, generate 2-3 sample products/services appropriate for their business type, and configure a basic Stripe deposit link.
  **CUJ (Critical User Journey):**
  1. Maya signs up on her iPhone.
  2. She types "I make custom cakes in Austin."
  3. The system shows a loading state.
  4. She receives an Action Card in her feed saying "Store setup complete" with a preview of her new shop.
  **Acceptance Criteria:**
  - The UI must be fully functional on a 375px mobile screen.
  - The backend must parse the prompt, create a tenant, and populate initial data without any manual form filling from the user.
  - E2E Playwright tests must verify the flow from prompt entry to the appearance of the setup completion Action Card.
  - No mock data should be used; actual API calls to the LLM (or a local test adapter) and database insertions must occur.

  **Priority:** P0
  **Estimated Scope:** Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
