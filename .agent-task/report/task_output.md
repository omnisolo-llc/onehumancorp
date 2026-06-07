issue_title: "Implement Agent Feed Action Cards for Proactive Owner Interaction"
issue_description: |
  # OHC Agent Feed: Action Cards Implementation

  ## Problem Statement
  Currently, business owners relying on platforms like Shopify or GoDaddy suffer from "dashboard fatigue"—they must constantly pull information from various tools, check inboxes, and manage apps to figure out what needs attention. For OneHumanCorp (OHC) to fulfill its promise of an "AI work assistant," the system needs a centralized, proactive feed where agents push critical updates, suggested actions, and drafted communications directly to the user. The platform must transition from a reactive tool to a proactive agent interface.

  ## Research Report
  Based on our analysis of competitor systems (Shopify Sidekick, Wix, GoDaddy) and market dynamics, standard AI integrations are mostly reactive chatbots (e.g., Shopify Sidekick) or basic setup generators. Non-technical owners (like our personas Maya, Carlos, and Fatima) need a "Zero-Setup" experience where AI departments (Operations, Customer Success, Marketing) coordinate invisibly and present a clear, unified feed of actionable items.

  Competitors fail at unifying operations; users suffer from app bloat and disjointed UIs. OHC's key differentiation is the Agent Feed, unifying messages, tasks, bookings, and alerts into simple Action Cards optimized for a 375px mobile screen.

  ## Design Doc: Agent Feed Architecture

  ### Core Concept
  The Agent Feed serves as the primary command center. When the backend event ingestion pipeline (e.g., Stripe webhook, new order, new Instagram DM) fires, the Intent & Context Resolution layer (LLM) classifies the intent, queries tenant-specific data via RAG, and generates an actionable response. This draft is pushed as an "Action Card" to the feed.

  ### Mobile UX Flow (375px First)
  1.  **Feed Entry**: The primary view is a vertical feed of Action Cards.
  2.  **Action Card Structure**:
      *   **Header**: Context (e.g., "Ambassador Agent: New DM from @user").
      *   **Body**: The drafted response or suggested action.
      *   **Actions**: "Approve", "Edit", "Discard" (prominent, 44x44px touch targets).
  3.  **Interaction**: Tapping "Approve" triggers the backend to execute the action (e.g., send the DM via Instagram API) and removes/collapses the card. Tapping "Edit" opens a modal (native keyboard friendly) to tweak the draft.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant External as Webhook/API (e.g., IG, Stripe)
      participant Ingestion as Event Ingestion (Redis Pub/Sub)
      participant LLM as Intent & Context Resolution (LLM+RAG)
      participant DB as Postgres (Tenant Data)
      participant Feed as Agent Feed API
      participant App as Mobile App (375px)

      External->>Ingestion: Event Triggered
      Ingestion->>LLM: Process Event
      LLM->>DB: Query Context (Inventory, FAQs)
      DB-->>LLM: Context Data
      LLM->>Feed: Draft Action/Response
      Feed->>App: Push Action Card
      App->>App: User Reviews Card
      App->>Feed: User Approves
      Feed->>External: Execute Action
  ```

  ### AI Agent Integration Points
  *   **Event Pipeline**: Hook into existing message buses or event queues.
  *   **Prompt Architecture**: Each agent capability (e.g., Ambassador, Promoter) needs a `system_prompt`, tenant-scoped memory access, and structured output for the Action Card format.

  ## Implementation Prompt (For Implementer Agents)
  **Feature**: Proactive Agent Feed Action Cards.

  **Target Persona**: Maya (Home Baker) using her iPhone (375px viewport).

  **Outcome**: When Maya receives a DM, the system automatically drafts a context-aware response based on her inventory and pushes it to her Agent Feed as an Action Card. She can approve it with one tap.

  **Critical User Journey (CUJ) to Implement:**
  1.  Create the backend API endpoints and data models for the `AgentFeed` and `ActionCard`.
  2.  Implement the UI for the vertical Agent Feed on the mobile-first shell (Flutter/PWA).
  3.  Design the Action Card component with "Approve", "Edit", and "Discard" actions, adhering strictly to the OHC Premium Token library (translucent materials, clean hierarchy) and 44x44px touch targets.
  4.  Wire a test event (e.g., a mock DM inquiry about a product) through the pipeline to appear as a card.
  5.  Implement the "Approve" flow, ensuring it triggers the intended action and updates the UI state (truthful pending/success states).

  **Acceptance Criteria:**
  *   The feed UI must be fully functional and aesthetically premium on a 375px viewport without horizontal scroll.
  *   Data must flow end-to-end from the backend API to the UI.
  *   Playwright E2E tests MUST verify the feed rendering, card approval interaction, and state changes using the real OHC stack (no UI mock data).

  ## Estimated Scope
  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
