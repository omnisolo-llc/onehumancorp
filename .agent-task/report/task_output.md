issue_title: "Integrate WhatsApp Business via Twilio for Unified Customer Communication"
issue_description: |
  ### Title: Unify Customer Messaging with Twilio WhatsApp Business API

  ### Problem Statement
  Owners like Maya (Home Baker) and Fatima (Food Cart) handle a massive volume of customer demand, pre-orders, and inquiries through WhatsApp. Currently, this data is siloed on their phones. They have to switch contexts between WhatsApp and their operational tools, risking missed orders, forgotten follow-ups, and fragmented customer memory. They need an assistant that brings WhatsApp messages into a unified triage feed, auto-drafts replies, and connects chats to bookings or orders without technical setup.

  ### Research Report
  **Integration Target**: Twilio WhatsApp Business API
  **Market Need**: WhatsApp is the dominant communication channel in many markets (LATAM, India, Europe) and is widely used for SMB commerce. Competitors like Shopify (via apps), HubSpot, and localized tools (e.g., Trengo, MessageBird) offer WhatsApp integration, but often as a complex CRM feature rather than an assistant-first triage tool.
  **Tool Deep-Dive Evaluation**:
  - **User-First Value Mapping**: By connecting OHC to Twilio's WhatsApp API, incoming customer messages will land directly in the OHC Work Triage feed. The AI Customer Assistant can instantly match the sender's phone number to their profile, recall previous orders, and draft context-aware replies for the owner to approve or send.
  - **Capabilities & Limits**: Twilio provides a robust, developer-friendly API for WhatsApp. It handles webhook deliveries for incoming messages and standard HTTP POST requests for outgoing messages. It supports rich media (images for cake orders or receipts). A limitation is Meta's 24-hour session window for free-form replies, requiring template messages outside that window.
  - **SaaS Viability**: Twilio operates on a pay-as-you-go model. It is highly viable for cloud (multi-tenant) architectures where OHC handles tenant isolation and webhook routing. For standalone (local) setups, users would need to supply their own Twilio credentials.

  ### Design Doc
  - **Trigger**: Incoming WhatsApp messages trigger a Twilio webhook directed to an OHC endpoint.
  - **Action**: OHC routes the message to the specific tenant based on the Twilio account/number. The AI Work Triage agent parses the message, identifies the customer, and groups it into the owner's feed. The Customer Assistant drafts a reply.
  - **User Interface**: The owner sees a new item in their Assistant Triage feed: "New inquiry from Sarah via WhatsApp." The drafted reply is shown. The owner clicks "Send" or edits the draft. There is a simple settings page for the owner to connect their Twilio account.

  ### Implementation Prompt
  Implement a Twilio WhatsApp integration that receives incoming messages via webhooks and allows outgoing replies.
  - Create a user-facing settings component for owners to input their Twilio credentials (Account SID, Auth Token, WhatsApp Number).
  - Ensure incoming messages appear in the owner's triage/chat feed in the UI.
  - Enable the AI assistant to read these messages and generate draft replies.
  - Verify the integration allows sending messages back to the customer via the Twilio API.
  - Acceptance Criteria: A non-technical owner can connect their Twilio account, receive a WhatsApp message in OHC, see an AI-drafted reply, and successfully send the reply back to the customer from the OHC interface.

  ### Priority
  P1

  ### Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
