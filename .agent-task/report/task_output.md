issue_title: "Architectural Design: Unified Agent Feed for Mobile Operations"
issue_description: |
  ## Problem Statement
  Small business owners are overwhelmed by scattered tools and notifications. They lack a single place that tells them what requires their attention right now. Traditional admin dashboards are desktop-oriented, data-heavy, and reactive. They force owners to hunt for information across tabs. Owners need a mobile-first, proactive "work feed" where AI agents coordinate background tasks and present unified, actionable cards (e.g., "Approve drafted reply to Instagram DM," "Restock flour," "Approve 10% discount campaign") that can be executed with a single tap on a 375px screen.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify & Wix:** Companion mobile apps are primarily for viewing stats and fulfilling orders. Complex operations require a desktop. They lack a proactive, unified feed of agent-driven tasks.
  - **Intercom Fin / Zendesk:** Excellent AI resolution engines, but focused solely on customer support, not general business operations.
  - **Link-in-bio tools (Linktree, Stan Store):** Perfect mobile execution but lack depth.
  - **OHC Opportunity:** The "Unified Agent Feed" is the differentiator. It shifts the paradigm from "Dashboard" to "Inbox." Agents do the work (drafting, coordinating) and present the owner with simple "Approve/Discard/Edit" cards in a vertical feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Event Bus - Kafka/Redis] -->|Webhooks, Orders, DMs| B(Agent Dispatcher)
      B --> C[Operations Agent]
      B --> D[Customer Success Agent]
      B --> E[Finance Agent]
      C -->|Action Proposal| F[Unified Feed DB]
      D -->|Action Proposal| F
      E -->|Action Proposal| F
      F --> G[Mobile App - 375px Feed UI]
      G -->|1-Tap Approve| H[Action Executor API]
  ```

  ### Mobile UX Flow (375px First)
  - **Home Screen:** The primary view is a vertical, chronological feed of "Cards" (similar to a social feed or modern email inbox).
  - **Card Anatomy:**
    - **Header:** Agent Identity (e.g., "Operations Agent") and Priority Label.
    - **Context:** Brief summary ("Inventory for 'Vegan Cake' is below 5 units").
    - **Proposed Action:** Clear description of what the agent wants to do ("Draft purchase order to Supplier X").
    - **Actions:** Large, thumb-friendly buttons (44x44px minimum target): "Approve", "Edit", "Dismiss".
  - **Interaction:** Tapping "Approve" executes the action and removes the card with a satisfying animation. Tapping "Edit" opens a focused modal.

  ### AI Agent Integration Points
  - **Agent Identity:** Each department's agent (Operations, CS, Finance) must identify itself clearly on the card to build trust.
  - **Context Window:** The agent must include a concise "Why this matters" snippet to help the owner make a quick decision.
  - **Approval Handoff:** The Agent Feed is the "Human-in-the-Loop" gate. Once approved, the executor API hands control back to the specific agent to finalize the task.

  ### Key Design Decisions
  - **Push vs. Pull:** The user does not "go to the inventory page" to check stock. The Operations agent pushes a card to the feed when action is needed.
  - **Glassmorphism UI:** Cards should use translucent materials over a subtle blurred background to indicate depth and hierarchy, feeling premium (Apple-esque).

  ## Implementation Prompt
  **User-Facing Outcome:** When Maya opens the OHC app, her home screen is a feed of actionable items: one drafted reply to a customer DM, one alert that she needs to restock flour, and a summary of yesterday's sales. She taps "Approve" on the DM and "Order" on the flour, clearing her immediate tasks in 10 seconds.

  **Critical User Journey (CUJ):**
  1. Owner logs into the OHC mobile app.
  2. The home screen displays a "Unified Agent Feed".
  3. The feed contains at least one real Action Card from an Agent that's persisted in the backend. There should be ZERO mocked data.
  4. Owner taps "Approve".
  5. The card visually resolves/dismisses, and the action is recorded in the backend.

  **Estimated Scope:** Large

  **Acceptance Criteria:**
  - Define the `FeedItem` data model (handling state, payload, and agent source).
  - Create the Feed UI component ensuring it fits perfectly on a 375px viewport with no horizontal scrolling.
  - Implement the "Approve/Dismiss" interaction logic.
  - Ensure 100% unit test coverage for the feed logic and Playwright E2E coverage for the card interaction CUJ.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
