issue_title: "Native WhatsApp Cloud Integration via Twilio"
issue_description: |
  **Problem Statement**:
  OHC operators (such as Maya the baker or Carlos the field service tech) communicate with their customers heavily over WhatsApp. Previously, OHC relied on an external third-party integration (Chatwoot) to bridge these communications. This dependency adds infrastructure complexity, latency, and points of failure that contradict our "Owner Clarity" and "Radical Simplicity" values. We need a native Rust implementation to directly manage WhatsApp channels, messages, and templates without external crutches.

  **Research Report**:
  After reviewing the Chatwoot source code, it's clear that Chatwoot's WhatsApp integration heavily leverages external provider APIs (such as WhatsApp Cloud via Facebook Graph API and Twilio) to handle inbound/outbound messages, interactive templates, and attachments. Chatwoot maintains `Channel::Whatsapp` models to track phone IDs, API keys, templates, and webhooks. Our strategy is to entirely retire the Chatwoot dependency and migrate this logic into OHC's native Rust backend. For WhatsApp, Twilio is the industry standard for SMS and WhatsApp Business integration, providing robust APIs for sending and receiving messages.

  **Design Doc**:
  1.  **Rust Native Microservice**: Create a native Rust module in the OHC backend to handle WhatsApp integrations (e.g., `whatsapp_service.rs`).
  2.  **Twilio WhatsApp API**: Use Twilio's WhatsApp API to send and receive messages. The integration will handle text, attachments (images, PDFs), and interactive templates.
  3.  **Webhook Handler**: Implement a Twilio webhook endpoint in Rust to receive incoming WhatsApp messages from customers and map them to OHC tasks, conversations, or agents.
  4.  **Database schema**: Add necessary tables/columns to OHC's PostgreSQL database to store Twilio configuration per tenant (phone number, account SID, auth token) and message sync status, replicating the necessary parts of Chatwoot's `Channel::Whatsapp`.
  5.  **UI/UX**: Build a settings screen in the Flutter frontend for operators to connect their Twilio WhatsApp account securely.

  **Implementation Prompt**:
  Implement a native Twilio WhatsApp integration in the OHC backend (Rust) and frontend (Flutter). Operators should be able to connect their Twilio WhatsApp Business number via the OHC settings page. Once connected, OHC should be able to send outbound WhatsApp text and template messages. It should also receive incoming customer WhatsApp messages via webhooks and display them in the unified "Work Triage" feed. Ensure that the integration works flawlessly for sending invoices, appointment reminders, and generic conversational replies.
  *Acceptance Criteria*:
  - A tenant can securely save their Twilio credentials.
  - The system can send text and template messages via Twilio WhatsApp.
  - The system can receive incoming WhatsApp messages via a webhook and store them.
  - The Flutter UI allows the user to view and send WhatsApp messages in a conversational interface.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
