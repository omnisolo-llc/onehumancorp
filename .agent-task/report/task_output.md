issue_title: 'WhatsApp Cloud API Integration'
issue_description: |
  **Outcome**: Blocked / No-work

  **Verification Evidence**:
  - The repository's native implementation in `src/server/integrations/whatsapp_cloud` contains a fully functional provider and client (`provider.rs`, `client.rs`).
  - Webhook ingestion from Meta is already implemented and processing WhatsApp Cloud API events in `src/server/api/meta_webhook.rs`.
  - Storage mapping and identity resolution via WhatsApp accounts to omni-channel customer conversations is already correctly utilizing the `whatsapp_cloud_api` tag inside `src/server/lib.rs` and `src/server/api/meta_webhook.rs`.
  - The issue `#36714` describes implementing a backend solution for native WhatsApp Cloud API integration, but according to current search logs, all required logic and modules (including Meta Webhook routing, signature verification, and provider abstraction) are already integrated within the active Rust server scope.

  Since the functionality requested is already fully present in the codebase, no further code modifications are required for this issue.

issue_priority: P0
issue_category: backend
issue_type: feature
issue_label: [agent-report]
assignees: []
outcome: blocked
