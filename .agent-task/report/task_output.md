issue_title: "[research] Build Mobile-First, Agentic Unified Operations Manager & AI Action Feed"
issue_description: |
  # OHC Mobile-First, Agentic Unified Operations Manager & AI Action Feed Research Report

  ## 1. Executive Summary
  OHC is positioned to solve the SMB "App Tax Fatigue" and "Setup Paralysis" by moving from reactive software tools to proactive, invisible AI agents. The current market forces SMB owners (like Maya the Baker) to piece together 3-5 different tools and spend hours configuring tax, shipping, and email marketing. We aim to replace this with an AI-native ecosystem.

  Our core differentiation is the **Agent Feed (Unified Action Feed)** and **Zero-Click Onboarding**. Instead of a static dashboard full of metrics, the user logs into a mobile-first feed containing action cards from various AI "departments" (e.g., Marketing Agent proposing a new Instagram post based on inventory, Sales Agent proposing a quote for a new lead).

  ## 2. Competitive & Gap Analysis (Track 1 & 2)

  *   **Shopify Sidekick:** It is a reactive chatbot. You have to know what to ask ("How do I set up shipping?").
  *   **Wix/Squarespace/GoDaddy:** Excellent for initial setup, but lack autonomous operations. They provide the tool, not the staff.
  *   **Durable AI:** Incredible 30-second setup, but shallow backend operational capabilities.
  *   **OHC Gap:** We currently have disconnected operational services (`booking`, `quoting`, `pos`, `delivery`). We lack the central nervous system—the **Agent Feed**—to unify these into a proactive, owner-centric mobile experience. We also need to fully connect the **Zero-Click Onboarding** flow to this feed.

  ## 3. High-Level Architectural Design (Track 3)

  ### Mobile UX Flow (375px First)
  1.  **Login/Onboarding:** User enters the app. If new, they hit the "Zero-Click Business Generator" and describe their business in one sentence. The AI provisions the tenant, database schema, initial products, and configures the Agent Feed.
  2.  **The Home Screen (Agent Feed):** The main screen is NOT a dashboard of charts. It is a prioritized feed of Action Cards.
  3.  **Interaction:** A card from "The Ambassador" (Customer Success Agent) appears: "Maya, 3 customers asked about Vegan Cakes on Instagram. I checked inventory—we have 5 left. Should I reply and send a checkout link?"
  4.  **1-Tap Execution:** The user taps [Approve] or [Edit]. The agent executes the action.

  ### System Architecture
  *   **Event Pipeline:** Redis Pub/Sub captures internal events (inventory drops, new bookings) and external webhooks (Stripe, Instagram DMs).
  *   **Agent Evaluator:** A worker pool (Rust/Tokio) subscribes to events. It uses an LLM (Gemini/MiniMax) to evaluate intent against the tenant's context (RAG).
  *   **Feed Ingestion:** If the LLM determines an action is needed, it generates a proposed JSON payload and inserts a record into `agent_feed_items` (PostgreSQL) with `lifecycle_state = 'PENDING_APPROVAL'`.
  *   **Client Sync:** The Flutter/PWA mobile client receives a real-time update via WebSockets or periodic polling and renders the Action Card.
  *   **Execution (Action Router):** When the user taps [Approve], the client hits `PUT /api/v1/agent-feed/:id/state`. The backend updates the state and dispatches the proposed payload to the appropriate service (e.g., `booking`, `inbox`, `inventory`) via an `ActionRouter`.

  ```mermaid
  graph TD
      A[External Webhook / System Event] --> B(Event Pipeline - Redis)
      B --> C{AI Agent Evaluator Worker}
      C -- "No action needed" --> D[Log]
      C -- "Action Proposed" --> E[agent_feed_items DB]
      E --> F((Unified Agent Feed UI - Mobile))
      F -- "User Approves" --> G[Action Router]
      G --> H[Execution Service (e.g., Send Email, Update DB)]
  ```

  ## 4. Implementation Prompt (Track 4)

  **Mission for Engineering Swarm:** Implement the backend "Action Router" and the frontend "Unified Agent Feed" mobile components to enable proactive AI agent operations.

  **Critical User Journey (CUJ):**
  1. An external event occurs (e.g., a simulated webhook for a low inventory alert).
  2. The backend AI Worker processes the event, drafts a proposed action (e.g., "Reorder supplies from vendor"), and inserts it into the `agent_feed_items` table.
  3. The owner logs into the mobile UI (375px) and sees the "Action Required" card in their Unified Agent Feed.
  4. The owner taps "Approve".
  5. The backend Action Router intercepts the approval, executes the state change (or simulated external call), and updates the feed card to "Approved".

  **Acceptance Criteria:**
  *   **Backend:** Ensure the `AgentFeedService` correctly handles incoming events and generates structured `proposed_action` JSON. Implement the `ActionRouter` (e.g., `dispatch_action`) in `update_feed_item_state` to actually execute the approved payloads.
  *   **Frontend:** The Agent Feed must render correctly on a 375px viewport (no horizontal scrolling). Touch targets for Approve/Edit/Dismiss must be >= 44x44px.
  *   **Testing:** Write a Playwright E2E test (`unified_agent_feed_proactive.spec.ts`) that simulates an event, verifies the card appears in the feed, approves it, and verifies the final state. 100% unit test coverage on new Rust modules.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
