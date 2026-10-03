outcome: no_work
issue_title: "[Native Chat] Core Rust Omnichannel Inbox Data Model"
issue_description: |
  The requested Core Rust Omnichannel Inbox Data Model is already fully implemented.

  Implementation Details:
  1. Database Migrations: Found in src/server/migrations/233_chat_omnichannel.sql and src/server/migrations/1025_native_omnichannel_chat.sql. Tables chat_inboxes, chat_channels, chat_contacts, chat_conversations, and chat_messages are created.
  2. Tenant ID and RLS: All tables have a tenant_id column and Row-Level Security enabled.
  3. Rust Structs: The corresponding Rust definitions using SeaORM are present in src/server/integrations/omnichannel/src/models.rs.
  4. Test Coverage: Focus tests via `cargo test -p server_integrations_omnichannel` executed successfully (11 tests passed). `make test-backend` timed out after 400.79 seconds, and `cargo check --workspace --exclude app --all-targets` timed out after 401.68 seconds, so focused subsystem tests were used to verify completion.

  Since all acceptance criteria are met, this is a no-work finding.
