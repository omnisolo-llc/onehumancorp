issue_title: "Architectural Design: Unified Agent Action Feed & Event-Driven Action Cards"
issue_description: |
  ## Mission Queue Protocol

  **Problem Statement**:
  Business owners (like Maya the baker and Carlos the handyman) are overwhelmed by complex dashboards and isolated notification streams. Legacy platforms require manual navigation to multiple sections (Orders, Inbox, Marketing) to piece together necessary daily actions. OHC must guide users from unclear work to clear next action in minutes via an "Agent Action Feed"—a prioritized stream of actionable cards where AI agents propose drafts, coordinate tasks, and await 1-tap owner approval.

  **Research Report**:
  Competitive analysis against Shopify, Wix, and standard Link-in-Bio tools reveals a critical gap in mobile-first operational management. While Shopify provides excellent order viewing, acting on complex operations (e.g., launching a discount, replying to customer inquiries across channels) requires falling back to desktop.
  Our research in `ohc_smb_mobile_first_design_research.md` and `agent_feed_deep_dive.md` confirms that OHC's primary differentiator is the "Approval Interface Paradigm." Instead of finding tasks, AI agents push context-rich cards (e.g., drafted Instagram DM replies, inventory restocking alerts, generated marketing emails) directly to a unified feed.

  **Design Doc**:
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Event Mesh / MsgBus] -->|Tenant Events| B(Agent Dispatcher)
      B --> C[The Ambassador Agent]
      B --> D[The Operations Agent]
      C -->|Drafts Reply| E[Action Required Queue]
      D -->|Drafts Restock| E
      E --> F[Unified Feed Service]
      F --> G[Mobile Client 375px]
      G -->|User Taps Approve| H[Action Executor]
      H --> I[External Integrations]
      H --> J[DB State Update]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Screen (375px):** The default landing page is the "Agent Feed", a single-column scrolling list of `Action Cards`.
  - **Action Card Structure:**
    - **Header:** Agent Identity (e.g., Operations, Marketing) & Urgency.
    - **Context Area:** Summarizes the trigger (e.g., "3 new orders pending fulfillment" or "Sarah asked about vegan cakes").
    - **Proposed Action Area:** Shows the AI-generated artifact (e.g., a drafted email, an invoice preview).
    - **Action Bar:** Large touch targets (min 44x44px). Primary action is "Approve & Send" or "Execute". Secondary action is "Edit" or "Dismiss".
  - **Visual Language:** OHC Premium Tokens. Cards feature Translucent Glassmorphism, blurred background layers, and distinct hierarchical typography to allow quick reading on the go.

  ### AI Agent Integration Points
  - Agents subscribe to business events (Webhooks, Orders, DMs).
  - The LLM provider (Gemini/MiniMax) synthesizes context and generates structured JSON representing the `ActionCard`.
  - Action proposals are stored in PostgreSQL with state `PENDING_APPROVAL`.
  - Upon user interaction, the `Action Executor` processes the user's decision, calling appropriate domain services (e.g., Stripe for invoices, SendGrid for emails).

  ### Key Design Decisions
  - **Pull vs. Push:** Move from a pull-based dashboard (user looks for work) to a push-based feed (system tells user what to do).
  - **State Machine:** Action cards must have strict state transitions (`PENDING`, `APPROVED`, `REJECTED`, `EDITED`, `EXPIRED`).
  - **Zero-Trust & Multi-Tenancy:** The action feed API must strictly filter by `tenant_id` and leverage row-level security.

  **Implementation Prompt**:
  **Objective:** Implement the backend domain service, PostgreSQL schema, and mobile UI components for the Unified Agent Action Feed.
  **User-Facing Outcome:** When Maya opens her app, she sees a feed of cards. The top card says "Drafted reply to Instagram DM from John: 'Yes, we have vegan options!'". She taps "Approve" and the card visually resolves and disappears.
  **CUJ & Acceptance Criteria:**
  1. Create the `ActionCard` data models and PostgreSQL migrations with RLS.
  2. Implement the `UnifiedFeedService` API (gRPC/REST) to list and update action cards for a tenant.
  3. Build the Flutter/Tauri mobile UI component for the Action Card following the 375px Translucent Glassmorphism design tokens.
  4. Write Playwright E2E tests: Simulate an action card being created in the DB, log in as a test user, verify the card is visible on the mobile feed layout, tap "Approve", and verify the card's state transitions to approved via the API.

  **Priority**: P0
  **Estimated Scope**: Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
