issue_title: "Agent Feed Deep Dive & Action Card Implementation"
issue_description: |
  # Research Report: Agent Feed & Actionable Work Command Center

  ## 1. Problem Statement
  The OHC Promise is that an owner can open OHC and "immediately know what needs attention today." However, currently, the experience requires the owner to seek out information manually across different dashboards and screens. Small business owners like Maya (the baker) and Carlos (the handyman) don't have the time to hunt for tasks, review isolated logs, or click through deep menus to figure out what their AI agents have drafted or what operations need their approval. The core gap is a proactive, unified "Agent Feed" that pushes drafted communications, urgent tasks, and operational insights directly to the user as actionable cards.

  ## 2. Research Report
  - **Market Landscape:** Products like Tencent Workbuddy, Shopify Sidekick, and HubSpot rely on either passive dashboards or chat-based assistant interfaces. While chat is great for deep inquiries, it's inefficient for rapid, daily triage. Owners need a prioritized feed of actions (similar to a highly intelligent inbox or social feed) rather than a blank chat prompt or a complex analytics dashboard.
  - **OHC Architecture Readiness:** OHC already possesses the `ohc_job_queue` and universal ledger for background task coordination. The `Dynamic Workflows` system can orchestrate complex agent tasks. However, the *output* of these background operations (e.g., an LLM drafting a reply to a customer DM) is disconnected from a unified UI feed where the owner can simply tap "Approve" or "Edit."
  - **The Missing Link:** An "Agent Feed" architecture where the results of background AI work are materialized as actionable cards (e.g., "Draft Reply to Customer", "Low Stock Alert: Restock Needed", "Payment Received: Schedule Delivery") that the owner can triage in seconds.

  ## 3. Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      Events[External & Internal Events\ne.g., Instagram DM, Low Stock] --> Queue[OHC Job Queue]
      Queue --> Agent[AI Agent Workers\n(Intent Resolution, Drafting)]
      Agent --> DB[Agent Feed Table\n(Action Cards)]
      DB --> API[REST/gRPC API]
      API --> UI[Flutter/PWA Mobile App]
      UI -->|User taps Approve/Reject| API
      API --> Queue2[Execution Queue]
      Queue2 --> Action[Final Action\ne.g., Send Message, Order Stock]
  ```

  ### Mobile UX Flow (375px First)
  1. **Home Screen (The Feed):** The primary view is a vertically scrolling list of Action Cards. No complex navigation needed initially.
  2. **Action Card Design:**
     - **Context:** "Instagram DM from @cake_lover"
     - **Draft:** "Hi! Yes, we can make a vegan chocolate cake for Saturday. It will be $50. Would you like a payment link?"
     - **Actions:** Large, thumb-friendly buttons (≥ 44x44px) for "Approve & Send", "Edit Draft", "Discard".
  3. **Triage Interaction:** Tapping an action immediately executes it (optimistic UI update) and removes the card from the feed, keeping the inbox "clean."

  ### Key Design Decisions
  - **Push, Not Pull:** Information comes to the user formatted as a decision, not raw data.
  - **Unified Stream:** Combines operations, customer service, and finance into one timeline.
  - **Immutable Audit:** Once an action is taken on a feed item, it is logged to the `ohc_universal_ledger` for historical tracking.

  ## 4. Implementation Prompt
  **Target User:** Maya (Home Baker) & Carlos (Field Service Owner)
  **CUJ (Critical User Journey):**
  1. Maya opens the OHC app in the morning.
  2. The home screen displays her "Agent Feed".
  3. The top card says: "Drafted Reply: Customer asked about vegan options. Draft: 'Yes, we have vegan cakes!'"
  4. Maya taps the "Approve & Send" button.
  5. The card instantly dismisses (optimistic UI), and the backend sends the message via the connected channel.
  6. The next card says: "Pending Booking: Carlos requested a roof repair estimate. Draft Quote attached."

  **Acceptance Criteria:**
  1. Define a robust Postgres schema for `agent_feed_items` (tenant isolated) that stores the card content, associated draft data, and status (pending, approved, rejected).
  2. Implement backend API endpoints to fetch the feed, approve an item, and reject an item.
  3. Create a mobile-first (375px) UI component for the "Action Card" using the macOS Translucent Glass and UniFi modular dashboard design tokens.
  4. Integrate the UI with the backend to display the feed and handle approve/reject actions with optimistic UI updates.
  5. Ensure 100% unit test coverage for new backend logic and at least 3 Playwright E2E tests covering the feed triage CUJ.

  ## 5. Priority & Scope
  - **Priority:** P0 (Core to the OHC Promise)
  - **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
