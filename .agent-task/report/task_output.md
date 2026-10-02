outcome: no_work
issue_title: "Architecture: Native Rust Omnichannel Inbox & Chatwoot Replacement"
issue_description: |
  The implementation of a native Rust omnichannel inbox for chat communication is already complete within the `chat` service, as found in `src/server/services/chat/`. The core tables `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` are already fully implemented with Tenant Row-Level Security in `src/server/migrations/233_chat_omnichannel.sql` and `src/server/db/migrations/233_chat_omnichannel.sql`. Additionally, the `ChatService` logic in `src/server/services/chat/service.rs` is fully operational. This directly contradicts the problem statement that this platform lacks a native multi-tenant omnichannel inbox.

  Unverified Acceptance Criteria:
  - 100% unit test coverage for the CRUD Rust backend module
  - Create an API endpoint for fetching a unified feed of conversations.
