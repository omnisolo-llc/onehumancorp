outcome: no_work
issue_title: "Implement Native Omnichannel Chat (Legacy Chat Replacement)"
issue_description: |
  I have investigated issue #35642 which requests the implementation of the Native Omnichannel Chat.
  Based on the current state of the codebase, I found the following evidence:
  1. The migrations for the omnichannel chat already exist: `src/server/migrations/1009_native_omnichannel_chat.sql`.
  2. The data models and the `ChatService` have already been implemented in `src/server/services/chat/models.rs` and `src/server/services/chat/service.rs`. The code perfectly maps the tables from the migrations and matches the issue description requirements.
  3. The issue requests the core data schema and Rust service layer for the native OHC Omnichannel Chat system, which is already present.

  Therefore, this is a "blocked no-work finding" since the required code is already in the codebase.
issue_priority: P0
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
