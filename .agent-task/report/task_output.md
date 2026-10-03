issue_title: "Implement Native Rust Omnichannel Chat System Core Architecture"
issue_description: |
  The requested feature is already implemented in the codebase.
  The PostgreSQL data schemas for `chat_inboxes`, `chat_conversations`, `chat_messages`, `chat_channels`, and `chat_contacts` with Row Level Security (tenant_id) are already fully defined and applied in `src/server/migrations/233_chat_omnichannel.sql` and `src/server/migrations/1009_native_omnichannel_chat.sql`.
  The core API and backend routing logic are already present in `src/server/api/omnichannel_webhook.rs`, `src/server/api/unified_inbox_webhook.rs`, and `src/server/services/inbox/service.rs`.
  Since the native omnichannel architecture replaces Chatwoot completely and operates exactly as requested, this is considered a no-work finding.
issue_priority: "P2"
issue_category: "backend"
issue_type: "feature"
issue_label: "agent-ready"
assignees: []
outcome: no_work
