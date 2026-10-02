outcome: no_work
issue_title: "Implement Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  The native omnichannel chat system has already been fully implemented in the codebase as a Chatwoot replacement.
  Existing files include src/server/db/migrations/1009_native_omnichannel_chat.sql, src/server/services/chat/service.rs, and src/server/domain/chat/mod.rs.
  They implement the requested models (Inbox, Conversation, Message, Contact) with multi-tenancy and PostgreSQL RLS enabled.
