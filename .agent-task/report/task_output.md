issue_title: "Integrate Twilio WhatsApp Business API for Omni-Channel Customer Messaging"
issue_description: |
  ## Problem Statement
  Owners like Maya (Home Baker) and Carlos (Field Service Owner) struggle to manage customer communications across multiple platforms. Customers frequently reach out via WhatsApp, but managing these messages on a personal phone or standalone WhatsApp Business app leads to dropped leads, forgotten follow-ups, and a fragmented view of the customer. They need a unified assistant that brings WhatsApp messages directly into their OHC work feed, allowing the AI to draft replies, capture context, and trigger operational workflows (like quoting or booking) without context switching.

  ## Research Report
  ### Tool: Twilio WhatsApp Business API
  Twilio provides a robust, scalable API for integrating WhatsApp Business capabilities.

  **Capabilities & Fit for OHC:**
  - **Unified Messaging:** Allows OHC to send and receive WhatsApp messages programmatically, routing them directly into the OHC Work Triage feed.
  - **Customer Service Windows:** Meta allows free-form messaging within a 24-hour window after a customer initiates contact, which aligns perfectly with our reactive, assistant-led conversational model.
  - **Template Messaging:** Supports pre-approved templates for Utility (order updates, appointment reminders), Authentication (OTPs), and Marketing. This enables OHC to proactively notify customers (e.g., Fatima alerting a customer their order is ready).
  - **SaaS Viability:** Twilio supports multi-tenant architectures, allowing OHC to act as the primary platform while provisioning sub-accounts or managing numbers on behalf of tenants.

  **Pricing & Affordability:**
  - **Twilio Fee:** $0.005 per message (inbound or outbound).
  - **Meta Fee:** Varies by template category and region. Utility templates are generally cheaper and highly relevant for our operations focus.
  - **Customer Service Window:** Messages within the 24-hour window bypass Meta's template fees, making reactive customer support very affordable for small operators.
  - **Overall:** The pay-as-you-go model with low per-message costs is highly accessible for SMBs and scales efficiently.

  ## Design Doc
  **Integration Strategy:**
  - **Trigger:** A customer sends a WhatsApp message to the business's Twilio-provisioned number.
  - **Action:** Twilio triggers a webhook to OHC. The OHC Backend processes the incoming payload, associates it with the correct tenant (using the recipient phone number mapping), and creates a new message event in the Work Triage feed.
  - **User Experience:**
    - The owner sees the WhatsApp message in their unified feed alongside Instagram DMs and emails.
    - The Customer & Relationship Assistant automatically drafts a contextual reply.
    - The owner approves or edits the reply in the OHC UI.
    - OHC sends the reply via the Twilio WhatsApp API.
  - **Proactive Workflows:** Operations Assistant can automatically trigger WhatsApp Utility templates (e.g., "Your cake is ready for pickup, Maya") based on state changes in the distributed state machine.

  ## Implementation Prompt
  **Acceptance Criteria:**
  1. Add a "WhatsApp Integration" setup card in the OHC Settings area, allowing an owner to connect a Twilio account or provision a number through OHC.
  2. Implement webhook endpoints in the OHC server to receive incoming WhatsApp messages from Twilio securely (verifying Twilio signatures).
  3. Route incoming WhatsApp messages to the correct tenant's Work Triage feed, displaying the WhatsApp logo next to the message.
  4. Enable the UI to send replies back to the customer via the Twilio API within the 24-hour customer service window.
  5. Ensure the AI Assistant can read the WhatsApp conversation history and draft suggested replies.

  **Note:** Do not prescribe specific database schemas or internal gRPC definitions; focus on the data flow and user experience.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
