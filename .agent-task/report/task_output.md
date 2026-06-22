issue_title: "Research: Enhance OHC Mobile Setup Onboarding Flow"
issue_description: |
  # Architecture Gap: Mobile-First Onboarding & Setup

  ## Problem Statement
  Based on the competitive analysis of OHC against tools like Shopify, Durable, Wix, and Squarespace, it is clear that OHC lacks a robust, < 10-minute mobile-first onboarding flow that allows non-technical owners (e.g. Maya the baker, Carlos the handyman) to transition from "blank state" to "ready to work".
  Currently, owners must manually set up services, navigate dashboards, and click through multiple screens to prepare their environment. We are missing the "Zero-Click Onboarding Agent" identified in our research which allows a user to provision their space using natural language.

  ## Research Report
  - **Shopify & traditional platforms** require complex 30-60 min configuration.
  - **Durable AI & 10Web** can stand up a storefront in <1 minute.
  - OHC sits at roughly 1 hour for manual configuration.
  - User sentiment shows 73% of non-technical users abandon setups if they involve extensive manual inputs instead of straightforward conversational prompts.

  ## Design Doc
  - **Architecture diagram (Mermaid.js)**
  ```mermaid
  graph TD;
    User[Owner: 375px Mobile View] --> SetupUI[Conversational UI Component];
    SetupUI --> AssistantLayer[OHC AI Agentic Layer];
    AssistantLayer --> AgentOps[Agent Ops Exec (CRUD)];
    AgentOps --> DB[PostgreSQL Multi-Tenant DB];
    DB --> View[Dashboard Feed populated];
  ```
  - **UI wireframes/Flow (375px)**:
      1. Welcome Screen: "Hi, I'm your OHC assistant. Tell me about your business."
      2. Chat Interface: Owner types "I'm a baker in Austin, I do custom cakes".
      3. Processing State: "Creating your catalog, setting up booking, preparing initial offers..." (Translucent Glass modal).
      4. Success State: Directs user to the "Dashboard Feed" showing 3 active tasks/reminders generated from the prompt.
  - **AI Agent Integration**:
      - Introduce a dedicated Setup/Onboarding system prompt for Gemini/OpenAI that accepts a short text description and returns a structured JSON payload defining initial products, services, and tasks.
      - Route this JSON payload to the existing multi-tenant CRUD APIs to persist the data to the tenant.

  ## Implementation Prompt
  **Goal:** Build a mobile-first (375px) conversational onboarding screen in Flutter/Next that captures a single business description from the user, uses the AI Assistant backend to generate a basic setup (1 product/service, 1 sample customer, and 1 feed item), and commits it to the database so the user starts with a populated work feed.

  **Acceptance Criteria:**
  1. The user logs in and sees an empty state "Setup" screen on mobile.
  2. The user enters a sentence (e.g., "I'm Carlos, I fix plumbing").
  3. The backend agent generates 1 service (Plumbing Repair), 1 sample task (Follow up with new leads), and seeds it into the PostgreSQL DB.
  4. The user is redirected to the home feed, which now displays the new data using the macOS Translucent Glass UI styling.
  5. The UI must be fully functional on a 375px viewport.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
