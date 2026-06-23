issue_title: "Integration Research: WhatsApp Business API via Twilio for OHC Triage & Messaging"
issue_description: |
  ## Integration Research & Strategy: WhatsApp Business API via Twilio

  ### Problem Statement
  For many owners like Carlos (Field Service) and Maya (Home Baker), WhatsApp is the primary communication channel with their customers, especially in LATAM, India, and European markets. Currently, managing WhatsApp DMs requires them to constantly check their phones, often intermingling personal and business messages. There is a strong need to ingest these messages into OHC's Work Triage, allowing the Customer Assistant to draft replies, Operations Assistant to schedule bookings based on conversations, and creating a unified inbox for the owner. Without WhatsApp integration, OHC is blind to the most critical demand channel for these personas.

  ### Research Report & Market Findings
  **Tool Evaluated**: Twilio WhatsApp Business API

  **Why Twilio?**
  - **Reliability & Scalability**: Twilio provides a robust, developer-friendly API wrapper over the complex Meta WhatsApp Business API.
  - **SaaS Viability**: Twilio's pricing is transaction-based (per conversation), which fits well with OHC's multi-tenant architecture. We can proxy tenant WhatsApp numbers through our Twilio account or allow Bring-Your-Own-Twilio-Credentials.
  - **Ecosystem Scraping**: Competitors like HubSpot, Zoho, and specialized CRMs heavily feature WhatsApp integrations as top-tier app marketplace offerings.
  - **Ease of Use for Owners**: Owners do NOT want to deal with Meta Business Manager verifications. By using Twilio, OHC can abstract away the technical API layers and present a simple "Connect WhatsApp Number" flow, even utilizing Twilio's embedded signup flows in the future.
  - **Capabilities**: Supports text, media (images/documents), read receipts, and structured template messages (required by Meta for outbound initiated chats).

  ### Design Doc
  **Trigger & Ingestion**:
  1. **Webhook Receiver**: OHC will expose a Twilio-compatible webhook endpoint to receive incoming WhatsApp messages.
  2. **Tenant Routing**: Incoming messages are mapped to the appropriate `tenant_id` based on the `To` phone number registered in OHC.
  3. **Work Triage Creation**: The message creates or updates a conversational thread in the Work Triage feed.

  **Owner Actions (UI)**:
  1. **Unified Inbox**: The owner sees WhatsApp messages in the OHC feed with a clear "WhatsApp" badge.
  2. **AI Drafting**: The Customer Assistant auto-drafts replies based on context (e.g., pricing, availability).
  3. **Sending**: When the owner approves or types a reply, OHC sends it via the Twilio API back to the customer's WhatsApp.

  **Storage & Coordination**:
  - Store external `customer_phone` mapping.
  - Handle WhatsApp 24-hour session windows gracefully (disabling free-form replies if the window closes and prompting the owner to use a template).

  ### Implementation Prompt
  **User-Facing Outcome & Acceptance Criteria**:
  1. The owner can connect a WhatsApp Business number to OHC through a simplified settings page.
  2. When a customer sends a WhatsApp message to that number, it appears in the OHC Work Triage feed within 5 seconds.
  3. The OHC Assistant can read the message context and provide a drafted response.
  4. The owner can click "Send" on the draft or write their own reply in the OHC UI. The customer receives the reply on WhatsApp natively.
  5. If an image is sent by the customer (e.g., Carlos's client sending a photo of a broken pipe), it is visible in the OHC conversation view.

  **Constraint**: The integration must clearly communicate the 24-hour Meta messaging window rule to the owner if they attempt to reply late.

  ### Priority
  P1

  ### Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
