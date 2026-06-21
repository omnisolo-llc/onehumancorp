issue_title: "Implement Zero-Click Agentic Onboarding Flow for SMB Owners"
issue_description: |
  ## Problem Statement
  Small business owners like Maya (Home Baker) experience "Setup Paralysis". 34% of SMB owners abandon setup due to technical complexity. They want to sell custom products (like cakes) but struggle with configuring DNS, Stripe deposits, product variants, and photo catalogs. OHC currently takes ~1 hour of manual setup. We need to reduce this to < 10 minutes using a conversational agentic flow that configures the workspace invisibly.

  ## Research Report
  - **Market Findings:** Competitors like Durable.co generate a full website and CRM from a single sentence in under 30 seconds. Shopify's "Sidekick" assists with store modifications but requires manual initial store setup.
  - **Codebase Discovery:** OHC has a robust KAIROS orchestration engine and specialized multi-tenant PostgreSQL backing. However, the onboarding flow is predominantly manual and lacks an entry-point for the AI agent to orchestrate the entire setup (business name, primary product, deposit configurations).
  - **User Pain Points:** Non-technical owners cannot be bothered with terms like "Payment Gateways", "DNS records", or "Webhooks". The interface must speak their language: "How much deposit do you require for custom cakes?"

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      actor Maya
      participant OnboardingUI as Mobile Assistant UI
      participant OnboardingAgent as Zero-Click Agent
      participant KAIROSEngine as KAIROS Orchestrator
      participant TenantService as Tenant API
      participant StripeService as Payment API

      Maya->>OnboardingUI: "I sell custom cakes on Instagram."
      OnboardingUI->>OnboardingAgent: Prompt: Setup Bakery Business
      OnboardingAgent->>KAIROSEngine: Dispatch Setup Tasks
      KAIROSEngine->>TenantService: Create Tenant "Maya's Cakes"
      KAIROSEngine->>StripeService: Provision Connect Account & Deposit Link
      KAIROSEngine->>TenantService: Create "Custom Cake" Product with Photo Requirement
      OnboardingAgent-->>OnboardingUI: "You're set! Here is your custom deposit link for Instagram."
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  1. **Greeting Screen:** A clean, translucent glass interface (Apple/Ubiquiti style). "What do you do?" with a native keyboard input or voice dictation.
  2. **Agentic Processing:** Loading state showing "Creating your workspace...", "Setting up payment deposits...", "Drafting product catalog...".
  3. **Assistant Feed (Home):** The owner lands directly on the Assistant Feed. First card: "Your workspace is ready. Let's upload a photo of your best cake."

  ### AI Agent Integration Points
  - **Zero-Click Onboarding Agent:** Uses Gemini Pro to parse the user's initial description and output a structured JSON of `TenantProfile`, `InitialProduct`, and `PaymentRequirement`.
  - **Background Orchestration:** Dispatches standard internal API requests (Tenant creation, Product creation) based on the LLM's parsed intents.

  ### Key Design Decisions
  - **Conversational Entry:** Bypass standard multi-step forms. The LLM extracts required fields from natural language and prompts for missing critical info (e.g., "Do you need a deposit?").
  - **Immediate Value:** The user should receive a functional URL or payment link within minutes, before being asked to set up "Advanced Settings".

  ## Implementation Prompt
  **Role:** Frontend & Backend Implementer
  **Task:** Implement the "Zero-Click Onboarding" conversational flow.
  **CUJ:**
  1. A new user opens the OHC mobile web app (375px).
  2. The user sees a single chat-like input asking about their business.
  3. The user types: "I bake and sell custom vegan cakes via Instagram. I need to take a $20 deposit for orders."
  4. The system leverages the new Onboarding Agent to process this text.
  5. The backend automatically creates the Tenant, sets up a generic "Custom Vegan Cake" product, and configures a deposit rule.
  6. The user is transitioned to the main Assistant Feed, showing a welcome message and their new product's shareable link.
  **Acceptance Criteria:**
  - 100% functional on 375px mobile without horizontal scrolling.
  - The conversational setup correctly provisions the backend database tables with proper row-level tenant isolation.
  - Playwright E2E test verifying the flow from chat input to the generated product link in the Feed.
  - Zero mock data; must interact with the real `KAIROS` agent layer.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
