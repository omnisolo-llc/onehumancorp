issue_title: "Unified Agent Feed Mobile App Architecture"
issue_description: |
  **Problem Statement**
  Business owners on OHC currently lack a centralized, mobile-first unified view of proactive AI agent actions. Agent actions (e.g., Marketing drafting an email, Customer Success resolving an inquiry, Operations generating an invoice) are either siloed in specific dashboards or sent as disjointed notifications. This forces the owner to manually piece together what needs their attention, violating the core principle of "unclear work -> clear next action in minutes". Without a unified feed, owners experience notification fatigue and lose the benefit of coordinated, invisible AI automation. The lack of a seamless mobile approval flow forces them back to desktop interfaces to manage agent proposals.

  **Research Report**
  Market analysis (Shopify, Wix, Linktree) highlights a significant gap: legacy platforms provide complex mobile dashboards optimized for viewing metrics, not executing operations. Link-in-bio tools are too simple, lacking robust backend integration. OHC's unique value proposition is the "Agentic Work Assistant."
  The `Agent Feed` is meant to be the central nervous system for OHC users. Our current database architecture utilizes an `agent_approvals` table and an `agent_feed_items` table. However, the mobile frontend is fractured. We need to implement a "Unified Agent Feed" mobile application architecture that aggregates these into actionable "Cards".

  **Design Doc**

  *Architecture Diagram*
  ```mermaid
  sequenceDiagram
      participant Owner (Mobile UI)
      participant API Gateway
      participant Agent Feed Service
      participant Multi-Tenant DB
      participant AI Agents (Departments)

      AI Agents (Departments)->>Multi-Tenant DB: Insert/Update Action Proposal (agent_feed_items)
      Owner (Mobile UI)->>API Gateway: GET /api/v1/agent-feed (with tenant_id)
      API Gateway->>Agent Feed Service: Fetch Pending Actions
      Agent Feed Service->>Multi-Tenant DB: Query pending feed items
      Multi-Tenant DB-->>Agent Feed Service: Return items
      Agent Feed Service-->>API Gateway: Format as Feed Cards
      API Gateway-->>Owner (Mobile UI): Display Unified Feed
      Owner (Mobile UI)->>API Gateway: POST /api/v1/agent-feed/{id}/approve
      API Gateway->>Agent Feed Service: Approve Action
      Agent Feed Service->>Multi-Tenant DB: Update state to 'APPROVED'
      Agent Feed Service->>AI Agents (Departments): Trigger Execution (Pub/Sub)
  ```

  *Mobile UX Flow (375px First)*
  1.  **Dashboard Entry:** The primary screen after login is the "Unified Agent Feed," replacing static metrics.
  2.  **Card Layout:** Each pending action is a card. Cards use glassmorphism, have a clear title (e.g., "Drafted Instagram Reply"), context snippet, and an explicit action button (e.g., "Approve & Send"). Touch targets are minimum 44x44px.
  3.  **Action Flow:** Tapping "Approve" triggers an optimistic UI update (card slides away, success toast appears) while the backend processes the state change.
  4.  **Offline Support:** Optimistic UI state and queued approvals if offline, syncing when connectivity restores.

  *AI Agent Integration Points*
  All agents (Marketing, Operations, Advisory) must route high-confidence proposals and required interventions through the `agent_feed_items` database table, standardizing the payload structure (`event_source`, `context_payload`, `proposed_action`, `lifecycle_state`). The feed acts as the single point of human-in-the-loop (HITL) approval.

  **Implementation Prompt**
  Build the backend API and Mobile-First (Flutter/Tauri) UI for the "Unified Agent Feed".
  *   **CUJ**: An owner logs into the mobile app, sees three pending agent actions (e.g., an SEO optimization proposal, a drafted customer reply, and an inventory alert). They review the details of the customer reply, tap "Approve", and the action is seamlessly executed.
  *   **Acceptance Criteria**:
      1. Implement a REST API endpoint to aggregate `agent_feed_items` and `agent_approvals` for a specific tenant.
      2. Build a mobile-optimized (375px viewport) feed UI using OHC Premium Tokens (Glassmorphism).
      3. Implement 1-tap approval/dismissal logic with optimistic UI updates.
      4. Ensure all interactive elements have >= 44x44px touch targets.
      5. Include robust E2E Playwright tests simulating the owner reviewing and approving an item in the feed.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
