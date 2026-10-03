outcome: blocked
issue_title: "Architecture: Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: "Native Omnichannel chat architecture is already present, with tables like chat_inboxes, chat_conversations, chat_messages already implemented with sqlx schemas (e.g. 233_chat_omnichannel.sql). The API router omni_inbox_webhook.rs exists, handles these requests, stores into PostgreSQL/SQLite, enqueues triaging tasks for AI processing, and triggers websocket events. Chatwoot has already been removed per deploy/tests/no_chatwoot_residue_test.sh and the architecture in docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md has already been integrated. Since no work remains, outcome is blocked / no_work."
issue_priority: "P0"
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
