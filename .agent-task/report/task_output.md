outcome: no_work
issue_title: "[Native Chat] Implement Rust-Native Omnichannel Chatwoot Replacements (Data Models & Core Service)"
issue_description: |
  Upon exploring the repository, it's evident that the requested features have already been implemented or superseded:

  - The `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` tables, alongside the multi-tenant RLS checks, are defined in `src/server/db/migrations/233_chat_omnichannel.sql` and `src/server/migrations/1009_native_omnichannel_chat.sql`/`1025_native_omnichannel_chat.sql`.
  - The core Rust data models (`sea_orm` models) and the API logic reside in `src/server/integrations/omnichannel/src/models.rs`, `src/server/integrations/omnichannel/src/repository.rs`, and `src/server/integrations/omnichannel/src/router.rs`. All the tests for this crate exist and pass.
  - Furthermore, `src/server/services/chat/service.rs` uses `sqlx` and implements `create_inbox`, `create_channel`, and `create_contact`.
  - Rest APIs are partially wired in `src/server/api/widget/chat.rs` using `OmniChannelRepo` (which handles CRUD operations).
  - The AI Assistant hooks exist in `src/server/api/omni_inbox_webhook.rs` and `src/server/api/omnichannel_webhook.rs`.

  The acceptance criteria outlined in the issue are already satisfied by the codebase.
issue_priority: P1
issue_category: operations
issue_type: feature
issue_label: omnichannel
assignees: []
