outcome: no_work
issue_title: "Native Rust Omnichannel Chat System Architecture (Chatwoot Replacement)"
issue_description: |
  I have investigated the codebase and found that the Native Rust Omnichannel Chat System Architecture is already implemented.
  - The database migrations for the chat schema are present in `src/server/migrations/233_chat_omnichannel.sql` and `src/server/db/migrations/233_chat_omnichannel.sql`. They define `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` with strict RLS on `tenant_id` (enforcing multi-tenant isolation).
  - The core logic and endpoints for CRUD operations are implemented in `src/server/services/chat/service.rs` (e.g., `create_inbox`, `create_channel`, `create_contact`, `start_conversation`, `send_message`), fulfilling the backend data access requirements.
  - The websocket real-time events implementation exists in `src/server/api/unified_ws.rs`, indicating real-time WebSocket foundation is present.
  - Since the requested feature is already implemented, I am returning a `no_work` report as per instructions.
