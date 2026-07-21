issue_title: "Implement Agent Feed Core Architecture and Mobile UI"
issue_description: |
  # Implement Agent Feed Core Architecture and Mobile UI

  ## Problem Statement
  Small business owners are overwhelmed by managing fragmented operations, customer inquiries (Instagram, WhatsApp, Email), and business insights across disjointed tools. Traditional dashboards require owners to actively seek out information and perform manual actions. They need a proactive, mobile-first unified inbox and action center—an "Agent Feed"—that pushes drafted communications and operational insights for 1-tap approval, reducing cognitive load and saving hours of manual work.

  ## Research Report
  - **Market Landscape:** Legacy tools like Shopify offer reactive AI chatbots or complex app ecosystems that require significant technical setup. Tools like Wix have generic inboxes that don't proactively draft contextual responses.
  - **The Gap:** There is a strong need for "Invisible AI Automation." SMB owners need "staff," not just tools. Our target is the upper-right quadrant: highly proactive autonomous agents combined with radical simplicity.
  - **Key Use Cases:** Instagram DM overload (The Ambassador agent), inventory/marketing alignment (The Promoter agent), and business advisory (The Advisor agent).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Event Sources: Webhooks, CRON, DB Triggers] --> B(Event Ingestion Pipeline)
      B --> C{Intent & Context Resolution}
      C -->|Query| D[Unified Customer Graph & Catalog DB]
      C --> E[LLM Layer: Ambassador, Promoter, Advisor]
      E -->|Draft Proposal| F[agent_feed_items Table]
      F --> G[OHC Mobile App Feed 375px]
      G -->|User Taps Approve| H[Action Dispatcher]
      H --> I[External APIs / DB Updates]
  ```

  ### Mobile UX Flow & UI Wireframes (375px)
  - **Feed View:** The primary home screen is a vertical feed of "Action Cards". No horizontal scrolling.
  - **Action Card Layout:**
    - Top: Context header (e.g., "Message from Maya (Instagram)").
    - Middle: Proactive AI draft ("Hi Maya, yes we have vegan cakes...").
    - Bottom: Sticky CTA bar with prominent "Approve & Send", and secondary "Edit" / "Discard" buttons.
  - **Visual Design:** macOS-style Translucent Glass materials, readable typography, clear status tokens.

  ### AI Agent Integration Points
  - **The Ambassador:** Triggered by incoming messages. Uses RAG against `interaction_events` and inventory data to draft replies into `agent_feed_items`.
  - **The Promoter:** Scheduled CRON jobs analyzing inventory to draft social media posts into the feed.
  - **The Advisor:** Analyzes weekly PostgreSQL metrics to draft plain-language business insights and proposed actions (e.g., promotional discounts).

  ### Key Design Decisions
  - **Push vs. Pull:** Shift from a dashboard model to an inbox model. The user reviews and approves rather than initiates.
  - **Multi-Tenant Security:** Strict row-level security on `agent_feed_items` using `tenant_id`.
  - **Resilience:** Use of PostgreSQL `SKIP LOCKED` for processing asynchronous agent actions to guarantee delivery and avoid missed notifications.

  ## Implementation Prompt
  **Goal:** Build the full-stack foundation for the Agent Feed, enabling the "Ambassador" agent to populate the feed and the user to approve the action.

  **Critical User Journey (CUJ):**
  1. A background event (e.g., simulated webhook) triggers the Ambassador Agent.
  2. The agent queries mock business data and drafts a reply, creating an entry in the `agent_feed_items` table with `lifecycle_state` = 'pending_approval'.
  3. A non-technical owner logs into the OHC mobile app (375px view).
  4. The owner sees the new "Action Card" in their home feed.
  5. The owner taps "Approve & Send".
  6. The `lifecycle_state` updates to 'approved' and the system simulates sending the message.

  **Acceptance Criteria:**
  - Create the API endpoints to fetch and update `agent_feed_items`.
  - Build the Flutter/Tauri/Next.js frontend Feed screen adhering strictly to the 375px mobile-first Translucent Glass design.
  - Ensure zero mock data in the UI (data must come from the backend).
  - Include full Playwright E2E tests verifying the 1-tap approval flow on mobile viewports.
  - Maintain 100% unit test coverage for new backend logic.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
