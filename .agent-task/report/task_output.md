issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  The requested issue ("Implement Custom Rust Omnichannel Chat System") describes implementing core tables (inbox, conversation, message) and APIs for a native omnichannel chat.
  Upon auditing the codebase, it is clear that this system has already been implemented:
  - Tables such as `chat_inboxes` and `chat_channels` are implemented in `src/server/migrations/1009_native_omnichannel_chat.sql` with multi-tenant isolation via RLS policies.
  - The models are defined in `src/server/services/chat/models.rs` using SQLx.
  - The service layer CRUD operations are defined in `src/server/services/chat/service.rs`.
  - Real-time websocket broadcasts and unified inbox features are implemented via `src/server/api/unified_ws.rs` and `src/server/api/unified_inbox_webhook.rs`.

  The acceptance criteria outlined in the issue are already satisfied by existing code in the repository. As per instructions, "If the problem or default behavior described by the issue is obsolete... If the issue is already complete or requires unavailable authorization/evidence, return an explicit no_work or blocked outcome."
  Superpowers skill provenance: None loaded
outcome: blocked
