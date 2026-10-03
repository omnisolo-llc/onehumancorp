issue_title: "GitHub Issue #36221: Native Rust Omnichannel Inbox & Chat System"
issue_description: |
  The requested feature, GitHub Issue #36221, asking for a Native Rust Omnichannel Inbox & Chat System, has already been implemented in the codebase.

  The `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` tables are successfully defined in `src/server/migrations/233_chat_omnichannel.sql` and `src/server/migrations/1009_native_omnichannel_chat.sql` with full row level security (RLS) policies targeting `tenant_id` to ensure isolation.

  Additionally, the native implementation is located at `src/server/integrations/omnichannel` with `models`, `repository`, and `router` fully present and correctly mapped. All omnichannel tests within that crate pass seamlessly (`cargo test -p server_integrations_omnichannel`).

  The workspace successfully compiles with these features implemented without regressions (`cargo check --locked -p omnisolo`).

  Due to the memory constraints forbidding duplicate work and redundant tasks, this is marked as a blocked no-work finding.
issue_priority: P0
issue_category: UI/UX
issue_type: feature
issue_label: [agent-ready]
assignees: []
outcome: blocked
