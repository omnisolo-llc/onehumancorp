outcome: no_work
issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  The requested feature to implement a custom Rust omnichannel chat system replacing Chatwoot is already implemented.

  Verified Acceptance Criteria:
  1. Database Schema: `chat_inboxes`, `chat_contacts`, `chat_conversations`, and `chat_messages` are implemented with `tenant_id` isolation (verified in `src/server/migrations/233_chat_omnichannel.sql`). `omni_inbox_messages` is also implemented (verified in `src/server/migrations/1001_create_omni_inbox_messages_and_quotes_fix.sql`).
  2. Rust Backend: Axum API endpoints are implemented for webhooks (verified in `src/server/api/omni_inbox_webhook.rs`) and inbox actions (verified in `src/server/api/inbox_api.rs`).
  3. AI Hook: AI drafts are integrated into the `omni_inbox_messages` table via the `draft_reply` field (verified in `src/server/lib.rs` and `src/server/api/omni_inbox_webhook.rs`).
  4. Frontend UI: A React component is built to display the unified inbox (verified in `src/ui/next/src/app/inbox/page.tsx` querying `omni_inbox_messages`).
  5. Action: Approve/Send button functionality is implemented (verified via test locator in `src/ui/next/src/e2e/omni_inbox_triage.spec.ts`).
  6. Tests: Playwright E2E tests are implemented and available (verified `src/ui/next/src/e2e/omni_inbox_triage.spec.ts`).

  Unverified Criteria:
  - None (all specified acceptance criteria components were found in the codebase).
