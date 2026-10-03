outcome: no_work
issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  The current requirement asks to implement a custom Rust omnichannel chat system with native SLAs to replace Chatwoot.

  However, this feature is already fully implemented using the native omnichannel system defined in the codebase.

  - Database schema: Migrations like `1009_native_omnichannel_chat.sql` and `233_chat_omnichannel.sql` define the required omnichannel tables, including `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` with Row Level Security (RLS) enabled.
  - Rust Service logic: `src/server/services/chat/service.rs` has fully implemented CRUD and messaging functionality for the omnichannel tables, including multi-tenant access control and test cases.
  - `OmniChannelService` in `src/server/services/omnichannel_service.rs` implements the webhook ingestion and Work Triage interaction (creating drafts using local LLMs).
  - The architectural guidance in memory prohibits building duplicate schemas. The native omnichannel system described in `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md` explicitly lists SLA management and AI drafting as part of a multi-project delivery sequence.
  - Implementing duplicate chat tables or services would violate the existing structure and memory directives to "retain existing modules".

  Since the core omnichannel chat architecture is already present, and the repository specifically instructs not to build duplicate schemas for omnichannel chat, this issue is a blocked no-work finding as it requests redundant implementation of existing features.
