issue_title: "Native Rust Omnichannel Chat: WhatsApp Cloud API Channel Connector"
issue_description: |
  ## Problem Statement
  Owners like Maya (Home Baker) and Fatima (Food Cart Operator) receive a huge portion of their orders and customer inquiries via WhatsApp. Currently, OHC lacks a native way to ingest, triage, and reply to WhatsApp messages directly from the unified assistant feed. Since we have retired third-party Chatwoot integrations in favor of a native Rust omnichannel engine, we need to build our own WhatsApp channel connector. Without this, owners are forced to context-switch between their phone's WhatsApp app and OHC, losing the benefits of AI triage, automated draft replies, and unified customer memory.

  ## Research Report: Chatwoot Benchmarking
  Following the mandate to benchmark against [Chatwoot's source code](https://github.com/chatwoot/chatwoot), an evaluation of their WhatsApp implementation reveals the following:
  - **Architecture**: Chatwoot supports WhatsApp via multiple providers (WhatsApp Cloud API, Twilio, 360Dialog). The official WhatsApp Cloud API is the most direct and cost-effective route for our users.
  - **Webhooks**: They rely on a central webhook endpoint to receive incoming messages (`app/controllers/webhooks/whatsapp_controller.rb`). This endpoint validates the payload signature (`verify_meta_signature!`) using the `WHATSAPP_APP_SECRET` and processes different message types asynchronously via `Webhooks::WhatsappEventsJob`.
  - **Data Models**: Chatwoot maps WhatsApp's `wa_id` (phone number) to their internal `Contact` model. A separate `Channel::Whatsapp` model stores the `phone_number_id`, `business_account_id`, and provider credentials. It verifies the token on incoming payload by comparing with `webhook_verify_token` in `provider_config`.
  - **Outbound**: Sending messages uses background jobs (`app/services/whatsapp/send_on_whatsapp_service.rb`). It differentiates between template messages (`send_template_message`) and session messages (`send_session_message`), failing appropriately if a session message is outside the messaging window (`I18n.t('errors.whatsapp.message_outside_messaging_window')`).
  - **SaaS Viability**: By building this natively in Rust, we eliminate the need for a separate Chatwoot instance, reducing our infrastructure footprint and allowing tight integration with OHC's multi-tenant PostgreSQL schema (row-level security) and AI Job Queue. The WhatsApp Cloud API is free for the first 1,000 service conversations per month, making it highly accessible for our small-business personas.

  ## Design Doc
  - **Trigger**: When an owner connects their WhatsApp Business account via the OHC UI, a native OHC OAuth/setup flow will securely store their WhatsApp Cloud API credentials in the PostgreSQL `tenant` schema (encrypted).
  - **Ingestion**: A new Rust gRPC/REST service (`ohc-chat-gateway`) will expose a webhook endpoint for Meta. Upon receiving a webhook, it will verify the signature, extract the payload, and enqueue a job via our PostgreSQL `SKIP LOCKED` pattern, similar to Chatwoot's async jobs.
  - **Processing**: The AI Job Queue worker will dequeue the message, identify the tenant via the `phone_number_id`, and map the sender to a unified customer record. It will then pass the context to the Work Triage AI to generate an auto-draft reply and alert the owner in their feed.
  - **Outbound**: When the owner approves an AI draft or types a manual reply in the OHC UI, the system enqueues an outbound job. A Rust worker picks this up and dispatches the payload to the Meta Graph API. We will differentiate template vs session messages and update message status, mirroring Chatwoot's outbound logic.
  - **User Experience**: The owner only sees a simple "Connect WhatsApp" button. Once connected, WhatsApp messages seamlessly appear in their daily OHC feed alongside emails and Instagram DMs.

  ## Implementation Prompt
  **User-Facing Outcome:** Owners must be able to link their WhatsApp Business account and immediately start receiving and replying to WhatsApp messages directly from the OHC mobile (375px) and desktop assistant feed.

  **Acceptance Criteria:**
  1. Provide a secure API endpoint to receive and validate incoming webhooks from the WhatsApp Cloud API, implementing meta signature validation.
  2. Implement background ingestion workers that map incoming WhatsApp messages to the correct tenant and customer record.
  3. Integrate the ingested messages into the AI Work Triage feed, allowing the AI to read them and generate draft replies.
  4. Build the outbound dispatch logic to send owner-approved replies back to the WhatsApp Cloud API (handling session vs template rules).
  5. Ensure all database interactions respect multi-tenant row-level security.
  6. No technical jargon in the UI—the connection flow and message feed must feel natural and fully integrated into the OHC assistant.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
