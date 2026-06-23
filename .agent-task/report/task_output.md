issue_title: "[research] Architect Real-Time Unified Mobile Agent Feed"
issue_description: |
  # Research Report: Real-Time Unified Mobile Agent Feed

  ## 1. Problem Statement
  Legacy business platforms force non-technical owners to navigate disjointed, desktop-oriented dashboards containing raw data points and complex settings (e.g., Shopify, Wix, WooCommerce). For mobile-first owners (like Fatima, the food cart operator, or Maya, the home baker), performing critical business tasks—such as reviewing generated social posts, accepting draft emails, recovering abandoned bookings, or observing anomaly alerts—on a 375px mobile device is confusing, cumbersome, and technically overwhelming.

  Owners need a streamlined "work assistant" UI. Instead of hunting through different screens to find out "what to do next," OHC must push all actionable AI-generated insights, drafts, and system alerts into a single, vertically scrolling, unified Agent Feed.

  ## 2. Research Report
  - **Competitive Analysis:**
    - *Shopify Sidekick / Wix ADI:* Mostly conversational chatbot interfaces that react to user queries. Not proactively pushing complex actionable cards to a unified feed.
    - *Lindy.ai / 11x.ai:* Excellent at autonomous execution but geared heavily toward enterprise/B2B (sales outreach, executive assistants) and typically operate headless or via email/Slack.
    - *Link-in-Bio (Linktree, Stan Store):* Simple but lack underlying commerce & operations logic (no inventory sync, no agentic draft reviews).
  - **Gap to Close:** OHC currently lacks the core feed architecture to unify events from the Operations Agent, Customer Success Agent (The Ambassador), and Marketing Agent (The Promoter) into a singular mobile-first "Approval" UI. This gap prevents the realization of the "Approval Interface Paradigm."

  ## 3. Design Doc (High-Level Architecture)
  ### 3.1 UX & Mobile Flow (375px)
  - **The Feed View:** The default view upon opening the app. A vertically scrolling list of "Action Cards" (e.g., "Drafted Reply for Maya's Custom Order").
  - **Action Cards:** Each card is rendered with macOS-style Translucent Glass aesthetics (`backdrop-filter: blur`, rounded corners). Touch targets > 44px.
  - **Interactions:** Cards contain contextual data (e.g., the draft text) and 1-tap actions: "Approve & Send", "Edit", "Discard".
  - **Real-Time Updates:** The feed updates in real-time as background agents generate new tasks or insights.

  ### 3.2 Architecture Components
  - **Agent Feed Service (Rust):** Central gRPC/REST service managing the timeline of actionable events for a specific tenant.
  - **Event Pub/Sub:** Backend agents (e.g., Customer Success, Marketing) publish "Proposals" (drafts, alerts, decisions) to an event bus (e.g., Redis Pub/Sub).
  - **Feed Storage (PostgreSQL):** Action cards are persisted in a `feed_items` table with tenant isolation, status (pending, approved, dismissed), and JSON payload containing the specific UI card structure.
  - **Real-time Push:** Server-Sent Events (SSE) or WebSockets to push new feed items instantly to the Flutter/Tauri mobile client.

  ### 3.3 System Flow Diagram
  ```mermaid
  sequenceDiagram
      participant App as Mobile App (375px)
      participant API as Agent Feed API
      participant DB as PostgreSQL (feed_items)
      participant Agent as Background Agents (e.g., Promoter)

      Agent->>API: Publish Proposal (e.g., "Draft Social Post")
      API->>DB: Store Feed Item (status: pending)
      API-->>App: Push Real-Time Event (SSE)
      App->>User: Display Action Card
      User->>App: Tap "Approve"
      App->>API: Submit Approval
      API->>DB: Update status to "approved"
      API->>Agent: Execute Action
  ```

  ## 4. Implementation Prompt
  **Mission:** Implement the backend "Agent Feed Service" and the mobile-first "Agent Feed" UI component.

  **Backend Outcomes (Rust/PostgreSQL):**
  - Create the `feed_items` database schema (with strict tenant isolation).
  - Implement an internal API for background agents to publish actionable items (with JSON payloads for rendering context).
  - Implement an external API for the client to fetch pending feed items and submit approvals/dismissals.

  **Frontend Outcomes (Mobile-First / 375px):**
  - Build the Unified Feed view, ensuring no horizontal scroll on a 375px device.
  - Design generic "Action Cards" utilizing the OHC Premium Glassmorphism tokens (translucent backgrounds, 16px border radii).
  - Ensure all CTAs (Approve, Edit) meet the 44x44px minimum touch target.
  - Provide a truthful empty state ("You're all caught up!") and smooth transitions when cards are approved.

  **Acceptance Criteria:**
  - E2E Playwright test verifies a background agent pushing a task, the task appearing in the feed, and the user successfully tapping "Approve".
  - UI is visually perfect on a 375px viewport simulation.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
