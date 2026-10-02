outcome: no_work
issue_title: "Implement Native Rust Omnichannel Inbox Adapters (WhatsApp & Instagram)"
issue_description: |
  **Outcome: No work / Blocked**

  The issue requests building native Rust Omnichannel Inbox Adapters for WhatsApp and Instagram, replacing a mock or CW dependency with a unified channel abstraction in Rust.

  However, upon auditing the source code in `src/server/domain/inbox.rs`, `src/server/api/omni_inbox_webhook.rs`, `src/server/api/unified_inbox_webhook.rs`, and testing with `grep`, it's clear that **the core "Omnichannel Inbox Adapters" feature is already implemented**.

  Specifically:
  1. `omni_inbox_messages` table exists and tracks omnichannel messages.
  2. `omni_inbox_webhook_post_handler` resolves identities and saves payloads via standard DB pools (PostgreSQL/SQLite) and queues background `message_triage` jobs.
  3. `src/server/domain/inbox.rs` (in `handle_inbox_action`) explicitly routes and handles `source == "whatsapp"`, `source == "whatsapp_cloud_api"`, `source == "instagram"`, and `source == "facebook"`. It fetches integration credentials (`integration_credentials` table) and dispatches messages to `TwilioProvider` or the generic `IntegrationsRegistry::send_whatsapp` / `send_message`.
  4. There are UI Playwright tests simulating incoming Instagram DMs and WhatsApp Cloud API events (`src/e2e/whatsapp-cloud-api-flow.mock-contract.ts` and `src/e2e/e2e_ambassador_instagram_outbound.mock-contract.ts`).

  The request asks to implement the database schemas and Rust traits for this omnichannel inbox because they are missing, but they are clearly already present in the codebase. Therefore, no work is required.
