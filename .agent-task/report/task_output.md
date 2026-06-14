issue_title: "Twilio WhatsApp Business API for Unified Customer Inbox"
issue_description: |
  # Twilio WhatsApp Business API for Unified Customer Inbox

  ## Problem Statement
  Owners and operators receive a significant portion of their customer communications via WhatsApp. Without an integrated inbox, they must constantly switch between their personal/business WhatsApp app and the OHC platform. This leads to missed leads, delayed responses, and fragmented customer context. Non-technical users need a seamless way to view and reply to WhatsApp messages directly within the OHC Assistant, just like SMS or Email, without wrestling with Meta's complex API configuration.

  ## Research Report
  - **Tool:** Twilio WhatsApp Business API.
  - **Market Context:** Competitors like Inbox/Zendesk and HubSpot provide unified inboxes. WhatsApp is essential in non-US markets and for certain US demographics (e.g., LATAM operators, service businesses).
  - **Usability:** Non-technical owners cannot directly integrate Meta's Graph API. Twilio abstracts this complexity and provides a straightforward webhook-based system that works identically to standard SMS.
  - **Cost:** Twilio WhatsApp pricing is per-conversation (e.g., $0.015 for user-initiated, $0.07 for business-initiated in NA), which is viable for SaaS tiering.
  - **Platform Viability:** Fully compatible with both multi-tenant Cloud (via Twilio subaccounts or shared sender IDs) and Standalone/Local modes (via custom Twilio credentials).

  ## Design Doc
  - **Trigger:** Incoming WhatsApp messages trigger a Twilio webhook (`/api/v1/integrations/twilio/webhook`).
  - **Action:** The system parses the Twilio payload (distinguishing `whatsapp:` prefixed numbers from SMS). It matches the destination number to a tenant, logs the message into `inbox_messages`, and dispatches a `tenant.message.received` event.
  - **User Visibility:** The OHC Triage Feed captures these messages. The AI Assistant receives the event (flagged as `whatsapp` source) and can draft context-aware replies for the owner.
  - **Reply Flow:** When the owner approves a draft or writes a reply, the Triage Feed sends it back through Twilio to the customer's WhatsApp.

  ## Implementation Prompt
  Implement a Twilio webhook endpoint that handles incoming WhatsApp and SMS messages. The webhook must parse URL-encoded form data, properly un-prefix `whatsapp:` sender IDs, map the destination number to the correct tenant, insert the message into `inbox_messages` with the appropriate schema fields (`original_content`, `content`, `draft_reply`, `status`, `sender_id`, `created_at`), and fire the Orchestrator event. Ensure the `message_triage_worker` correctly interprets `whatsapp` source types to draft replies. Acceptance criteria: WhatsApp messages appear in the Triage Feed and the AI generates draft responses.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
