outcome: no_work
issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  The Omnichannel Chat System replacing Chatwoot is already implemented.

  Verified Acceptance Criteria:
  - Database migrations for the core chat schema are created. (Verified in `src/server/db/migrations/1009_native_omnichannel_chat.sql` and `src/server/migrations/233_chat_omnichannel.sql`).
  - Rust API endpoints for CRUD operations on conversations and messages pass all unit tests. (Verified in `src/server/services/chat/service.rs` containing `test_create_inbox`).
  - Tenant isolation is verifiably enforced. (Verified in `src/server/services/chat/service.rs` containing `test_rls_isolation`).
  - No external Chatwoot dependency is used. (Verified via `grep -ri "chatwoot" src/server/`).

  Unverified Criteria/Gaps:
  - The WebSocket signaling foundation for real-time `message.created` events is not explicitly verified as fully integrated with the chat service in the observed unit tests, though WebSocket paths exist in `src/server/api/realtime.rs`.
