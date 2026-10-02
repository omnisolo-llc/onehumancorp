outcome: no_work
issue_title: "Native Rust Omnichannel Chat System Architecture (Chatwoot Replacement)"
issue_description: |
  I have investigated the codebase and found that the requested feature (native Rust omnichannel chat system replacing Chatwoot) is already implemented and completed.

  Evidence:
  - The schema and database migrations are present in multiple SQL files (e.g., `src/server/db/migrations/233_chat_omnichannel.sql` and `src/server/db/migrations/1009_native_omnichannel_chat.sql`). These files define tables for `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` with complete Row-Level Security (RLS) enforcement using `app.current_tenant_id` as specified in the issue.
  - The Rust service layer implementing the requested data models and CRUD endpoints is already implemented inside `src/server/services/chat/service.rs`. The code uses Axum and SQLx to securely insert and query these tables while enforcing strict tenant isolation.
  - The domain models are correctly defined in `src/server/domain/chat/mod.rs` mapping to the entities described in the architectural diagram (Inbox, Conversation, Message, Contact).
  - The issue specifies the removal of the external Chatwoot dependency, and searches across the codebase show no residual traces or integration with `Chatwoot`.

  Therefore, no new work is required as the acceptance criteria described in the issue are fully met by the current codebase state.
