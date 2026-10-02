issue_title: "Implement Core Chatwoot Domain Models in Rust (Native Omnichannel Chat)"
issue_description: |
  The requested domain models for the native omnichannel chat engine replacement already exist in the codebase.

  The tables for \`chat_contacts\`, \`chat_inboxes\`, \`chat_conversations\`, and \`chat_messages\` are fully implemented with the requested structure and multi-tenancy rules (RLS) in \`src/server/migrations/1009_native_omnichannel_chat.sql\`.

  The corresponding Rust structures (\`ChatInbox\`, \`ChatChannel\`, \`ChatContact\`, \`ChatConversation\`, \`ChatMessage\`) are already fully defined in \`src/server/services/chat/models.rs\`.

  The implementation logic is present in \`src/server/services/chat/service.rs\`.

  Because the requested data structures and PostgreSQL DDL are already implemented and meet the requirements, no further action can or needs to be safely taken for this feature.
issue_priority: P0
issue_category: IMPLEMENTER
issue_type: Feature
issue_label: []
assignees: []
outcome: blocked
