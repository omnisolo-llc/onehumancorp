issue_title: "Native Rust Omnichannel Chat: WhatsApp Cloud API Connector"
issue_description: |
  ## Mission Queue Protocol: Integration Research Brief

  **Problem Statement**
  OHC owners (like Carlos and Maya) rely heavily on WhatsApp to coordinate with customers, send service estimates, and collect payments. However, relying on external third-party services like Chatwoot for WhatsApp integration is fully retired. We need a native Rust omnichannel chat system that matches Chatwoot's capabilities for WhatsApp, ensuring high-performance, multi-tenant row-level security, and a seamless native experience inside OHC.

  **Research Report: Chatwoot WhatsApp Implementation Benchmarking**
  After checking out the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) and evaluating its `Channel::Whatsapp` architecture, here are the key findings to replicate natively in Rust:
  1. **Providers**: Chatwoot supports `whatsapp_cloud` (Meta's Cloud API) and `default` (360dialog). We should focus exclusively on Meta's WhatsApp Cloud API (`whatsapp_cloud`) with Embedded Signup support.
  2. **Data Model**: The channel model stores `phone_number`, `business_management_token`, `provider_config` (which holds `api_key`, `phone_number_id`, `business_account_id`, `webhook_verify_token`), and `message_templates`.
  3. **Webhooks**: WhatsApp webhook setup requires registering the phone number, validating PINs, and subscribing to specific fields (`messages`, `smb_message_echoes`, and `calls` if voice is enabled).
  4. **Embedded Signup Flow**: Chatwoot handles OAuth code exchange to retrieve tokens, fetch phone information, and dynamically create or reauthorize the WhatsApp channel and inbox.
  5. **Template Management**: Message templates are synced and cached to be used for proactive messaging (e.g., 24-hour window bypass).

  **Design Doc: Native Rust WhatsApp Integration**
  - **Component**: A new Rust microservice/crate inside `onehumancorp/mono` for the Omnichannel engine.
  - **Data Layer**: PostgreSQL tables (e.g., `whatsapp_channels`) with `tenant_id` for row-level security.
  - **API Endpoints**:
    - Embedded Signup OAuth callback handler.
    - Webhook receiver for Meta's Cloud API to process incoming WhatsApp messages.
  - **Worker Queue**: Background jobs (using PostgreSQL `SKIP LOCKED` pattern) to process incoming messages, download media attachments, sync templates, and dispatch outbound messages.
  - **UI/UX**: OHC frontend will present an "Inbox" view where WhatsApp messages seamlessly appear alongside other channels. Owners can connect their WhatsApp Business account via a smooth Meta embedded signup flow.

  **Implementation Prompt**
  Implement the native Rust WhatsApp Cloud API channel connector.
  - Create the multi-tenant schema for WhatsApp channels and provider configurations.
  - Implement the Embedded Signup token exchange and channel provisioning flow.
  - Build the webhook receiver to securely verify and ingest incoming WhatsApp messages and statuses.
  - Ensure the integration handles Meta's API rate limits and accurately reflects message delivery statuses in the OHC UI.
  - Add comprehensive E2E Playwright tests covering the embedded signup flow and message sending/receiving using test-mode credentials.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
