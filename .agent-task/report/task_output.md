outcome: no_work
issue_title: "GitHub Issue #35652: Native Rust Omnichannel Chat: Architecture & Design"
issue_description: |
  Verified code state: The requested Native Rust Omnichannel Chat feature is already completely implemented.

  - **Database Schema**: Existing SQL migrations `233_chat_omnichannel.sql`, `1009_native_omnichannel_chat.sql`, and `1025_native_omnichannel_chat.sql` already fully implement `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` tables with strict RLS on `tenant_id`.
  - **Models & Repository**: `src/server/services/chat/models.rs` holds the data models (`ChatInbox`, `ChatChannel`, `ChatContact`, `ChatConversation`, `ChatMessage`), and `src/server/services/chat/service.rs` fully implements the service queries (with `tenant_id` isolation logic) required by the issue.
  - **gRPC API**: `src/proto/inbox.proto` and `src/proto/hub.proto` already define the services. `src/server/services/inbox/service.rs` is implemented as an abstraction. `src/server/domain/repository/omnichannel_repo.rs` already contains data access implementations for these models with comprehensive integration points. The requested task work is fully completed, hence reporting as a no_work finding.
issue_priority: P0
issue_category: operations
issue_type: feature
issue_label: omnichannel
assignees: ""
