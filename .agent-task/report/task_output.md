issue_title: "Implement Agentic Work Triage Feed & Mobile Action Center"
issue_description: |
  ## Title
  Implement Agentic Work Triage Feed & Mobile Action Center

  ## Problem Statement
  Business owners like Maya (Home Baker) and Carlos (Field Service Owner) suffer from "dashboard fatigue" and fragmented workflows. Current platforms like Shopify or Wix provide passive dashboards that require the owner to hunt for what needs attention—checking separate tabs for orders, messages, inventory alerts, and abandoned carts. Maya doesn't want to read a dashboard; she wants to be told exactly what needs her attention right now (e.g., "3 new custom cake inquiries," "2 upcoming deliveries"). Without a unified, proactive triage feed, owners miss leads, forget follow-ups, and spend hours context-switching on their mobile phones instead of executing work.

  ## Research Report
  - **Competitive Analysis:**
    - **Shopify (Sidekick):** Reactive. The user must ask Sidekick a question to get an action or summary. It does not natively push an actionable feed to the merchant's lock screen or home page.
    - **Wix / Squarespace:** Provide basic notification bells, but lack intelligent aggregation. They notify about an event (e.g., "New form submission") but do not draft the reply or propose the next business action.
    - **HubSpot (Breeze):** Closer to the ideal, grouping tasks and drafted emails, but it is built for B2B sales teams, not a solopreneur on a 375px mobile device.
  - **Findings:** 73% of non-technical small business owners abandon complex setups and ignore analytics dashboards. They rely on "Inbox Zero" paradigms. OHC must transform the traditional dashboard into an "Inbox of Work"—a prioritized feed where AI agents surface drafted replies, pending approvals, and urgent tasks.
  - **Codebase Context:** The underlying data schema (`agent_feed_items` in `031_agent_feed.sql`) already exists, storing `event_source`, `context_payload`, and `proposed_action`. However, the end-to-end AI pipeline to populate this feed intelligently, and the mobile-first UI to consume and approve these actions, are missing.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Ingestion
          A[Incoming DM/Webhook] --> B(Event Router);
          C[Order State Change] --> B;
          D[Inventory Low] --> B;
      end

      subgraph AI Orchestration Layer
          B --> E{Work Triage Agent};
          E -->|Query Context| F[(PostgreSQL: Tenants & Orders)];
          E -->|Draft Action| G[Agent Feed Engine];
          G --> H[(DB: agent_feed_items)];
      end

      subgraph Presentation & Execution
          H --> I[OHC Mobile App Feed 375px];
          I -->|Owner Taps Approve| J[Action Execution Pipeline];
          J --> K[External System / DB Mutation];
      end
  ```

  ### UI Wireframes & Screen Flow (375px First)
  - **The Work Feed (Home Screen):** A single vertical scroll of clean, Apple/Ubiquiti-style translucent cards.
  - **Card Anatomy:**
    - **Header:** Icon + Event Type (e.g., 💬 Message, 📦 Inventory, 🗓️ Booking).
    - **Context Body:** "Carlos, a new lead from Instagram asked about gutter cleaning."
    - **Proposed Action (Glassmorphism inset):** The drafted reply or proposed action (e.g., "Drafted: Hi! I can come by tomorrow at 2 PM. It's usually $150. Should I book it?").
    - **Actions:** Large (44x44px min touch target) buttons: `[Approve & Send]`, `[Edit]`, `[Dismiss]`.

  ### Mobile UX Flow
  1. User opens the OHC app. Instead of charts, they see the "Today's Triage" feed.
  2. First card: Drafted reply to an Instagram DM. User taps "Approve." The card smoothly animates away (Zero WIP philosophy).
  3. Second card: Low inventory alert for a physical product with a drafted supplier re-order email. User taps "Edit," modifies the quantity, and confirms.

  ### AI Agent Integration Points
  - **Work Triage Agent (Gemini Pro):** Listens to the internal message bus. Evaluates the urgency of the event, retrieves relevant tenant context (e.g., past interaction history, current inventory), and populates the `proposed_action` JSON field.
  - **Execution Handoff:** When the user approves an item, the action payload is passed back to the specific departmental agent (e.g., Operations Agent for inventory, Customer Success Agent for DMs) to execute the mutation.

  ### Key Design Decisions
  - **Feed over Dashboard:** Dashboards are for reading; feeds are for doing. We optimize for immediate action.
  - **Approval-Driven Autonomy:** The AI drafts and prepares, but the Owner maintains final control via one-tap approvals, building trust in the system.
  - **Unified Event Schema:** By centralizing all tasks into `agent_feed_items`, we ensure the UI frontend only needs to render one type of component (the Action Card), drastically simplifying the Flutter/Web implementation.

  ## Implementation Prompt
  **User-Facing Outcome:** The owner logs into OHC and sees a clean, prioritized feed of tasks and drafted actions, allowing them to clear their daily operational backlog in minutes using one-tap approvals.

  **CUJ (Critical User Journey):**
  1. Log into the OHC web/mobile shell.
  2. The Home route displays the `Agent Feed`.
  3. An incoming customer inquiry triggers a backend event.
  4. The UI dynamically updates to show a new Action Card with a drafted response.
  5. The owner clicks "Approve," which mutates the feed item's state to 'executed' and triggers the backend to send the response.

  **Acceptance Criteria:**
  - Build the backend service to populate `agent_feed_items` based on generic events.
  - Implement a mobile-first (375px) Feed UI in the frontend using the OHC premium translucent design tokens.
  - Ensure interactive buttons (Approve, Edit, Dismiss) have functional state transitions and error handling.
  - Write Playwright E2E tests simulating the owner reviewing and approving an action card from the feed, ensuring the database reflects the 'executed' state.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
