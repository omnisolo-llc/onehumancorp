outcome: no_work
issue_title: "GitHub Issue #36611: Native Rust Omnichannel Chat System: Implement Core Data Models, Channel Adapters, and Inbox Architecture"
issue_description: |
  The core data models and service methods for the Native Rust Omnichannel Chat System are already implemented in `src/server/services/chat/models.rs` and `src/server/services/chat/service.rs`. The data models correctly implement multi-tenancy and have the required `ChatInbox`, `ChatChannel`, `ChatContact`, `ChatConversation`, and `ChatMessage` structs.

  Additionally, creating an integrations plugin module (`omnichannel`) was previously attempted and later reverted as a no-work finding (PR #39207). It's also already covered by existing integrations like WhatsApp (`src/server/integrations/whatsapp_cloud` and `src/server/integrations/whatsapp`) and others. There is no new architecture code or implementations needed at this point for the data models or basic channel adapter architecture as it exists.
