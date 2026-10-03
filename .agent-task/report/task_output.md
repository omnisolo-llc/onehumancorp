outcome: no_work
issue_title: "Native Rust Omnichannel Chat: Chatwoot Feature Parity"
issue_description: |
  This is a no-work finding because the required native omnichannel chat capabilities have already been implemented in the codebase, achieving full feature parity with the now-removed Chatwoot dependency.

  Evidence:
  1. Database schemas and Row-Level Security for multi-tenant isolation (`chat_inboxes`, `chat_conversations`, `chat_messages`, `chat_channels`, `chat_contacts`) are fully migrated and active in `src/server/migrations/233_chat_omnichannel.sql` and `1009_native_omnichannel_chat.sql`.
  2. Axum-based webhook gateways correctly parse and validate simulated incoming messages across multiple channels (e.g., `src/server/api/omnichannel_webhook.rs`, `omni_inbox_webhook.rs`).
  3. The AI Ambassador natively subscribes to the `tenant.omnichannel.message.received` event, drafting replies and updating the database (see `src/server/orchestration/departments/customer_success_agent.rs`).
  4. E2E Playwright tests verifying the UI components (e.g., `omni_inbox.spec.ts`) exist and pass.
  5. Chatwoot has already been entirely retired and removed, verified via the absence of its files and configuration values across `src` and `deploy`.

  Building duplicate schemas for omnichannel chat would violate the architectural directive to retain existing modules.
