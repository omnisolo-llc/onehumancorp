issue_title: "Scout: Tool Integration Research - Twilio for WhatsApp Business API"
issue_description: |
  **Title**: Scout: Tool Integration Research - Twilio for WhatsApp Business API

  **Problem Statement**:
  Solo service business owners (web, design, marketing) need to meet clients where they are, which increasingly means WhatsApp. Managing client communications across email and personal WhatsApp creates fragmented records and missed opportunities. We need a reliable way to orchestrate omnichannel messaging (specifically WhatsApp) within OmniSolo to allow the AI to draft responses, handle inquiries, and send notifications without the owner mixing personal and business channels.

  **Research Report**:
  *   **Tool Evaluated**: Twilio for WhatsApp Business API.
  *   **Capabilities & Limits**: Twilio provides a robust API for sending and receiving WhatsApp messages. It supports rich media, templates (required for business-initiated conversations), and session-based messaging (24-hour window for free-form replies).
  *   **Authentication & Access**: Uses standard Twilio Account SID and Auth Token, with specific Sender IDs for WhatsApp. Webhooks are used for incoming messages and require cryptographic signature validation to ensure authenticity.
  *   **Pricing**: Pay-as-you-go based on conversations (business-initiated vs. user-initiated). Suitable for a multi-tenant environment, but requires careful cost tracking to attribute usage to the correct tenant.
  *   **Cloud vs. Standalone**: Highly viable for Cloud. For Standalone, it requires a public endpoint (or ngrok/localtunnel equivalent) for webhooks to receive incoming messages, which must be clearly documented for local users.
  *   **Non-Technical Operator View**: The owner simply connects their Twilio account (or we provision a number on their behalf), and their AI agent can now read and draft WhatsApp messages in the same shared inbox as email.

  **Design Doc**:
  *   **Integration Point**: A new `twilio_whatsapp` connector in the `tool_integrations` module.
  *   **Triggers**: Incoming webhook from Twilio triggers an ingestion event. Outbound messages are triggered by the AI agent drafting a response or a scheduled notification.
  *   **User Flow**: The owner authorizes the WhatsApp channel in their settings. Incoming messages appear in the OmniSolo inbox. The AI drafts replies, which the owner can approve or set to auto-send within the 24-hour session window.
  *   **Architecture**:
      *   Webhook receiver endpoint (`/api/webhooks/twilio/whatsapp`) with strict signature validation and `tenant_id` resolution based on the receiving number.
      *   Outbound message sender using the Twilio REST API.
      *   State management to track the 24-hour active session window for free-form messaging.

  **Implementation Prompt**:
  1.  Implement a `TwilioWhatsAppConnector` that handles sending messages via the Twilio API.
  2.  Create a webhook endpoint to receive incoming WhatsApp messages, ensuring strict Twilio signature validation to prevent spoofing.
  3.  Implement a mechanism to map the incoming Twilio phone number to the correct `tenant_id` for strict multi-tenant isolation.
  4.  Ensure all outbound messages are drafted by the AI and require explicit owner approval before sending, unless the owner has explicitly configured auto-reply for routine inquiries within an active 24-hour session.

  **Priority**: P1

  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
