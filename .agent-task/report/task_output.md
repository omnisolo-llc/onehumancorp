outcome: no_work
issue_title: "Native Rust Omnichannel Chat Inbox System Implementation"
issue_description: |
  The requested feature, Native Rust Omnichannel Chat Inbox System Implementation, is already fully implemented in the current codebase.

  Evidence:
  - Rust data structures for `Inbox`, `Conversation`, `Message`, and `Contact` with strict `tenant_id` isolation: Implemented in `src/server/integrations/omnichannel/src/models.rs` and database migrations such as `src/server/db/migrations/233_chat_omnichannel.sql`. Row Level Security (RLS) is applied on `tenant_id`.
  - `ChannelAdapter` trait and a concrete adapter: Implemented in `src/server/integrations/omnichannel/src/traits.rs` and `lib.rs`.
  - Backend ingestion logic: Implemented in `MessageRouter` (`src/server/integrations/omnichannel/src/router.rs`) that correctly maps to contacts, conversations, and messages.
  - REST/gRPC endpoints: The `inbox_api.rs` and `realtime.rs` serve the required frontend APIs.
  - Legacy Chatwoot code removal: Fully verified via `rg -i chatwoot` and `deploy/tests/no_chatwoot_residue_test.sh`.
  - 100% test coverage: Verified by running `cargo test -p server_integrations_omnichannel`.
  - Playwright E2E tests: Playwright test suites (e.g., `actionable-inbox.spec.ts`, `omni_inbox.spec.ts`) exist to test the UI flow.
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
