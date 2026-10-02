issue_title: "🔗 Link: [no-work finding: Native Rust Omnichannel Messaging System Replication]"
issue_description: |
  # Native Rust Omnichannel Messaging System Replication

  ## Problem Statement
  OmniSolo currently relies on an external integration for omnichannel customer support and inbox functionality. This violates our core tenet of a unified, self-contained, native platform, and limits our ability to seamlessly inject our AI agents directly into the core event stream. We need to replace the external dependency entirely by replicating its core omnichannel data models, real-time WebSocket messaging, and inbox architecture natively in Rust inside \`omnisolo/mono\`.

  ## Verification Evidence
  The issue is already resolved and verified. Chatwoot has been fully removed from the codebase and the new native Rust omnichannel system is in place.

  - The residue script \`deploy/tests/no_chatwoot_residue_test.sh\` correctly executes with 0 exits and outputs.
  - No active references to Chatwoot in the source code exist.
  - Native models in \`src/server/integrations/omnichannel/src/models.rs\` and webhook handler in \`src/server/api/omnichannel_webhook.rs\` confirm the omnichannel capabilities are natively implemented in Rust.
  - E2E tests for omnichannel interactions such as \`src/e2e/playwright/omnichannel_unified_inbox.mock-contract.ts\` exist.

  Therefore, no further implementation is needed.

issue_priority: P0
issue_category: backend
issue_type: Feature Request
issue_label: omnichannel
outcome: no_work
