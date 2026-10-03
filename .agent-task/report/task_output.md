outcome: "no_work"
issue_title: "Native Rust Omnichannel Chat: Core Models & Multi-tenant Inbox Architecture"
issue_description: |
  The requested Native Rust Omnichannel Chat functionality already exists in the codebase and satisfies all acceptance criteria.

  Evidence:
  1. Migrations implementing strict row-level security (RLS) on tenant_id for the chat entities exist at src/server/migrations/233_chat_omnichannel.sql and src/server/migrations/1009_native_omnichannel_chat.sql.
  2. The core data models (ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage) are defined in src/server/services/chat/models.rs and src/server/integrations/omnichannel/src/models.rs.
  3. The Rust data access layer (repository) enforcing tenant isolation with a real database pool (PgPool) is implemented in src/server/domain/repository/omnichannel_repo.rs and src/server/services/chat/service.rs.
  4. API endpoints providing functionality to create and list conversations and messages, with tenant validation, are available at src/server/api/widget/chat.rs.
