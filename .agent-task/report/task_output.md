issue_title: "[Native Chat] Implement Rust-Native Omnichannel Chat & Messaging Engine (Chatwoot Replacement)"
issue_description: "The issue #36168 requesting the implementation of a Rust-native omnichannel chat engine to replace Chatwoot has been evaluated and confirmed as already implemented. An analysis of the repository shows the requested Rust data models and PostgreSQL schemas (including `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` tables with tenant isolation and row-level security) are already present and fully implemented. Specifically, the data models reside in `src/server/services/chat/models.rs`, the schema migrations in `src/server/migrations/233_chat_omnichannel.sql` and `src/server/migrations/1009_native_omnichannel_chat.sql`, and the core logic in `src/server/services/chat/service.rs`. The current behavior matches the exact issue requirements. Following the repository's strict no-work reporting rules for redundant tasks, this is marked as a blocked no-work finding."
issue_priority: "P0"
issue_category: "backend"
issue_type: "feature"
issue_label: "no_work"
outcome: "blocked"
assignees: []
