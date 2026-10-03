outcome: no_work
issue_title: "Implement Mobile-First Agent Approval Feed (The Central OHC Dashboard)"
issue_description: |
  **Issue**: The user wants a mobile-first unified agent approval feed (375px mobile viewport) where owners can review, approve, or reject actions drafted by various AI agents, replacing complex, multi-tab dashboards.

  **Verified limitations & Existing Capabilities**:
  The system already implements this requirement in exactly the way requested.
  - The UI for the agent feed is located in `src/ui/tauri/src/components/AgentFeed.tsx` and `src/ui/tauri/src/components/AgentFeedCard.tsx`.
  - It uses a mobile-first approach, wrapped in `w-full max-w-[375px] mx-auto`.
  - It fetches pending tasks from `/api/v1/inbox/action_required` and provides a vertical feed of "Action Cards".
  - Cards include "Approve & Send" and "Edit Draft" buttons.
  - The UI employs OHC glassmorphism tokens (e.g. `bg-[rgba(255,255,255,0.65)]`, `backdrop-blur-[30px]`, `backdrop-saturate-[210%]`).
  - The backend endpoints are present at `src/server/api/inbox/action_required.rs` which interact with `src/server/domain/repository/action_required_queue_repo.rs`.
  - The `ActionRequiredQueueRepo` queries the `agent_draft` database table and ensures multitenant isolation.
  - All described functionality in the design doc is actively working in the codebase and has test coverage (`src/ui/tauri/src/components/__tests__/AgentFeed.test.tsx` and `src/server/api/inbox/action_required_test.rs`).

  **Outcome**:
  Because the requested mobile-first agent approval feed has already been implemented and merged, no further code changes are required. The task is a blocked no-work finding as the code already satisfies the requirements exactly.
issue_priority: P0
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
