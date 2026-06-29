issue_title: "Architectural Design: Zero-Click Autonomous Onboarding Agent"
issue_description: |
  # Mission Queue Protocol: Zero-Click Autonomous Onboarding Agent

  ## Problem Statement
  Currently, OneHumanCorp (OHC) onboarding takes approximately 1 hour and requires manual configuration of services, products, and payment gateways. Based on competitor research (Durable.co, Shopify Sidekick), 34% of small business owners abandon platform setup due to technical complexity. For our core personas like Maya (the baker) or Carlos (the handyman), configuring DNS, setting up Stripe for deposits, and manually creating product listings are non-starter tasks. They want to sell cakes and book services, not administer software. We lack the "Zero-to-One" autonomous experience that AI-native competitors provide.

  ## Research Report
  - **Competitor Analysis:** Durable.co generates a complete business website, CRM, and invoicing in under a minute via simple prompts. Wix Studio AI and Squarespace Blueprint guide users but still require significant manual design adjustments. Shopify Sidekick assists with store management but does not eliminate the initial setup friction.
  - **Target Persona:** Maya (Home Baker) & Carlos (Field Service).
  - **Gap Identified:** OHC lacks a unified, invisible onboarding agent that can intake a simple natural language prompt (or voice note) and autonomously configure the tenant workspace, products, payment integrations, and initial storefront.
  - **Proposed Solution:** A "Zero-Click Onboarding Agent" under the KAIROS orchestration engine. The owner simply chats with OHC for 5 minutes. The agent autonomously provisions the domain/tenant context, configures Stripe for custom deposits, creates the first product/service from a photo or description, and sets up the booking calendar.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      Owner[Owner/Operator] -->|Natural Language / Photo| OHCApp[OHC Mobile Shell 375px]
      OHCApp -->|gRPC/REST| TriageAgent[KAIROS Triage Agent]
      TriageAgent -->|Context Analysis| DeptRouter[Department Router]

      DeptRouter -->|Provisioning| OpsAgent[Operations Agent]
      DeptRouter -->|Catalog| SalesAgent[Sales & Revenue Agent]
      DeptRouter -->|Brand| MarketingAgent[Brand & Marketing Agent]

      OpsAgent -->|Tenant Config| Postgres[(PostgreSQL Row-Level Security)]
      SalesAgent -->|Stripe Connect/Deposit| Payments[(Payment Service)]
      MarketingAgent -->|Storefront Asset| Blob[(GCS/MinIO Storage)]

      OpsAgent --> Feed[Owner Work Feed]
      SalesAgent --> Feed
      MarketingAgent --> Feed
      Feed --> OHCApp
  ```

  ### Mobile UX Flow (375px First)
  1. **Welcome Screen:** "Tell us what you do or upload a photo of your work." (Native mobile keyboard/camera input).
  2. **Agent Processing State:** Translucent glass overlay showing real-time agent tasks (e.g., "Creating 'Custom Vegan Cake' product...", "Setting up booking calendar...").
  3. **Verification Screen:** A unified, Apple/Ubiquiti-style card layout displaying the configured store. "Here is your new storefront. Do you want to connect a bank account now to accept $50 deposits?"
  4. **Action:** One-tap approval to finalize the setup and transition to the daily Owner Work Feed.

  ### AI Agent Integration Points
  - **LLM Provider:** Gemini Pro (primary) with fallback to OpenAI/MiniMax.
  - **Agent State:** Redlock used to coordinate Ops, Sales, and Marketing agents so they don't overwrite the same tenant setup records.
  - **System Prompting:** The KAIROS Triage Agent is prompted to ask a maximum of 3 clarifying questions before executing the setup.

  ### Key Design Decisions
  - **Conversational UI over Forms:** Replaces the traditional multi-step setup wizard with a chat/voice-first interface.
  - **Piecemeal Approval:** The agent drafts the entire setup (products, calendar, pricing) into a pending state. The owner approves it with a single tap, moving records from `draft` to `active`.
  - **Mobile-First Constraints:** The onboarding chat and preview cards must be perfectly touch-optimized (44x44px minimum targets) for 375px viewports, completely hiding complex settings behind "Advanced Options."

  ## Implementation Prompt
  **To the Implementer:**
  Implement the "Zero-Click Onboarding Agent" feature in the frontend and backend.
  1. Create a conversational onboarding UI in the Tauri/Flutter shell that works beautifully on a 375px width. Use translucent materials and UniFi-style clean cards.
  2. Implement the backend KAIROS agent routing to handle the initial business description prompt.
  3. The agent should be able to autonomously create at least one Product/Service and initialize the tenant's workspace configuration based on the user's input.
  4. Add an E2E Playwright test simulating Maya the Baker uploading a cake description and the system generating her store.
  Do not prescribe specific database tables or schemas; design the most efficient multi-tenant architecture to support this flow.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
