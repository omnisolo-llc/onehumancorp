outcome: no_work
issue_title: "Implement Native Rust Omnichannel Inbox to Replace Chatwoot Dependency"
issue_description: |
  The requested feature to implement a Native Rust Omnichannel Inbox to replace the Chatwoot dependency is already implemented and completed.

  Verification evidence:
  1. The `docs/reports/production_agent_optimization_report.md` file explicitly confirms the Chatwoot removal and states: "Removed from the active application and deployment graph... The native OmniSolo omnichannel inbox remains in place".
  2. The native Rust codebase already contains multiple implementations for omnichannel webhooks: `src/server/api/omnichannel_webhook.rs`, `src/server/api/inbox/webhook.rs`, and `src/server/api/unified_inbox_webhook.rs`, handling ingestion and AI drafting.
  3. The `src/e2e/omnichannel_unified_inbox.mock-contract.ts` file contains an active End-to-End test that validates the `tenant.omnichannel.message.received` event and UI behavior.

  Therefore, no new implementation is needed.
