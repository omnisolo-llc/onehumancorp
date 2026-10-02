outcome: blocked
issue_title: "GitHub Issue #36850: [Platform] Implement Native Rust Omnichannel Chat Inbox Foundation"
issue_description: |
  The issue requested the creation of database schema migrations and Rust data models (Diesel structs) for `inboxes`, `conversations`, and `messages` within the OHC server.

  Upon inspecting the repository, the requested schema migrations and models have already been implemented in `src/server/migrations/1009_native_omnichannel_chat.sql` and `src/server/services/chat/models.rs`, respectively.

  - `chat_inboxes`
  - `chat_conversations`
  - `chat_messages`

  Since the native omnichannel chat schema migrations and Rust data models are already fully implemented with multi-tenant isolation, there is no further safe and well-scoped implementation that follows from the repository state. The acceptance criteria are satisfied by the existing implementations.

  Therefore, no new work is needed.
issue_priority: "P0"
issue_category: "Platform"
issue_type: "Feature"
issue_label: ""
assignees: []
