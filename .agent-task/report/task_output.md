outcome: no_work
issue_title: "Native Rust Omnichannel Chat System & Universal Inbox"
issue_description: |
  The Omnichannel Chat System has already been natively implemented in Rust. The `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` tables with tenant isolation RLS are created in `src/server/db/migrations/233_chat_omnichannel.sql` and `scripts/widget-chat-contract/compatibility/1009_native_omnichannel_chat.sql`. The logic is fully handled by `OmniChannelRepo` in `src/server/domain/repository/omnichannel_repo.rs` and the `ChatService` in `src/server/services/chat/service.rs`. External integrations like Chatwoot have already been removed.
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: []
assignees: []
