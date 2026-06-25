issue_title: "Implement Autonomous Zero-Click Onboarding Agent for Single-Prompt Setup"
issue_description: |
  ## Mission Queue Protocol

  **Title:** Implement Autonomous Zero-Click Onboarding Agent for Single-Prompt Setup

  **Problem Statement:**
  Setup paralysis is the largest friction point for non-technical small business owners like Maya (the home baker). Our research shows that 34% of SMB operators abandon digital setup tools due to the "technical complexity" of configuring domains, menus, and booking rules. Traditional systems like Shopify and Wix offer chatbots that provide advice, but they still demand 30-60 minutes of manual configuration. Maya doesn't want to learn how to configure DNS, set up stripe accounts, or define product catalogs—she just wants to state her business intent and see a functioning platform.

  **Research Report:**
  - **Market Landscape:** Competitors like Durable.co have introduced "30-second site generation" that produces basic functional websites. However, these lack deep operational integration (booking, custom quotes, deposits) right out of the box. Shopify's Sidekick helps manage an existing store but doesn't autonomously bootstrap a complete new entity from scratch.
  - **The Gap:** OHC currently takes an estimated 1 hour of manual setup. To fulfill the vision of an Owner Work Assistant, we must cut this to < 10 minutes by shifting from "AI advising" to "AI executing".
  - **Persona Fit:** Maya (Baker) can provide a single prompt ("I sell custom vegan cakes in Austin. I need 50% deposits and 3 days notice.") and the agent will synthesize a fully-operational, mobile-first workspace.

  **Design Doc:**

  *Architecture Design (Mermaid.js)*
  ```mermaid
  graph TD
      A[Owner/Maya] -->|Natural Language Prompt| B(Zero-Click Onboarding Interface)
      B --> C[Orchestrator Agent / KAIROS]
      C --> D[Business Taxonomy Analyzer]
      C --> E[Data Schema Synthesizer]
      C --> F[UI Layout Generator]

      D --> G{Domain Needs}
      G -->|Products| H(Catalog / Inventory Config)
      G -->|Services| I(Booking / Calendar Config)

      E --> J[(Tenant Isolated PostgreSQL)]
      F --> K[Mobile-First View Layer]

      H --> J
      I --> J
      K --> L(Live 375px Storefront & Owner Dashboard)
  ```

  *Mobile UX Flow (375px Baseline)*
  1. **Greeting Screen:** "What do you do?" (Large text input field, native keyboard).
  2. **Generating State (Translucent Glass UI):** A dynamic loading screen showing the agent's internal monologue ("Creating product schemas...", "Setting up default deposit rules..."). This builds trust through transparency without exposing technical jargon.
  3. **Review & Refine Screen:** The agent presents a generated mobile storefront. Maya can tap "Looks good, let's launch" or chat to refine ("Change the theme to pastel pink").

  *AI Agent Integration Points*
  - **LLM Prompting (Gemini Pro/MiniMax):** A specialized `system_prompt` focusing on extracting business entities (products, services, policies) from raw unstructured text.
  - **Structured Tool Execution:** The agent calls backend tools to provision `Tenant` records, `Product` models, and `Configuration` settings automatically.
  - **Observable Handoff:** Once generated, the Onboarding Agent hands off the initialized workspace context to the permanent Operations Assistant for daily tasks.

  **Implementation Prompt:**
  *Target Implementer: KAIROS Architecture Swarm*
  "Design and implement the 'Zero-Click Onboarding Agent' flow. Create a mobile-first (375px) chat-based onboarding sequence. The UI should accept a single natural language description of a business. The backend must orchestrate an AI agent that takes this prompt, extracts the business taxonomy, and executes the necessary database mutations (creating the tenant, setting up initial catalog/booking rules, and applying default Stripe deposit rules). The acceptance criteria is that a user can type 'I am a handyman in Chicago needing quotes and a calendar' and immediately be redirected to a fully populated, tenant-isolated owner dashboard with zero manual configuration steps. Ensure the UI adopts the Translucent Glass design tokens."

  **Priority:** P0 (Critical path for Activation)
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []