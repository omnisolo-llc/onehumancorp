outcome: no_work
issue_title: "[Architecture] Implement Native Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: >
  The requested implementation already exists in the codebase and satisfies the acceptance criteria. The PostgreSQL database schema migrations for `chat_inboxes`, `chat_contacts`, `chat_conversations`, and `chat_messages` with multi-tenant RLS are present in `src/server/migrations/233_chat_omnichannel.sql` and `src/server/db/migrations/233_chat_omnichannel.sql`. The Rust data models and repository layer are implemented in `src/server/domain/repository/omnichannel_repo.rs`. The backend service and API are implemented in `src/server/services/omnichannel_service.rs` and `src/server/api/chat.rs`.
