outcome: no_work
issue_title: Native Rust Omnichannel Chat System Architecture (Chatwoot Replacement)
issue_description: |
  The implementation of the native Rust omnichannel chat system is already present in the codebase.
  Specifically:
  - The PostgreSQL migrations for `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` with tenant isolation policies have been created (`src/server/db/migrations/1009_native_omnichannel_chat.sql`, `src/server/db/migrations/233_chat_omnichannel.sql`).
  - The `OmniChannelRepo` is present in `src/server/domain/repository/omnichannel_repo.rs` with methods to interact with `conversations` and `messages` tables.
  - The `ChatService` inside `src/server/services/chat/service.rs` has fully implemented CRUD for the chat system.
  - The API endpoints for chat operations are built (e.g. `src/server/api/widget/chat.rs` with `create_conversation`, `create_message`, `get_messages`).

  Unverified criteria:
  - The WebSocket signaling foundation (using Axum/Tokio) specifically for real-time `message.created` events was not found in the verified files. (General unified websocket handling exists in `src/server/api/unified_ws.rs`, but the specific event handler for `message.created` related to chat was not identified in the trace).
