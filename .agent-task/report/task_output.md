outcome: no_work
issue_title: "Implement Native Omnichannel Unified Inbox & Agentic Customer Triage"
issue_description: |
  ## Finding: No-Work (Already Implemented)

  The "Native Omnichannel Unified Inbox & Agentic Customer Triage" feature described in Issue #36411 is already fully implemented in the current codebase.

  **Evidence:**
  - **Backend**: The Rust backend has a migration (`1001_create_omni_inbox_messages_and_quotes_fix.sql`) and an active data model in `omni_inbox_messages` handling properties like `original_content`, `draft_reply`, `status`, and `source`. The `load_ui_omni_inbox_from_db` function queries this for the UI, and triage operations are handled by `apply_omni_inbox_action` (both in `src/server/lib.rs`).
  - **Frontend**: The React UI at `src/ui/next/src/app/inbox/page.tsx` correctly consumes this API, rendering a unified feed with actionable agentic triaging buttons (e.g., "✨ Send quote for $", "✨ Approve & Send Draft (Deduct Inventory)").
  - **Tests**: Playwright tests such as `src/ui/next/src/e2e/omni_inbox.spec.ts` and `src/ui/next/src/e2e/omni_inbox_triage.spec.ts` explicitly test this capability and wait for the correct agentic triage buttons.

  Per the operating contract, creating dummy changes or rewriting working features for an already complete capability is prohibited. Thus, this issue requires no new work.
