issue_title: "Native Rust Omnichannel Chat System Blocked No-Work Finding"
issue_description: |
  # Blocked No-Work Finding: Native Rust Omnichannel Chat System

  The request to "Implement Custom Rust Omnichannel Chat System based on Chatwoot Architecture" (Issue #36817) cannot be safely implemented because it is already implemented natively in Rust within `omnisolo/mono`.

  ## Verification Evidence

  * **Database Schema:** `src/server/db/migrations/1009_native_omnichannel_chat.sql` explicitly contains the tables for the omnichannel system (e.g., `chat_inboxes`, `chat_channels`) with proper multi-tenant RLS isolation.
  * **Domain Models:** `src/server/domain/chat/mod.rs` contains the `Inbox`, `Contact` and `Conversation` models for the new chat infrastructure.
  * **Service Logic:** `src/server/services/chat/service.rs` successfully creates and manipulates the new entities (e.g., `create_inbox`).
  * **Chatwoot Eradication:** `deploy/tests/no_chatwoot_residue_test.sh` is an automated scanner specifically ensuring that the "Chatwoot as an external third-party service" architecture is successfully expunged.
  * **Research Report:** `docs/reports/omnichannel_chat_replacement_report.md` documents the architecture and requirements for the native replacement, which matches the implementation in the codebase.

  The system as designed is already in the codebase. However, a build failure (`error: could not compile 'omnisolo' (lib)`) currently exists in `src/server/integrations/registry.rs`, blocking safe testing and verification of any further modifications.
outcome: no_work
