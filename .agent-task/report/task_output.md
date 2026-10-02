issue_title: "Native Rust Omnichannel Chat System"
issue_description: |
  Superpowers skill provenance: Loaded skills/using-superpowers/SKILL.md from f8e9d8dd5c099f417df0c32f6131e9b465e5fb20.

  The Native Rust Omnichannel Chat System issue is already fully implemented.

  Code exploration found that the requested entities (Inbox, Conversation, Message, Contact) and their corresponding Row Level Security (RLS) tenant isolation policies are already implemented in `src/server/migrations/1009_native_omnichannel_chat.sql`. The core service layer handling conversation management and message ingestion is fully implemented in `src/server/services/chat/models.rs` and `src/server/services/chat/service.rs`. The AI drafting functionality with The Ambassador is hooked up via `src/server/api/omnichannel_webhook.rs` and `src/server/services/inbox/service.rs`, tracking draft readiness.

  # Executed test commands
  ```
  cargo test -p server_omnisolo api::omnichannel_webhook::
  ```

  # Verified trace limitations
  None.

issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
outcome: no_work
