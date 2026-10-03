outcome: no_work
issue_title: "Architecture: Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  The requested Native Rust Omnichannel Chat System is already implemented.
  - The schema for `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` is defined in existing migrations like `src/server/migrations/233_chat_omnichannel.sql` and `src/server/migrations/1009_native_omnichannel_chat.sql` with strict RLS for tenant isolation.
  - The backend Rust service is fully implemented in `src/server/services/chat/service.rs`, which provides functionalities for creating inboxes, channels, contacts, starting conversations, and sending messages. It also includes comprehensive unit tests for tenant isolation.
  - Superpowers skill provenance: Loaded `skills/using-superpowers/SKILL.md` from upstream commit `8ca22dba9a94f28898bbce59f2537ff4d87c747d`.
