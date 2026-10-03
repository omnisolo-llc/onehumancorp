outcome: no_work
issue_title: "Native Rust Omnichannel Chat: Core Data Models & Inbox Architecture"
issue_description: |
  The requested issue ("Native Rust Omnichannel Chat: Core Data Models & Inbox Architecture") is completely already implemented in the current repository.

  - The requested PostgreSQL migrations creating `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` with strict `tenant_id` mapping and Row Level Security (`tenant_isolation_policy`) already exist in `src/server/migrations/233_chat_omnichannel.sql` and `src/server/migrations/1009_native_omnichannel_chat.sql`.
  - The Rust models are already fully implemented via standard struct models in `src/server/domain/chat/mod.rs` and `src/server/services/chat/models.rs`, and SeaORM entities in `src/server/integrations/omnichannel/src/models.rs`.
  - The integration code already uses `chat_channels` and other tables accurately in `src/server/services/chat/service.rs`.
  - Comprehensive unit tests exist and verify behavior (e.g. `cargo test -p server_integrations_omnichannel` executed successfully and verified 8 passing tests for omnichannel models and adapters).

  Since the database schema with RLS and corresponding Rust models are already completely present in the codebase along with unit tests, no further implementation is necessary.
