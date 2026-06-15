issue_title: "[Research] Full Platform Architecture & Zero-Setup Autonomy"
issue_description: |
  # Title: Research Report: Platform Architecture for Zero-Setup Autonomy

  ## 1. Problem Statement
  Our research has identified a core dichotomy in the current market for SMB software solutions:
  - **Traditional Legacy Builders (Shopify, Wix, Squarespace)**: These platforms offer powerful, feature-rich ecosystems but fail significantly in setup and day-to-day operations for micro-businesses. They expect the user to act as an IT administrator, manually configuring shipping zones, installing 10+ third-party plugins, and writing marketing copy. Setup takes days to weeks.
  - **AI-Native Innovators (Durable, 10Web, Framer)**: These tools solve the "Setup Paralysis" by generating a site in under a minute via AI. However, they lack the deep, robust backend operations needed to actually run the business (inventory management, complex quoting, staff scheduling).
  - **The OHC Opportunity**: A small business owner (like Maya the Baker) signs up for Shopify and immediately hits a wall configuring Stripe and delivery zones, often abandoning the setup. OneHumanCorp must bridge this gap by offering **Invisible Automation**—an AI workforce that not only builds the site but runs the site via specialized departments.

  ## 2. Research Report
  ### Competitive Audit & OHC Gap
  - **The Problem**: 34% of SMBs abandon complex setups. The "Shopify Tax" requires merchants to pay for and configure multiple plugins.
  - **The OHC Gap**: OHC currently requires manual intervention to set up a workspace, products, and services. It lacks a fully autonomous, "Zero-Click" onboarding experience.

  ## 3. Design Doc
  To achieve true Zero-Setup Autonomy, OHC must architect its system around **AI Departments** rather than static features.

  - **The Setup Agent**: Replaces the traditional SaaS setup wizard with a conversational interface. Provisions the multi-tenant DB schema, generates the storefront, drafts the product catalog, and configures Stripe connect.
  - **The Operations Agent**: Monitors booking and POS services. Checks travel time, schedules slots, and drafts confirmations.
  - **The Marketing Agent**: Continuously monitors inventory and abandoned carts to draft social posts and emails.

  ### Mobile UX Flow (375px First)
  1. **Greeting Screen**: Full-screen conversational UI: "Hi Maya! Let's get your bakery online. What do you sell?"
  2. **Generation State**: Translucent glass loading indicator as the agent provisions the tenant.
  3. **Preview & Connect**: Displays a preview of the generated products with a 1-tap button to "Connect Bank for Deposits".
  4. **Unified Feed Transition**: Smooth animation into the Assistant-First Shell (Owner Dashboard).

  ### AI Agent Integration Points
  - The Frontend chat sends user intent directly to the Orchestration Layer.
  - Orchestrator triggers `Setup Agent` -> provisions DB, calls `Product Agent` to generate catalog items, calls `Theme Agent` to apply Glassmorphism design tokens.

  ### Architecture & Data Models

  #### High-Level Sequence Diagram
  ```mermaid
  sequenceDiagram
      actor Owner (Mobile)
      participant Conversational UI
      participant Orchestrator
      participant Setup Agent
      participant DB (Tenant Registry)

      Owner (Mobile)->>Conversational UI: "I make custom vegan cakes"
      Conversational UI->>Orchestrator: user_intent(setup, "vegan cakes")
      Orchestrator->>Setup Agent: Generate Business Profile
      Setup Agent->>DB (Tenant Registry): provision_tenant(name="Vegan Cakes", type="Bakery")
      Setup Agent->>DB (Tenant Registry): create_sample_products(intent)
      Setup Agent->>Orchestrator: setup_complete(tenant_id)
      Orchestrator->>Conversational UI: Transition to Owner Dashboard
  ```

  #### Entity-Relationship Diagram (Multi-Tenant Isolation)
  ```mermaid
  erDiagram
      TENANT {
          uuid id PK
          string name
          string business_type
      }
      AGENT_SESSION {
          uuid id PK
          uuid tenant_id FK
          string status
      }
      PRODUCT {
          uuid id PK
          uuid tenant_id FK
          string name
          string description_ai
      }
      TENANT ||--o{ AGENT_SESSION : "has"
      TENANT ||--o{ PRODUCT : "owns"
  ```

  ## 4. Implementation Prompt
  **Feature Name:** Autonomous Agentic Onboarding Flow
  **Target Persona:** Maya the Home Baker
  **Objective:** Build a conversational onboarding interface that provisions a new business workspace without standard web forms.

  **Critical User Journey (CUJ):**
  1. Maya opens the OHC PWA on her iPhone (375px view).
  2. Instead of a standard signup form, she sees a chat UI: "Hi! Let's get your business online. What do you sell?"
  3. Maya types: "I make custom vegan cakes."
  4. The frontend sends this to the `Setup Agent`.
  5. The `Setup Agent` automatically generates a mock business profile, provisions the DB tenant, selects a bakery-appropriate premium theme, and creates three sample products with AI-generated descriptions.
  6. The chat UI responds: "Great! I've set up your bakery. Here is a preview link. Would you like me to connect a bank account to start taking deposits?"
  7. The UI transitions from Chat to the main Owner Dashboard (Unified Feed).

  **Acceptance Criteria:**
  - Build the Chat UI component in Flutter/Next.js (depending on current stack), applying strict 375px mobile constraints and Apple-style Translucent Glass aesthetics.
  - The chat interface must be capable of passing user intent to the backend to trigger the creation of a Tenant and sample Product data.
  - The UI must NOT contain mock data; it must hit the real backend API (or a documented local LLM adapter for testing).
  - Implement comprehensive Playwright E2E tests verifying the user can complete the chat flow and arrive at a populated dashboard.

  ## 5. Priority & Scope
  - **Priority:** P0 (Critical)
  - **Estimated Scope:** Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
