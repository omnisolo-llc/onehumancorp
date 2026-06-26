issue_title: "Unified Mobile-First Owner Action Feed Architecture & Agent Dispatcher"
issue_description: |
  ### Title
  Unified Mobile-First Owner Action Feed Architecture & Agent Dispatcher

  ### Problem Statement
  SMB owners like Maya (baker) and Carlos (handyman) are overwhelmed by incoming signals: messages on Instagram, new bookings, deposit payments, low inventory alerts, and tasks. Currently, they have to navigate multiple menus and tabs to see what needs attention. OHC needs a single, unified "Work Triage" feed that presents these signals as actionable, prioritized cards on a 375px mobile screen. This feed should not just list notifications; it should present agent-drafted actions (e.g., a drafted reply to a DM, a ready-to-send quote) that the owner can approve with one tap.

  ### Research Report
  **Findings & Competitive Analysis:**
  - **Shopify/Wix:** Rely on a traditional dashboard. Users have to hunt for information (Orders tab, Messages tab, Analytics tab).
  - **Notion AI/Microsoft Copilot:** Powerful but require the user to formulate prompts and initiate actions.
  - **OHC Opportunity:** The "Work Triage" Feed. The OHC Assistant acts as the aggregator and prioritizer. It uses the `Work Triage` capability to combine system events (bookings, payments) with communication events (DMs, emails) into a single stream. Crucially, each item in the feed must be paired with an *Actionable Recommendation* powered by specific agents (e.g., Customer Success Agent drafting a reply, Sales Agent drafting a quote).

  ### Design Doc

  **Architecture Diagram**
  ```mermaid
  graph TD
      A[Event Sources: DMs, Bookings, System Alerts] -->|Event| B(Unified Event Bus)
      B --> C[Work Triage Engine]
      C -->|Classify & Prioritize| D{Agent Dispatcher}
      D -->|Context Needed| E[Memory Consolidation / Vector DB]
      D -->|Customer Reply| F[Customer Success Agent]
      D -->|Quote/Booking| G[Sales & Ops Agent]
      F -->|Draft Action| H[Action Card Generator]
      G -->|Draft Action| H
      H --> I[Mobile-First Owner Feed]
      I -->|1-Tap Approve| J[Execution Engine]
      J --> K[External APIs / DB Updates]
  ```

  **Mobile UX Flow (375px viewport):**
  1. **Home Screen (The Feed):** The owner opens the app and sees a vertical, paginated list of Glassmorphism styled Action Cards (using OHC Design Tokens: translucent materials, clear typography).
  2. **Action Card Anatomy:**
      - **Header:** Priority Indicator (e.g., Urgent, Today, FYI) + Event Type (e.g., "New Instagram DM").
      - **Context:** Snippet of the trigger (e.g., "Customer asked about vegan cakes").
      - **Agent Recommendation:** The drafted action (e.g., Drafted reply: "Yes, we have vegan cakes! Here is the link...").
      - **Action Bar:** Prominent "Approve & Send" button (primary action, 44x44px minimum touch target), "Edit" (secondary), "Dismiss" (tertiary).
  3. **Execution:** Tapping "Approve" transitions the card to a reassuring "Done" state and removes it from the active feed, executing the agent's intent in the background.

  **AI Agent Integration Points:**
  - **Triage Agent (Gemini Pro/MiniMax):** Subscribes to the Unified Event Bus, classifies the priority of incoming events, and determines which specialized agent should handle it.
  - **Department Agents:** Receive the delegated task from the Triage Agent, query the vector DB for context (e.g., user policies, past interactions), and return a structured JSON response representing the drafted action.

  **Key Design Decisions:**
  - **Push vs. Pull:** Shift from a pull-based dashboard to a push-based feed.
  - **Agent Handoff:** The Triage engine must reliably delegate to sub-agents using the KAIROS Sub-Agent Queue to avoid blocking the main event loop.
  - **Data Isolation:** All events and agent contexts must strictly enforce the `tenant_id` at the database and memory retrieval levels.

  ### Implementation Prompt
  **Role:** Implementer Agent
  **Task:** Build the core backend models, API endpoints, and initial Flutter/Tauri UI components for the Unified Mobile-First Owner Action Feed.
  **Outcome:** A business owner can log into the OHC app, view a unified feed of synthetic `ActionCards`, and click an "Approve" button that triggers a state change.
  **Acceptance Criteria:**
  1. **Backend:** Implement a PostgreSQL table `action_feed_items` with tenant isolation (`tenant_id`). Create an Axum REST/gRPC endpoint to fetch the feed for a tenant, paginated and sorted by priority.
  2. **Event Hooks:** Create a basic service layer interface `FeedService::publish_action` that other modules can call to inject items into the feed.
  3. **Frontend (Mobile-First):** Build the UI for the Action Feed in the Tauri/Flutter app, ensuring perfect layout on a 375px viewport. Implement the glassmorphism card design specified in the design doc.
  4. **Interactivity:** The "Approve" button on a card must hit a backend endpoint to mark the item as resolved, smoothly animating its removal from the UI list.
  5. **Testing:** 100% unit test coverage on the backend service. A Playwright E2E test that logs in, navigates to the feed, and successfully approves a seeded action card.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
