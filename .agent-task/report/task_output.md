outcome: blocked
issue_title: "Implement native Rust Omnichannel Chat to replace Chatwoot"
issue_description: |
  Superpowers skill provenance: 8ca22dba9a94f28898bbce59f2537ff4d87c747d

  The task explicitly requests replacing Chatwoot with a new native Rust Omnichannel Chat System based on `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md`, citing Chatwoot retirement. However, a review of the current audit and codebase shows that Chatwoot has already been removed and replaced by native implementations.

  Verified trace limitations:
  - There are no active Chatwoot references, packages, or services remaining (verified by searching for "Chatwoot").
  - The native Rust Omnichannel API and DB models (`Conversation`, `CustomerProfile`) are already implemented in `src/server/domain/repository/omnichannel_repo.rs`, and the corresponding database tables (`chat_conversations`, `chat_contacts`, `omni_inbox_messages`, etc.) with RLS have already been added (e.g., in `src/server/migrations/1009_native_omnichannel_chat.sql` and `1001_create_omni_inbox_messages_and_quotes_fix.sql`).
  - Webhooks and omnichannel integrations (Twilio, API ingress) are also actively mounted in `src/server/api/inbox/webhook.rs`.

  The issue describes an obsolete state of the codebase. As per RESEARCH.md and the Operating Contract: "Verify current code and tests. If the issue is already complete or requires unavailable authorization/evidence, return an explicit no_work or blocked outcome." and "A no-work/blocked result with evidence is valid. No eligible work is a valid result, not a reason to manufacture a feature." Therefore, no new code implementation is required or safely applicable.
