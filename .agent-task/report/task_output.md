outcome: no_work
issue_title: "Native Rust Omnichannel Chat: Core Data Model & Multitenancy (Chatwoot Migration)"
issue_description: |
  The issue #36804 requested the creation of the foundational PostgreSQL schema and core Rust data structures for the Native OHC Omnichannel Chat system.

  After reviewing the current codebase, the requested functionality is already fully implemented. The exact tables requested (chat_inboxes, chat_channels, chat_contacts, chat_conversations, chat_messages) with strict multitenancy (tenant_id) and Row Level Security (RLS) policies are fully defined in the existing migration files:
  - `src/server/db/migrations/1009_native_omnichannel_chat.sql`
  - `src/server/db/migrations/233_chat_omnichannel.sql`
  - `src/server/migrations/233_chat_omnichannel.sql`

  The RLS uses `current_setting('app.current_tenant_id', true)::uuid` as expected.

  Additionally, the corresponding Rust models (ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage) and the `ChatService` logic to interact with the database are already implemented and located at:
  - `src/server/services/chat/models.rs`
  - `src/server/services/chat/service.rs`

  Unit tests for the service and RLS isolation checks are also present in `service.rs`.

  Because the target of this issue has already been fully executed and implemented as per the acceptance criteria, this task is marked as `no_work`.
