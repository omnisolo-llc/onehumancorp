outcome: no_work
issue_title: Architecture Design: Native Rust Omnichannel Chat Engine
issue_description: |
  The requested omnichannel chat engine is already fully implemented in the current codebase.

  Concrete Evidence:
  - Migrations: `src/server/db/migrations/1009_native_omnichannel_chat.sql` and `src/server/db/migrations/233_chat_omnichannel.sql` contain the exact `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` tables with RLS policies (`tenant_isolation_policy`).
  - Rust API & Services: `src/server/services/chat/service.rs`, `src/server/services/chat/models.rs`, and `src/server/api/widget/chat.rs` implement the core data models and CRUD operations in native Rust.
  - E2E Tests: `src/e2e/onboarding-chat.spec.ts` verifies the chat bubble UI styling (e.g. flexbox, border-radius, background colors), and `src/e2e/team_chat.spec.ts` and `src/e2e/help_chat.spec.ts` verify the actual chat interfaces.
