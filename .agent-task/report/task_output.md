issue_title: "Native Rust WhatsApp Cloud API Channel Integration"
issue_description: |
  ## Mission Queue Protocol Brief

  ### Problem Statement
  Owners and operators like Maya (Home Baker) and Carlos (Field Service Owner) rely heavily on WhatsApp to communicate with customers, receive orders, and answer queries. Managing these messages on a separate phone or via an external tool fractures their workflow. OHC needs to bring WhatsApp messages directly into the unified assistant feed, allowing the owner to triage, reply, and turn conversations into tasks or bookings without ever leaving OHC.

  ### Research Report
  Following the Chatwoot source benchmarking mandate, I evaluated Chatwoot's implementation of the WhatsApp Cloud API connector (`app/models/channel/whatsapp.rb` and `app/services/whatsapp/incoming_message_base_service.rb`).

  **Key Findings from Chatwoot:**
  - **Webhook Handling:** Chatwoot acts as the webhook receiver for Meta's WhatsApp Cloud API. It parses incoming payloads containing `messages`, `statuses`, and `contacts`.
  - **Message Types:** Supports text, button, interactive, location, and contacts. Unprocessable messages (like reactions/ephemeral) are ignored or marked as unsupported to ensure the conversation isn't completely missed.
  - **Attachments:** Media attachments are downloaded using Meta's `media_url` and an authorization header, then stored internally.
  - **Status Sync:** WhatsApp message statuses (delivered, read, failed) are synchronized back to the UI.
  - **Architecture:** As OHC retires external Chatwoot dependencies, this functionality must be built natively in Rust.

  ### Design Doc
  To integrate WhatsApp directly into OHC using our Cloud (multi-tenant, scaled) and Standalone environments:

  - **Native Webhook Listener (Rust):** Implement a high-performance HTTP endpoint in Rust to receive Meta's WhatsApp webhooks. It will authenticate the webhook signature.
  - **Tenant & Channel Routing:** Extract the WhatsApp Business Account ID from the payload to map the incoming event to the correct OHC `tenant_id`.
  - **Unified Inbox Mapping:**
    - Map WhatsApp `messages` (text, media, location) to OHC's unified communication feed.
    - Match phone numbers to existing OHC Customer Records or create new ones automatically.
    - Download WhatsApp media securely and store it in OHC's standard MinIO/GCS file storage.
  - **Outbound Service:** A Rust client for Meta's Graph API to handle outbound messages (replies, template messages).
  - **Resiliency:** Use the existing PostgreSQL `SKIP LOCKED` job queue for processing incoming webhooks asynchronously to prevent blocking the HTTP worker and ensure reliable processing during traffic spikes.

  ### Implementation Prompt
  Implement the native Rust WhatsApp Cloud Channel connector. The user-facing outcome is that an owner can connect their WhatsApp Business account to OHC, receive incoming WhatsApp messages in their OHC unified feed, and reply directly from the OHC interface without using their phone.

  **Acceptance Criteria:**
  - An owner can connect a WhatsApp Cloud API account.
  - Incoming text, image, and location messages automatically create or append to customer conversations in the OHC unified feed.
  - Outbound replies typed by the owner (or drafted by the AI assistant) are successfully delivered back to the customer's WhatsApp.
  - Delivery and read receipts from WhatsApp update the message status in the OHC UI.
  - The integration handles media attachments by downloading them and storing them in OHC's native storage system.
  - All operations respect multi-tenant row-level security (`tenant_id`).

  ### Priority
  `P1`

  ### Estimated Scope
  `Large`

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
