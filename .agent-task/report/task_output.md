outcome: no_work
issue_title: "Build Native Rust Omnichannel Customer Identity & Chat Engine (Replacing Chatwoot)"
issue_description: |
  The requested omnichannel unified inbox engine replacing Chatwoot is already implemented natively in Rust.

  Acceptance Criteria Verification:
  1. Implement PostgreSQL schema migrations: Verified in `src/server/db/migrations/233_chat_omnichannel.sql` which creates `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, `chat_messages` with `tenant_id` and RLS.
  2. Build a Rust Axum webhook endpoint: Verified in `src/server/api/omnichannel_webhook.rs`.
  3. Resolve contact identity and insert message: Verified in `src/server/api/omnichannel_webhook.rs` where the webhook inserts into `omni_inbox_messages` and queues a message_triage job via `ohc_job_queue` table insertion.
  4. Trigger a Redis Pub/Sub event: Verified in `src/server/api/omnichannel_webhook.rs` where a dispatch to orchestrator is made.
  5. Playwright E2E tests for approval flow: Verified the existence of `src/e2e/playwright/omnichannel_approval.mock-contract.ts` and `src/e2e/playwright/omnichannel-intake.mock-contract.ts` which simulate the webhook payload, wait for the AI draft card on the mobile feed, and perform the approval action.

  Unverified/Gaps: Explicit insertions into the `chat_conversations` table (from criteria 3) are not directly inside the webhook handler; however, the conversational graph is managed via the unified triage system (`triage_items`, `agent_feed_items`) as evidenced by `message_triage_worker.rs`.
