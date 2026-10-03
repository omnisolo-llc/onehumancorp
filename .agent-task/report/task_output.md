outcome: no_work
issue_title: "Native Rust WhatsApp Cloud API Channel Integration"
issue_description: |
  Verified current code and tests for the WhatsApp Cloud API integration.
  The native Rust WhatsApp Cloud Channel connector has already been implemented.
  Evidence:
  - `src/server/integrations/whatsapp_cloud` contains the native Rust implementation (`provider.rs`, `client.rs`).
  - `src/server/api/settings/integrations/whatsapp.rs` contains the HTTP endpoints `connect_whatsapp_cloud_api` and `connect_whatsapp_twilio` which persist the integrations into the `tool_integrations` and `integration_credentials` tables securely associating with the correct `tenant_id`.
  - `src/server/api/meta_webhook.rs` acts as the native webhook listener for Meta's WhatsApp webhooks, authenticates the webhook signature, extracts tenant information from phone numbers, maps incoming text and media (images, audio) into the unified feed via `omni_inbox_messages`, and integrates securely.
  - Test suites (`src/e2e/whatsapp-cloud-api-flow.mock-contract.ts`, `src/e2e/whatsapp-cloud-api-integration.spec.ts`) exist to verify the implementation end-to-end, validating incoming media handling, unified inbox, and proper Meta verification flow rules.
  - The integration handles all operations respecting multi-tenant boundaries (`tenant_id`).

  As the user-facing outcome and requested architecture are already natively implemented in Rust, there is no eligible work remaining for this issue. Returning a verified no-work outcome as per the requirements.
