outcome: no_work
issue_title: "Architecture Design: Native Rust Omnichannel Chat System"
issue_description: |
  Upon reviewing the codebase, the native Rust omnichannel customer support and chat system has already been fully implemented.
  - The necessary data models (Inbox, Conversation, Message, Channel) are already defined in `src/server/integrations/omnichannel/src/models.rs`.
  - Migrations such as `1009_native_omnichannel_chat.sql` and `233_chat_omnichannel.sql` have already created the `chat_inboxes`, `chat_conversations`, `chat_messages`, and `chat_channels` tables with row-level security for multi-tenant isolation.
  - The backend repository functions are present in `src/server/domain/repository/omnichannel_repo.rs`.
  - The UI for a unified inbox with AI draft integration has already been created in `src/ui/next/src/app/inbox/page.tsx` and handles WebSocket/realtime data rendering.

  Since the requested feature is already present, no further code implementation is required.
