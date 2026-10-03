outcome: blocked
issue_title: "Native Rust Omnichannel Chat: Inbox, Conversations, and Messaging Pipeline"
issue_description: "The issue instructs to implement 'Native Rust Omnichannel Chat' models and repositories replacing Chatwoot. However, auditing the current implementation indicates the required domains (ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage) along with strict tenant isolation rules, Postgres schema migrations (e.g. `233_chat_omnichannel.sql` and `1009_native_omnichannel_chat.sql`), and the `ChatService` repository layer in `src/server/services/chat/` have already been fully implemented. `src/server/integrations/omnichannel` contains further artifacts. The existing system fulfills the target schema, tenant rules, and AI integration via `omni_inbox_webhook`. Consequently, no further structural implementation is possible."
issue_priority: "P0"
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
