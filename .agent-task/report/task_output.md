outcome: no_work
issue_title: "Implement Native Omnichannel Chat & AI Unified Inbox (Chatwoot Replacement)"
issue_description: |
  The requested Native Omnichannel Chat & AI Unified Inbox (Chatwoot Replacement) has already been implemented natively within the OmniSolo architecture. Chatwoot has been completely removed as part of a prior migration (see `docs/superpowers/plans/2026-07-13-chatwoot-removal.md`).

  The research and strategy evaluation regarding compute/API charging, customer BYOK, and local inference mentioned in the task instructions remain unaddressed in this report as they fall outside the scope of the Chatwoot replacement implementation findings.

  Verification Evidence:
  - Database migrations `233_chat_omnichannel.sql` and `1009_native_omnichannel_chat.sql` exist and enable row-level security for native chat tables (`chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, `chat_messages`).
  - Rust backend services (`src/server/services/chat/`) and API routes (`src/server/api/omnichannel_webhook.rs`, `src/server/api/omni_inbox_webhook.rs`) are implemented to handle incoming messages and AI triage actions natively.
  - End-to-end tests (e.g., `src/e2e/omni_inbox_triage.mock-contract.ts` and `src/e2e/tests/unified_work_triage.mock-contract.ts`) explicitly test the AI unified inbox and triage behavior.
  - A residue guard script (`deploy/tests/no_chatwoot_residue_test.sh`) enforces that no Chatwoot footprint remains, and searching the codebase (`git grep -l -i 'chatwoot' -- . | sort`) confirms only approved historical and verification references exist.
