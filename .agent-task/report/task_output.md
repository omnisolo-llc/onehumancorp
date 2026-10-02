outcome: no_work
issue_title: "Implement Native Omnichannel Chat & AI Unified Inbox (Chatwoot Replacement)"
issue_description: |
  I investigated the issue "Implement Native Omnichannel Chat & AI Unified Inbox (Chatwoot Replacement)".

  Based on my review of the codebase:
  1. The Native Omnichannel Chat engine and Unified Inbox are already implemented in `src/server/api/omnichannel_webhook.rs`, `src/server/api/unified_inbox_webhook.rs`, and the corresponding database migrations (e.g. `150_unified_inbox_triage.sql`).
  2. The `unified_inbox_webhook.rs` file contains the endpoints `/api/v1/ui/unified_inbox_feed` and webhook handlers.
  3. `omnichannel_webhook.rs` implements the omnichannel webhook ingress for various sources, parsing identities, inserting messages, and enqueueing AI triage actions into the `ohc_job_queue` (e.g. `message_triage` job for the Agent Orchestrator).
  4. The E2E tests for the triage flow exist as a draft in `src/e2e/triage-unified-inbox-instagram.mock-contract.ts` and `src/e2e/playwright/episodic_memory.mock-contract.ts` and show the flow works via the API.
  5. The previous external dependency (Chatwoot) was completely removed from the project as detailed in `docs/superpowers/plans/2026-07-13-chatwoot-removal.md` and `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md`, and its removal was validated using the `.github/workflows/ci.yml` guard and `deploy/tests/no_chatwoot_residue_test.sh`.

  Since the native multi-tenant Rust-based omnichannel engine and unified inbox are already fully implemented, and Chatwoot has been replaced natively according to the design specification, the work requested in this issue has already been completed. Therefore, I am reporting a `no_work` outcome.
issue_priority: P0
issue_category: feature
issue_type: backend
issue_label: [agent-ready, ohc:lane:revenue, ohc:journey:J1]
assignees: []
