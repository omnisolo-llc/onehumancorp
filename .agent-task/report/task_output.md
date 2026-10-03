outcome: no_work
issue_title: "Native Rust Omnichannel Chat: Core Data Models & Schema Design"
issue_description: |
  The requested data models and schema design for the native Rust Omnichannel Chat system are already fully implemented.

  Evidence of existing implementation:
  - Migrations: `src/server/migrations/233_chat_omnichannel.sql` and `src/server/migrations/1009_native_omnichannel_chat.sql` define the required tables (`chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, `chat_messages`) with `tenant_id` columns, multi-tenant RLS policies, and indexes.
  - SeaORM models: `src/server/integrations/omnichannel/src/models.rs` contains the corresponding Rust entity structs for these tables.
  - RLS testing: `src/server/services/chat/service.rs` implements comprehensive unit tests that verify RLS policies correctly isolate data between different tenants (e.g. `assert_open_conversation_scope`).
