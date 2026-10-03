outcome: no_work
issue_title: "Implement Native Rust Omnichannel Chat & Agentic Triage (Chatwoot Replacement)"
issue_description: |
  Verified that the Native Rust Omnichannel Chat & Agentic Triage feature is already completely implemented.

  Scope comparisons:
  1. "Implement the Conversation and Message tables in PostgreSQL with Row Level Security (RLS) by tenant_id":
     Already exists as `unified_threads`, `unified_messages`, `unified_triage_actions`, and `omni_inbox_messages` with complete RLS (e.g. `CREATE POLICY tenant_isolation_unified_threads ON unified_threads USING (tenant_id::text = current_setting('app.current_tenant', true))` in `src/server/migrations/1014_feature_parity_and_runtime_contract.sql`).
  2. "Build the Rust gRPC/REST endpoints for message ingestion":
     Already implemented in `src/server/api/inbox/webhook.rs` and `src/server/api/unified_inbox_webhook.rs` via `handle_omnichannel_webhook`.
  3. "Build the 375px optimized Flutter UI card displaying the drafted reply with 'Approve' and 'Edit' buttons":
     Per memory constraints ("explicitly migrated away from the old Slint/Flutter UI; the maintained UI is Next.js inside a Tauri shell"), this is already implemented via the Next.js/React `AgentFeedCard` component located in `src/ui/tauri/src/components/AgentFeedCard.tsx` (which contains `Approve & Send` and `Edit Draft` buttons inside a 375px limited viewport card).
  4. "E2E Playwright test must prove a simulated webhook ingestion results in a clickable 'Approve' button in the UI":
     Covered by `src/e2e/omnichannel_feed.mock-contract.ts` test `receives instagram DM webhook and approves AI draft in dashboard`.

  As all requirements are completely implemented and integrated into the current repository stack and the explicit tests already exist for the CUJ, there is no eligible work remaining for this issue. Returning a blocked/no-work finding.
issue_priority: P1
issue_category: implementations
issue_type: feature
issue_label: [agent-ready, ohc:lane:ui]
assignees: []
