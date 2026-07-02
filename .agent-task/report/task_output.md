issue_title: "Zero-Click Agentic Storefront Generation & Onboarding Engine"
issue_description: |
  **Problem Statement**:
  Small business owners (like Maya the baker and Carlos the handyman) experience "setup paralysis" when confronting a blank canvas or complex admin panels (like Shopify or WordPress). Research shows 73% of non-technical users abandon complex setups. They need a system that translates a single natural language sentence into a fully configured, multi-tenant isolated storefront, complete with product catalogs, dynamic pricing, and an optimistic UI, without requiring them to navigate traditional settings pages.

  **Research Report**:
  - **Market Dynamics**: Competitors like Durable (30-second site generation) and Shopify (Sidekick) are moving towards AI assistance. However, Shopify's onboarding still takes 30-60 minutes and requires manual theme/app configuration. Wix and Squarespace require significant drag-and-drop effort.
  - **Pain Point**: SMB owners want an AI that *executes* the setup, not just advises on it.
  - **Competitor Audit**:
    - Shopify Sidekick: Chatbot assistant for existing setups, relies heavily on apps.
    - Durable: Great at initial generation but lacks deep operational backend (inventory, scheduling).
  - **OHC's Opportunity**: An "Operations Manager" Agent Protocol that leverages the OHC hybrid KAIROS orchestrator. A single conversational prompt ("I'm a baker in Austin") triggers the LLM to generate the `tenant_id` DB schema, product entities, localized taxes, and rendering rules.

  **Design Doc**:
  - **Architecture Diagram (Mermaid.js)**:
  ```mermaid
  graph TD;
      User[Mobile App User] -->|Natural Language Prompt| API[API Gateway]
      API --> Intent[Intent & Context Resolution LLM Layer]
      Intent --> Operations[Operations Manager Agent]
      Operations -->|Creates Tenant & Schema| DB[(PostgreSQL Ledger)]
      Operations -->|Configures Identity| SPIRE[SPIFFE/SPIRE]
      Operations -->|Provisions UI Elements| Storefront[Dynamic Storefront Engine]
      Storefront -->|Returns Preview| User
  ```

  - **Mobile UX Flow (375px First)**:
    1.  **Welcome Screen**: A simple, translucent glass conversational interface.
    2.  **Input**: User dictates or types: "I want to sell custom cakes in Austin."
    3.  **Loading/Generation State**: AI agent shows progress ("Generating catalog...", "Configuring Austin tax rates...").
    4.  **Action Card**: A push-notification style card showing the generated storefront preview.
    5.  **Review**: A simple "Approve & Launch" button or "Edit" button. Touch targets are at least 44x44px. The design uses OHC Premium Token translucent macOS/UniFi curves.

  - **AI Agent Integration Notes**:
    - Leverages the "Zero-Click Onboarding Agent".
    - Communicates via the central message bus (Redis Pub/Sub).
    - Uses the `HybridCache` for optimistic state generation before committing to the `Ledger_DB` with strict `tenant_id` boundaries.

  **Implementation Prompt**:
  User Journey: Maya opens the app for the first time. She inputs, "I'm a baker in Austin selling custom cakes." The Agentic Onboarding Engine autonomously provisions her tenant, creates a basic cake product catalog, configures local Texas sales tax, and generates a mobile-responsive storefront. She reviews the generated 375px preview and taps "Approve."

  To the Implementer Agent:
  Implement the core `AgenticOnboardingService` backend capabilities.
  1. Create the API endpoints and gRPC definitions for accepting the initial natural language business description.
  2. Implement the integration with the central LLM intent resolution layer to parse the description into structured `Tenant`, `Product`, and `Tax` payloads.
  3. Ensure the service safely provisions these entities within the PostgreSQL database using strict multi-tenant row-level security (`tenant_id`).
  4. Emit an event to the Event Mesh confirming setup completion to trigger the dynamic storefront generation.
  5. Do not prescribe specific ORMs or UI frameworks; focus on the robust, secure entity generation and agent handoff. Ensure 100% unit test coverage.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
