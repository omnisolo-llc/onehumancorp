issue_title: "Refactor Unified Agent Feed Data Model and UI"
issue_description: |
  # Research Report: The Unified Omnichannel Agent Feed Data Model Disconnect

  ## Problem Statement
  Currently, the OHC platform uses two separate database tables to power the "Unified Agent Feed" on the mobile dashboard: `agent_approvals` (a legacy table primarily populated by internal workflows) and `agent_feed_items` (the newer, more robust table designed for omnichannel messaging like Instagram DMs, quoting, and proactive actions).

  This split architecture causes significant friction:
  1. The frontend (`src/ui/tauri/src/ui/dashboard.html`) has complex, brittle parsing logic (`if (item.action_type === 'SocialPostDraft')` vs `if (item.event_source === 'instagram_dm')`) to handle the different data shapes.
  2. Approving an item requires the backend to conditionally update either `agent_approvals` or `agent_feed_items`, leading to bugs where an Instagram DM might not clear correctly from the queue after approval.
  3. Non-technical owners (like Maya the Baker) experience "ghost notifications" because the eventual consistency between the legacy table and the new feed table fails under load.

  ## Research Report
  - **The "Agent Feed Deep Dive" Document:** Outlines a vision where the Agent Feed is the central nervous system, pushing Action Cards to the owner. This requires a unified, predictable data schema.
  - **Code Audit:** `src/server/domain/repository/agent_feed_repo.rs` explicitly performs a `UNION ALL` across `agent_feed_items` and `agent_approvals` to serve the feed API. However, the `agent_approvals` table uses `status` (DRAFT, PAUSED) while `agent_feed_items` uses `lifecycle_state` (PENDING_APPROVAL).
  - **UI Complexity:** In `src/ui/tauri/src/ui/dashboard.html` around line 1400, the `renderTriageItems` function has massive `if/else` blocks to accommodate the two table structures.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Agent Departments] -->|Publish| B(Agent Feed Repo)
      C[Omnichannel Webhooks] -->|Publish| B
      B --> D[(agent_feed_items Table)]
      D --> E[Mobile Dashboard /feed]
  ```

  ### UI Wireframes & Screen Flow (375px)
  1. **Dashboard Home (375px):** A unified, single-column feed list using macOS-style Translucent Glass materials. Each card represents an `AgentFeedItem`. No legacy UI states are needed.
  2. **Item Details (375px):** Tapping a card opens a modal overlay detailing the context. The "Approve" button is sticky at the bottom.
  3. **Interaction:** Tapping "Approve" immediately removes the item from the dashboard feed list (optimistic update) while the backend updates the `agent_feed_items` table.

  ### Mobile UX Flow
  1. User (e.g., Maya) receives a push notification and opens the app to the Dashboard.
  2. The unified feed immediately loads `agent_feed_items` (no `UNION ALL` delays).
  3. Maya reviews an Instagram DM draft (an `AgentFeedItem`) and taps "Approve".
  4. The UI seamlessly transitions the item out of view.

  ### AI Agent Integration Points
  - **Marketing Agent:** Instead of writing to `agent_approvals` for social post drafts, it will write directly to `agent_feed_items` with a standardized schema.
  - **Operations Agent:** Inventory restock alerts will be pushed to `agent_feed_items`.
  - **Sales Agent:** Lead follow-up tasks will use the unified `agent_feed_items` structure.

  ### Proposed Architecture Changes
  1. **Deprecate `agent_approvals`:** All AI departments (Marketing, Operations, etc.) must write exclusively to the `agent_feed_items` table.
  2. **Standardize Data Shape:** The `AgentFeedItem` struct will be the sole source of truth. The `lifecycle_state` enum (`PENDING_APPROVAL`, `APPROVED`, `DISMISSED`) replaces the old `status` strings.
  3. **Simplify UI:** The `dashboard.html` and `triage.html` UI components will be heavily simplified to expect only one object shape, removing the legacy parsing logic.

  ## Implementation Prompt
  **Objective:** Unify the Agent Feed data model by migrating all legacy `agent_approvals` interactions to the new `agent_feed_items` table, standardizing the schema, and simplifying the mobile UI.

  **User Journey (CUJ):**
  As Maya the Baker, I open my OHC app and see a clean, unified feed of actions (Instagram DMs, Restock suggestions, Draft social posts). When I tap "Approve", the card instantly disappears and the action is executed without any ghosting or sync delays.

  **Acceptance Criteria:**
  1. Migrate all backend queries serving the feed to rely exclusively on the unified schema (do not use UNION ALL across disparate tables).
  2. Update worker services to publish their tasks/actions to the unified feed table instead of legacy approval queues.
  3. Simplify the frontend UI rendering logic to process a single, predictable data shape.
  4. Implement E2E tests verifying the unified feed flow from agent creation to user approval on the dashboard.

  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
