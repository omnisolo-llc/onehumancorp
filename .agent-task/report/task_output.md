issue_title: "Build Native Rust WhatsApp Cloud Connector for Omnichannel Inbox"
issue_description: |
  **Mission Queue Protocol**

  ## Title
  Build Native Rust WhatsApp Cloud Connector for Omnichannel Inbox

  ## Problem Statement
  Owners like Carlos (Field Service) and Maya (Home Baker) receive critical business inquiries, bookings, and customer updates via WhatsApp. Currently, without a native WhatsApp Cloud connector in OHC, they are forced to juggle the WhatsApp Business app on their phone alongside the OHC dashboard. This fragmentation causes missed leads, delayed responses, and lost context since automated agents cannot draft replies or update tasks based on WhatsApp conversations. OHC needs a native WhatsApp Cloud integration that brings WhatsApp messages directly into the unified inbox, allowing the AI assistant and the owner to collaborate on replies seamlessly.

  ## Research Report
  Based on the MANDATORY source benchmarking of Chatwoot (`https://github.com/chatwoot/chatwoot`), the WhatsApp Cloud integration requires:
  1. **Provider Verification**: Validating the WABA (WhatsApp Business Account) credentials (`phone_number_id`, `business_account_id`, `api_key`) using Meta's Graph API.
  2. **Webhook Ingress**: Receiving incoming webhook events (messages, statuses, echoes) from Meta. The webhook must be verified using a webhook verification token, and events must be deduplicated (e.g., using Redis SET NX) to prevent double-processing.
  3. **Message Types**: Handling text, attachments (images, audio/voice, video, documents), location, and interactive messages. Chatwoot's `IncomingMessageBaseService` converts Meta's payload into internal message representations and downloads media assets via `media_url` using the access token.
  4. **Outbound Messaging**: Sending text, template, and interactive messages via the Meta Graph API (`/{phone_number_id}/messages`). Templates require syncing approved templates from the WABA.
  5. **Coexistence/BSUID**: WhatsApp's Business-Scoped User ID (BSUID) requires passing the identifier in the `recipient` field rather than the `to` field for certain users (as seen in Chatwoot's `BaseService#recipient_params`).

  For OHC, this native Rust connector will run securely in the multi-tenant Cloud environment or Standalone mode without relying on external Chatwoot services. The Meta API is free for the first 1000 service conversations per month, making it highly viable for small businesses.

  ## Design Doc
  The Native WhatsApp Cloud Connector will be implemented as a Rust crate within the OHC `src/server/integrations/whatsapp_cloud` directory, integrating with the canonical conversation domain (`src/server/domain`).
  - **Provider Config**: A tenant configures their WhatsApp Cloud credentials (`api_key`, `phone_number_id`, `business_account_id`) in the OHC UI.
  - **Ingress Webhook**: A secure webhook endpoint (`/api/v1/webhooks/whatsapp_cloud/{tenant_id}`) receives incoming events. It verifies the signature, deduplicates messages using Redis, and routes them to the canonical conversation ingress pipeline. Attachments are downloaded, virus-scanned (per OHC security requirements), and stored in the tenant's secure file storage.
  - **Egress Service**: When the owner or AI assistant replies, the delivery outbox pulls the message and uses the Rust connector to format and dispatch it via the Meta Graph API. Delivery receipts update the message status in the OHC UI.
  - **Template Sync**: A background worker periodically syncs approved WhatsApp templates from Meta for the tenant to use in campaigns or automated follow-ups.

  ## Implementation Prompt
  Implement a native Rust WhatsApp Cloud API connector for the OHC omnichannel inbox.
  - **User-Facing Outcome**: A non-technical owner can go to Settings > Channels, click "Connect WhatsApp Cloud", enter their Meta credentials, and immediately start receiving and replying to WhatsApp messages directly within the OHC unified inbox.
  - **Acceptance Criteria**:
    - Build the Rust WhatsApp Cloud API client supporting text, media attachments, and interactive messages.
    - Implement the webhook receiver to parse incoming WhatsApp messages and statuses, deduplicate them, and insert them into the canonical conversation domain.
    - Implement the credential validation flow to verify `api_key` and `phone_number_id`.
    - Ensure delivery receipts update the message delivery status in real-time.
    - Write 100% unit test coverage for the connector, including provider contract fakes, and Playwright E2E tests for the UI configuration flow.
    - No mock data should be used; rely on the real local OHC stack and test-mode credentials.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
