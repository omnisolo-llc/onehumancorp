outcome: no_work
issue_title: "Research & Design: Native Rust Omnichannel Chat Integration & AI Triage"
issue_description: |-
  The task is marked as "blocked no-work finding" because the requested implementation is largely already in place.

  **Verified Implementations:**
  1. **Omnichannel SQL Migrations**:
     - `src/server/db/migrations/1009_native_omnichannel_chat.sql` and `233_chat_omnichannel.sql` explicitly verify the presence of `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` tables with tenant isolation policies.
     - `src/server/db/migrations/150_unified_inbox_triage.sql` explicitly verifies the presence of `unified_threads`, `unified_messages`, and `unified_triage_actions`.
  2. **Rust Backend Implementation**:
     - `src/server/services/chat/service.rs` and `src/server/services/chat/models.rs` explicitly verify the `ChatService` which processes inbox, channel, and message creations.
     - `src/server/domain/repository/omnichannel_repo.rs` explicitly verifies the existence of `AiDraft` structure, and functions `create_ai_draft` and `update_ai_draft_status`.
  3. **UI / Frontend**:
     - `src/ui/next/src/app/triage/page.tsx` explicitly verifies a "Work Triage" frontend featuring glassmorphism styles, `isProcessing`, `handleDecision`, and translucent cards tailored towards "AI Drafted: {item.action_type}".
     - `src/ui/next/src/app/team/chat/page.tsx` explicitly verifies a chat page allowing approval via `approve-action-btn`.

  **Unverified Details:**
  1. `ai_draft_status` is not explicitly found as a column in the `chat_messages` table from the SQL migration. It uses a separate `ai_drafts` table via `omnichannel_repo.rs`.
  2. Webhook endpoints idempotency and Playwright 100% E2E test coverage are unverified due to test failures (`omnisolo` tests fail to compile, and `make test` times out).
