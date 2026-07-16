issue_title: "Unified Owner Feed: Deprecate Fragmented Action Queues (agent_draft, triage_proposed_actions)"
issue_description: |
  ## Problem Statement
  Currently, the OneHumanCorp backend contains multiple disjointed tables and API routes for handling actions that require owner approval:
  - `agent_draft` table combined with `work_item` for customer interactions (legacy route API: `/api/v1/inbox/action_required`).
  - `triage_items` / `triage_proposed_actions` for generalized event triage.
  - `agent_feed_items` which is the intended unified system.

  From the perspective of an owner (e.g., Maya the baker or Carlos the handyman), having multiple isolated feeds forces them to check different places or causes UI fragmentation where some approvals live in the inbox and others in a dashboard feed. The system must present a single, unified "Owner Feed" where all actionable intelligence (drafted replies, proposed quotes, anomaly alerts) flows into one queue, uniformly typed and easily actioned.

  ## Research Report
  - **Codebase findings:**
    - `src/server/domain/repository/action_required_queue_repo.rs` fetches drafts joining `agent_draft`, `work_item`, and `customer_profile`.
    - `src/server/migrations/109_triage_items.sql` introduced `triage_proposed_actions`.
    - `src/server/migrations/143_agent_feed_items.sql` and `src/server/lib.rs` have extensive support for `agent_feed_items` containing `context_payload` and `proposed_action` JSONB blobs.
    - The frontend `AgentFeed` component in `src/ui/tauri/src/components/AgentFeed.tsx` still fetches from the legacy `/api/v1/inbox/action_required` route but expects a payload that partially resembles `agent_feed_items`.
  - **Competitive Analysis:** Systems like Shopify Sidekick and Tencent Workbuddy unify operator interventions into a single stream. The user doesn't distinguish between a "triage action" and an "inbox draft action"; they are all just "Tasks requiring my approval."

  ## Design Doc
  - **Architecture Diagram (Mental Model):**
    All AI departments (Customer Service, Operations, Sales) -> single `agent_feed_items` table.
    - Remove/Deprecate `agent_draft` and `/api/v1/inbox/action_required` endpoint.
    - Remove/Deprecate `triage_proposed_actions`.
    - Expand unified `/api/v1/feed` endpoint backed by `agent_feed_items` that returns strongly-typed feed items (e.g., `DraftReply`, `QuoteApproval`, `PayoutSummary`).
  - **Mobile UX Flow:** A single tab on the 375px mobile UI showing a chronologically prioritized list of action cards. Swiping or tapping "Approve" triggers the associated action via a unified webhook/action endpoint.
  - **AI Agent Integration:** When an agent drafts a reply or proposes a quote, it always inserts an `agent_feed_items` row with `lifecycle_state = 'PENDING_APPROVAL'` and a JSONB `proposed_action`.

  ## Implementation Prompt
  Implement a unified Owner Feed by fully migrating the legacy `action_required` (draft replies) and `triage` flows to use the `agent_feed_items` table.
  1. Modify the backend to ensure any drafted replies from the Customer Service agent are inserted into `agent_feed_items` instead of `agent_draft`.
  2. Deprecate and remove `action_required_queue_repo.rs` and the `/api/v1/inbox/action_required` routes.
  3. Update `src/ui/tauri/src/components/AgentFeed.tsx` to fetch from the unified feed endpoint and render `AgentFeedCard` using the standard `context_payload` and `proposed_action` schema.
  4. Drop the `agent_draft` and `triage_proposed_actions` tables from the database.
  5. Provide exhaustive E2E tests confirming a drafted reply appears in the single feed and can be approved.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
