outcome: no_work
issue_title: "Build Native Rust Omnichannel Customer Identity & Chat Engine (Replacing Chatwoot)"
issue_description: |
  The requested feature is already fully implemented in the current repository state.

  Acceptance Criteria Verification:
  1. PostgreSQL schema migrations with RLS: Verified. `src/server/db/migrations/233_chat_omnichannel.sql` and `src/server/db/migrations/1009_native_omnichannel_chat.sql` implement `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` with `tenant_id` and Row-Level Security.
  2. Rust Axum webhook endpoint: Verified. `src/server/api/omnichannel_webhook.rs` and `src/server/lib.rs` map the `/api/v1/webhooks/omnichannel` route.
  3. Handler identity resolution & message insertion: Verified. The handler in `src/server/api/omnichannel_webhook.rs` calls `resolve_identity` and successfully inserts messages into the omnichannel tables.
  4. Trigger event upon message insertion: Verified. `src/server/api/omnichannel_webhook.rs` creates a `tenant.omnichannel.message.received` event and dispatches it via the orchestrator.
  5. Playwright E2E tests: Verified. `src/e2e/playwright/omnichannel_approval.mock-contract.ts` tests the exact scenario: user logs in, sees a drafted message card via webhook payload, and approves it on the mobile feed.
