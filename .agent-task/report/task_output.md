issue_title: "[Research] AutoDream Zero-Click Onboarding Pipeline"
issue_description: |
  # Research Report: AutoDream Zero-Click Onboarding Pipeline

  ## 1. Problem Statement
  Based on the competitive analysis of Shopify, Wix, and other SMB platforms, the highest friction point preventing non-technical users from digitizing their business is "Setup Paralysis." Existing platforms provide a blank canvas or generic templates, forcing the user to become an amateur web designer and copywriter. Emerging AI builders generate static sites but fail to configure backend business operations (inventory, bookings).
  OHC needs a true "Zero-Click" onboarding experience where a single conversational prompt autonomously provisions the entire operational state of the business, going from idea to fully configured storefront and database in under a minute.

  ## 2. Research Report
  Our market research ("ohc_market_dominance_agentic_workflows_vs_traditional_platforms_research_report.md" and "ohc_smb_platform_research_report.md") highlights that:
  - 42% of users struggle with complex setup and configuration paralysis.
  - SMB owners like Maya (The Home Baker) or Carlos (The Handyman) run their businesses from their phones and abandon complex desktop-first onboarding flows.
  - Competitor AI features (like Shopify Sidekick) are reactive chatbots that *advise* on configuration rather than *executing* it.

  **The Strategic Gap:** OHC must deliver an "AutoDream Pipeline" that acts as an invisible Agentic Department. It should take a simple user intent, break it down, and automatically execute the necessary database and state mutations to set up the store.

  ## 3. Design Doc

  ### Architecture Overview
  The AutoDream Zero-Click Onboarding relies on bridging the conversational intake with deterministic state generation.

  ```mermaid
  sequenceDiagram
      participant User (Mobile)
      participant Onboarding API
      participant AutoDream Pipeline (Agent)
      participant Database (PostgreSQL)

      User->>Onboarding API: "I am a baker in Austin needing to sell custom cakes"
      Onboarding API->>AutoDream Pipeline: Trigger AutoDream generation task
      AutoDream Pipeline->>AutoDream Pipeline: Process LLM extraction (business type, services, products)
      AutoDream Pipeline->>Database: Mutate schema (Create tenant, products, services)
      AutoDream Pipeline->>Onboarding API: Return fully formed Workspace state
      Onboarding API->>User (Mobile): Present fully functional store for 1-tap approval
  ```

  ### Mobile UX Flow
  1. **Intake Screen (375px):** A clean, chat-like interface asking: "What business are you building today?"
  2. **Processing State:** A dynamic loading screen indicating the agent is "Building inventory", "Writing copy", and "Configuring checkout".
  3. **Review & Approve:** The user sees a fully functional preview of their mobile store. They can make minor edits or tap "Launch My Business".

  ### AI Agent Integration
  - **Onboarding Agent / AutoDreamWorker:** The agent needs to interpret natural language, define the necessary business entities (e.g., physical products vs. service bookings), and translate that into structured data.
  - **Structured Output Generation:** Ensure the LLM outputs strictly typed JSON schemas that the backend can use to populate the `products`, `services`, and `tenant` tables.

  ## 4. Implementation Prompt
  **Task:** Implement the core backend orchestration for the AutoDream Zero-Click Onboarding flow.

  **Persona:** Maya (Home Baker) using her iPhone. She types: "I want to sell custom wedding cakes and cupcakes in Austin."

  **Outcome:** The system should take this text, use an LLM/Agent to parse the business requirements, and automatically generate a structured JSON payload representing her new business (including a name, description, and initial sample products/services). This payload should be saved as her draft state, ready for 1-tap approval.

  **Acceptance Criteria:**
  - The `process_intake` endpoint (or equivalent AutoDream pipeline integration) must successfully parse a free-text description and return a structured configuration (e.g., business name, inferred category, list of generated sample products).
  - The generated configuration must be saved to the database (or mock equivalent during development) associated with the user/tenant.
  - The flow must be executable without requiring the user to manually fill out separate forms for products, categories, or store details.
  - E2E tests must verify that an intake request results in a correctly populated draft state.

  ## 5. Priority & Scope
  - **Priority:** P0 (Critical path for user acquisition and core product differentiation)
  - **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
