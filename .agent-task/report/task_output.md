outcome: no_work
issue_title: "Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  The Native Rust Omnichannel Chat System has already been fully implemented as a Chatwoot replacement.

  Verification evidence:
  1. Migrations for the tables `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` with strict RLS policies already exist in `src/server/migrations/233_chat_omnichannel.sql`, `1009_native_omnichannel_chat.sql`, and `1025_native_omnichannel_chat.sql`.
  2. The Rust models, repository layers, and CRUD API logic (e.g., `MessageRouter`) are implemented in `src/server/integrations/omnichannel/src/`.
  3. Webhook handling for the incoming omnichannel messages is implemented in `src/server/api/omni_inbox_webhook.rs`.
  4. Focused tests in `src/server/integrations/omnichannel/src/lib.rs` are passing, covering models, in-memory repository, and the state machine.
issue_priority: P0
issue_category: integration
issue_type: feature
issue_label: []
assignees: []
