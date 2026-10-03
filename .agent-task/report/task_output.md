outcome: blocked
issue_title: "Implement Native Omnichannel Chat (Legacy Chat Replacement)"
issue_description: |
  The issue requests the implementation of the core data schema and Rust service layer for the native OHC Omnichannel Chat system (`ConversationService`, `MessageService`, and PostgreSQL migrations with strict RLS policies). However, based on the codebase audit, the native omnichannel chat schema (tables `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, `chat_messages` with strict `tenant_id` RLS) and the corresponding Rust services (e.g., `ChatService` inside `src/server/services/chat/service.rs`, `OmniChannelRepo` in `src/server/domain/repository/omnichannel_repo.rs`) have already been fully implemented, tested, and integrated. Attempting to rebuild or rename these to `ConversationService` / `MessageService` would merely introduce duplicate, dummy code without adding any new business functionality, violating the directives to not start new redundant rewrites and to return a `blocked` or `no_work` outcome when the requested implementation is already complete.
issue_priority: "P0"
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
