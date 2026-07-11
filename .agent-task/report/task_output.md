issue_title: "Implement Zero-Click Agentic Onboarding Flow"
issue_description: |
  # Zero-Click Agentic Onboarding Flow

  ## Problem Statement
  SMB owners (like Maya the baker or Carlos the handyman) experience "Setup Paralysis" when faced with a blank canvas or complex configuration screens. Legacy platforms (Shopify, Wix) require 30-60 minutes on a desktop to set up. OHC needs a "Zero-Click Generation" flow that takes a single sentence prompt (e.g., "I'm a baker in Austin") and autonomously generates the DB schema, product catalog, and storefront layout in under 10 minutes, fully operable from a 375px mobile screen.

  ## Research Report
  - Competitor analysis shows that 73% of non-technical users abandon complex setups.
  - Platforms like Durable generate sites quickly but lack deep operational capabilities (inventory, bookings, CRM).
  - Legacy platforms (Shopify) require third-party apps for basic SMB needs (bookings, reviews), leading to "App Tax Fatigue".
  - Link-in-bio tools are mobile-first but lack business logic depth.
  - OHC must combine the speed of AI generation with the depth of a full business platform, without the desktop requirement.

  ## Design Doc

  ### Key Design Decisions:
  1. **Conversational Entry Point:** The onboarding starts with a single chat input field: "What kind of business are you starting?".
  2. **The Promoter Agent (Marketing):** Takes the input, categorizes the business (e.g., "Food & Beverage"), and generates a preliminary brand identity (name, colors, typography).
  3. **The Manager Agent (Operations):** Generates the initial data schema (e.g., creating a 'Custom Vanilla Cake' product for Maya, or a '1-Hour Consultation' service for Carlos).
  4. **Progressive Unlocking:** Complex settings (custom domain, Stripe Connect) are hidden until the core value (a working storefront) is demonstrated.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      actor User
      participant App as OHC Mobile App (375px)
      participant API as OHC API Gateway
      participant Promoter as Marketing Agent (The Promoter)
      participant Manager as Operations Agent (The Manager)
      participant DB as PostgreSQL (Tenant Data)

      User->>App: Submits "I'm a baker in Austin"
      App->>API: POST /api/onboarding/generate
      API->>Promoter: Analyze intent & design brand
      Promoter-->>API: Returns Brand Theme JSON
      API->>Manager: Provision schema & initial inventory
      Manager->>DB: INSERT Tenant, Storefront, Products
      DB-->>Manager: Success
      Manager-->>API: Returns completed setup config
      API-->>App: Provisioning Complete (200 OK)
      App-->>User: Displays generated storefront preview
  ```

  ### Mobile UX Flow (375px Baseline):
  1. **Screen 1 (Welcome):** Large, friendly typography. Single input field. "I want to start a..." [Submit].
  2. **Screen 2 (Loading/Agent Working):** "The Promoter is designing your brand... The Manager is setting up your inventory..." (Translucent glass style progress indicators).
  3. **Screen 3 (The Reveal):** A full-screen preview of the generated storefront.
  4. **Screen 4 (Actionable Next Step):** "Looks good! Let's connect your bank to start taking orders."

  ### AI Agent Integration:
  - **Prompting:** The initial user input is sent to the LLM (Gemini Pro/GPT-4o).
  - **Structured Output:** The LLM must return a structured JSON response defining the business category, suggested products/services, and visual theme tokens.
  - **System Mutators:** The backend consumes this JSON to execute CRUD operations (creating the Tenant, configuring the Storefront, inserting initial Products into the Ledger).

  ## Implementation Prompt
  Implement the backend API endpoints and the corresponding Flutter mobile UI (starting at 375px) for the Agentic Onboarding Flow. The user should be able to input a single sentence describing their business. The backend should utilize the configured LLM to parse this intent, categorize the business, and automatically provision a Tenant with a default visual theme and at least one initial product or service catalog item. The UI should display a loading state while agents work, culminating in a preview of the generated storefront. Ensure all new logic is covered by 100% unit tests and at least one end-to-end Playwright test covering the user journey from the welcome screen to the storefront preview.

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
