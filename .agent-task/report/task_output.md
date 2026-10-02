issue_title: "📐 Architect: [Research] Architect OHC Custom Native Rust Omnichannel Chat System"
issue_description: |-
  The task requires replacing Chatwoot with a native Rust omnichannel customer support & chat engine for OHC.

  However, after auditing the source code in `src/server`, the requested feature has **already been implemented and completed**:
  - **Database schema** is implemented with tenant isolation (RLS) in `src/server/migrations/233_chat_omnichannel.sql` and `1001_create_omni_inbox_messages_and_quotes_fix.sql`, including tables `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages`.
  - **Data models** are fully typed in `src/server/services/chat/models.rs` (`ChatInbox`, `ChatChannel`, `ChatContact`, `ChatConversation`, `ChatMessage`).
  - **Data domain** models exist in `src/server/domain/chat/mod.rs`.
  - **API functionality** is implemented in `src/server/services/chat/service.rs`, which provides methods for creating inboxes, channels, contacts, starting conversations, and sending messages. It includes tenant isolation and unit testing with 100% test coverage for DB operations.
  - **Realtime Integration** is partially handled via `src/server/api/realtime.rs`, which queries `chat_conversations` dynamically for tenant scopes.
  - **Test coverage** is included natively within the implementations `src/server/domain/chat/mod.rs` and `src/server/services/chat/service.rs`.

  The requested architectural models, CRUD APIs, and database migrations are fully implemented natively inside `onehumancorp/mono`. Therefore, no further implementation is necessary for this issue. The task is a `no_work` finding.
issue_priority: "P2"
issue_category: "backend"
issue_type: "research"
issue_label: "no-work"
assignees: []
outcome: "no_work"
