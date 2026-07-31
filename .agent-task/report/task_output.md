issue_title: "Native Rust Omnichannel Chat: WhatsApp Provider Integration"
issue_description: |
  # Native Rust Omnichannel Chat: WhatsApp Provider Integration

  ## Problem Statement
  Owners like Maya (Home Baker) and Carlos (Field Service Owner) rely heavily on WhatsApp to receive custom orders, send quotes, and communicate with clients. Currently, they have to manually switch between their personal WhatsApp and the OHC assistant, losing context, missing follow-ups, and failing to capture leads into the OHC system. We need to seamlessly ingest WhatsApp messages into OHC's Unified Work Triage, draft replies via AI, and allow the owner to send messages directly from the OHC interface.

  ## Research Report
  - **Ecosystem Scraping:** The market standard (Chatwoot, Twilio, Meta Cloud API) allows connecting WhatsApp Business accounts.
  - **Chatwoot Source Benchmarking (MANDATORY):**
    - The `Channel::Whatsapp` model in Chatwoot (`app/models/channel/whatsapp.rb`) relies on `phone_number`, `provider` (defaulting to cloud APIs), `provider_config`, `message_templates`, and syncs `phone_number_health`.
    - Incoming messages are received via webhooks, parsed, and routed to the central `Conversation` model as `Message` objects.
    - We will REPLACE the external Chatwoot dependency by building a native Rust multi-tenant WhatsApp channel provider inside `onehumancorp/mono`.

  ## Design Doc
  - **Backend (Rust):**
    - Create a new crate/module `ohc_chat_whatsapp`.
    - Define a `WhatsappChannel` entity in PostgreSQL with row-level security (`tenant_id`, `phone_number`, `provider_config` (JSONB for token/API keys), `status`).
    - Implement a webhook receiver endpoint `POST /webhooks/whatsapp/:tenant_id` to ingest messages from the Meta WhatsApp Cloud API.
    - Parse incoming WhatsApp messages (text, image, interactive buttons) and normalize them into OHC's internal `UnifiedMessage` format.
    - Enqueue parsed messages to the AI Job Queue (PostgreSQL `SKIP LOCKED`) for the `Customer & Relationship Assistant` to process, draft replies, and add to the `Work Triage` feed.
    - Implement a sender service that uses `provider_config` to dispatch outbound replies back to the Meta API.
  - **Frontend (Flutter):**
    - Add a "WhatsApp Integration" tile in the Settings/Integrations area.
    - Build a simple connection flow where the user inputs their WhatsApp Business API credentials (or connects via OAuth if applicable).
    - Expose WhatsApp messages in the Unified Inbox with a "WhatsApp" channel indicator.

  ## Implementation Prompt
  - Build the Rust backend service and PostgreSQL migrations to support the `WhatsappChannel` model based on Chatwoot's data architecture, but natively integrated with OHC's `tenant_id` isolation.
  - Implement the Meta WhatsApp Cloud API webhook handler to ingest incoming messages into OHC's unified inbox.
  - Implement the Flutter UI for an owner to configure their WhatsApp integration and view WhatsApp messages in the unified inbox.
  - Acceptance Criteria: A user can configure a WhatsApp provider, receive an incoming WhatsApp text message in the OHC feed, and see a generated AI draft reply.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
