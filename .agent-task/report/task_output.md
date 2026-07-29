issue_title: "Native Rust WhatsApp Channel Integration (Chatwoot Parity)"
issue_description: |
  ### Problem Statement
  Small-business owners (like Carlos, the Field Service Owner, and Maya, the Home Baker) rely heavily on WhatsApp to capture demand, provide customer service, and close sales. Currently, OHC lacks a native omni-channel integration for WhatsApp. Relying on an external third-party inbox tool introduces latency, data fragmentation, and breaks the "one assistant" experience for the owner. We need a native WhatsApp channel directly within OHC's unified inbox to triage messages and coordinate actions automatically.

  ### Research Report (Chatwoot Benchmarking)
  **Target Evaluated**: WhatsApp Cloud API Channel Connector (based on Chatwoot source code benchmarking).

  *Context*: As per the Chatwoot Retirement standard, we are replacing external Chatwoot integrations by natively building its core capabilities in Rust for OHC.

  *Findings from Chatwoot Source (`app/models/channel/whatsapp.rb` & related)*:
  - **Provider Strategy**: Chatwoot supports Meta's WhatsApp Cloud API (`whatsapp_cloud`) and 360dialog. Meta's Cloud API provides the best long-term stability and cost structure (free tiers available for Meta developers).
  - **Core Capabilities observed**:
    - **Message Templates**: Syncing and managing pre-approved WhatsApp templates used for proactive outreach and 24h-window recovery.
    - **WhatsApp Calling**: Meta's calling API allows voice interactions natively, a feature Chatwoot enables via a specific `calling_enabled` configuration and webhook setup.
    - **Webhook Management**: Chatwoot dynamically registers and tears down webhooks during channel creation/destruction, passing verify tokens securely.
  - **Relevance to OHC Personas**:
    - *Maya*: Needs to receive cake inquiries via WhatsApp and have OHC draft replies natively.
    - *Carlos*: Can use the WhatsApp channel to send automated ETA updates via message templates when on route.

  ### Design Doc
  **Integration Strategy**:
  - We will implement a `WhatsApp Channel` directly in OHC's backend (using Rust instead of Ruby).
  - **Triggers**:
    - The owner can connect their Meta Business Account (WhatsApp Cloud) from the OHC settings via an OAuth or token-entry flow.
    - Incoming WhatsApp messages via Webhook trigger OHC's "Work Triage" agent.
  - **Owner View**:
    - WhatsApp messages appear in the unified OHC inbox alongside emails and SMS.
    - The assistant (AI) automatically reads incoming WhatsApp messages, retrieves context on the customer, and drafts a reply.
    - A specific UI indicator will show when the 24-hour standard reply window closes, suggesting a pre-approved template if needed.
  - **Backend (Rust)**:
    - Add a channel adapter pattern capable of receiving Meta Webhook events (JSON).
    - Map WhatsApp Webhook payloads to a generic OHC `Message` entity.
    - Store the `phone_number_health` and `message_templates` status to notify the owner if their WhatsApp Business account needs attention.

  ### Implementation Prompt
  **User-Facing Outcome**:
  - An owner can navigate to "Channels" in OHC, click "Connect WhatsApp", and input their Meta WhatsApp Cloud credentials (Token and Phone Number ID).
  - Once connected, any WhatsApp message sent by a customer immediately appears in the OHC feed.
  - The OHC Assistant can draft replies to these WhatsApp messages.
  - The UI must seamlessly render WhatsApp media (images/voice notes) and display delivery statuses (Sent, Delivered, Read).

  **Acceptance Criteria**:
  - Implement a setup flow in the UI to capture the Meta Cloud API token and Webhook Verify Token.
  - Implement webhook handlers in the Rust backend to process incoming text and image messages from WhatsApp.
  - Connect the ingested messages to the unified Inbox so they are visible to the Work Triage agent.
  - Enable replying to the customer via the WhatsApp Cloud API.
  - Ensure all interactions function smoothly on a 375px mobile screen.

  ### Priority
  P1

  ### Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
