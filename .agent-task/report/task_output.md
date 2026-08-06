issue_title: "Rust Omnichannel Support: WhatsApp Integration via WhatsApp Cloud API"
issue_description: |
  # Native WhatsApp Integration for OHC

  ## Problem Statement
  Owners like Maya (Home Baker) and Carlos (Field Service Owner) rely heavily on WhatsApp to communicate with clients. Managing messages across multiple platforms is a major pain point. OHC needs a native WhatsApp integration to unify communication channels into the central assistant workflow, without relying on retired third-party services like Chatwoot.

  ## Research Report
  ### The Chatwoot WhatsApp Implementation
  Analysis of the Chatwoot codebase (`app/models/channel/whatsapp.rb`, `app/services/whatsapp/providers/whatsapp_cloud_service.rb`) reveals the following architecture:
  *   **Providers**: Supports `whatsapp_cloud` (Meta's official API) and legacy 360dialog.
  *   **Authentication**: Uses access tokens (often via embedded signup or business management tokens).
  *   **Message Types**: Supports standard text, media, interactive messages, and pre-approved message templates (crucial for initiating conversations outside the 24-hour customer service window).
  *   **Webhooks**: Requires a webhook endpoint for Meta to push incoming messages, delivery statuses, and read receipts.
  *   **Voice Calling**: Meta now supports voice calling via the API, which Chatwoot has integrated.

  ### OHC Rust Implementation Strategy
  To build a native Rust equivalent, we need:
  1.  **Data Model**: A PostgreSQL table (e.g., `channel_whatsapp`) linked to the multi-tenant `tenant_id` and an `inbox_id`. It must store credentials securely (e.g., Meta App ID, Phone Number ID, Access Token) and manage webhook verification tokens.
  2.  **Webhook Handler**: A fast, asynchronous webhook receiver in Rust (e.g., using Axum or Actix-Web) to validate Meta signatures (`X-Hub-Signature-256`) and process incoming message payloads.
  3.  **Job Queue Integration**: Incoming webhooks should be rapidly validated and then enqueued into the PostgreSQL job queue for background processing (message creation, contact matching, triggering AI triage).
  4.  **Meta API Client**: A Rust client to interact with the WhatsApp Cloud API (`graph.facebook.com/v19.0/...`) for sending messages and syncing templates.

  ## Design Doc
  ### Data Model
  ```sql
  CREATE TABLE channel_whatsapp (
      id BIGSERIAL PRIMARY KEY,
      tenant_id UUID NOT NULL REFERENCES tenants(id),
      phone_number_id VARCHAR(255) NOT NULL,
      phone_number VARCHAR(255) NOT NULL,
      waba_id VARCHAR(255), -- WhatsApp Business Account ID
      access_token TEXT NOT NULL,
      webhook_verify_token VARCHAR(255) NOT NULL,
      provider_config JSONB DEFAULT '{}',
      created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
      updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
      UNIQUE(tenant_id, phone_number)
  );
  -- RLS policies will be required.
  ```

  ### Architecture
  1.  **Webhook Endpoint**: `/api/webhooks/whatsapp`
      *   `GET`: Handles Meta's webhook verification challenge using the configured `webhook_verify_token`.
      *   `POST`: Receives incoming messages. Validates the signature using the Meta App Secret (configured at the environment/platform level).
  2.  **Message Processing**: Webhook payloads are parsed and mapped to the internal `Message` and `Conversation` models. The `Customer` record is created or updated based on the sender's phone number.
  3.  **Sending Messages**: When the AI or user drafts a reply, a background job calls the Meta API to send the message.

  ## Implementation Prompt
  Implement the backend infrastructure for a native WhatsApp channel integration using the WhatsApp Cloud API.

  1.  **Database schema**: Add migrations for the `channel_whatsapp` table, ensuring row-level security is enabled.
  2.  **Rust Data Models**: Create the corresponding Rust structs and Diesel/SQLx models for the `channel_whatsapp` entity.
  3.  **Webhook Handlers**: Implement the GET and POST webhook endpoints in the Rust API layer. Ensure strict signature validation for POST requests and correct handling of the Meta verification challenge for GET requests.
  4.  **Client/Service Layer**: Create a Rust service that can send basic text messages via the Meta Graph API (`POST /<PHONE_NUMBER_ID>/messages`).
  5.  **Testing**: Write comprehensive unit tests for webhook signature validation and API payload parsing. Write integration tests simulating incoming webhooks and verifying the correct internal state changes (conversations/messages created).

  This implementation must NOT use Chatwoot. It must be built natively in the OHC Rust codebase.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
