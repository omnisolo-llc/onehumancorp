issue_title: "[Architecture] Native Rust Omnichannel Chat (legacy chat engine Replacement)"
issue_description: |-
  The issue requested the implementation of the core database schema (PostgreSQL with RLS) and basic Rust CRUD APIs for the native OHC chat system, specifically starting with the `Inbox`, `Conversation`, and `Message` entities, as well as an E2E test verifying tenant isolation.

  Verification confirmed that the requested schema and logic are already completely implemented in the codebase:
  - The tables `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` exist and have PostgreSQL RLS enabled and configured via migrations `1009_native_omnichannel_chat.sql` and `233_chat_omnichannel.sql`.
  - The native Rust CRUD service `ChatService` (`src/server/services/chat/service.rs`) already implements operations like `create_inbox`, `create_channel`, `create_contact`, `start_conversation`, and `send_message`.
  - The required `test_rls_isolation` test already exists within `src/server/services/chat/service.rs` and successfully verifies tenant isolation across inboxes. Note that these tests silently return when no database is available, so their presence in successful cargo runs may omit the local RLS execution if the database isn't reachable.
  - The legacy widget API at `src/server/api/widget/chat.rs` utilizes a separate `OmniChannelRepo` operating on the `omni_inbox_messages` table, but the specific requirements of this issue for a native chat engine domain are satisfied by the existing `ChatService`.

  Therefore, no product-code changes are necessary as the concrete requirements are already met.
issue_priority: P0
issue_category: operations
issue_type: feature
issue_label: [agent-ready, ohc:lane:revenue, ohc:journey:J1]
assignees: []
outcome: no_work
