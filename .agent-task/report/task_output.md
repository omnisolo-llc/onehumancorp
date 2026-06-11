issue_title: "Integrate WhatsApp Business API (via Twilio) for Lead Capture & Notifications"
issue_description: |
  ## Priority
  P1

  ## Problem Statement
  Owners like Maya (Home Baker) and Fatima (Food Cart Operator) run significant portions of their business through WhatsApp. Currently, OHC doesn't connect directly to WhatsApp, meaning these operators have to manually copy-paste leads, messages, and order updates between WhatsApp and OHC. This disconnect slows them down, breaks the "one assistant" promise, and causes dropped leads and slow customer response times.

  ## Research Report
  - **Tool Evaluated**: Twilio WhatsApp Business API.
  - **Why this tool**: Twilio offers a robust, globally reliable API for WhatsApp Business. It allows sending templated notifications (order confirmations, delivery updates) and receiving two-way conversational messages.
  - **Persona Fit**:
    - **Maya (Home Baker)**: Can receive custom cake inquiries directly into OHC from her WhatsApp business link, and OHC can draft replies.
    - **Fatima (Food Cart)**: Can send automated order pickup notifications to customers on WhatsApp without leaving her work stream.
  - **Competitor Landscape**: Feishu/Lark and HubSpot have deep integrations with WhatsApp and WeChat. Providing this natively in OHC removes the need for third-party sync tools like Zapier.
  - **SaaS Viability**: Twilio is highly reliable, supports multi-tenant architectures (via subaccounts or tenant-tagged webhooks), and has straightforward API pricing.

  ## Design Doc
  - **Integration Point**: OHC backend will expose a unified webhook endpoint to receive incoming WhatsApp messages from Twilio.
  - **Work Triage**: Incoming WhatsApp messages will create or update a Customer Conversation in OHC's Triage feed.
  - **AI Assistant**: The Customer Assistant capability will read the WhatsApp message context and draft suggested replies for the owner.
  - **Notifications**: Operations & Sales workflows (like order confirmation or appointment reminder) will be able to trigger outgoing WhatsApp template messages.
  - **UI/UX**:
    - The owner sees a "Connect WhatsApp" button in the integrations or settings page.
    - In the Triage feed, WhatsApp messages are badged with a WhatsApp icon to differentiate them from emails or web forms.
    - The reply composer supports WhatsApp-specific constraints (e.g., text limits, media types).

  ## Implementation Prompt
  - Create a new backend service or module for the Twilio WhatsApp API integration.
  - Implement a secure webhook endpoint to receive incoming messages, mapping Twilio's payload to OHC's internal `Message` and `Customer` entities, ensuring multi-tenant isolation based on the receiving Twilio phone number or account SID.
  - Implement the outgoing message path to send replies back via Twilio's API.
  - Update the Frontend Triage UI to display WhatsApp messages with an appropriate channel indicator.
  - Add integration tests verifying the webhook parsing and the outgoing message formatting. No actual Twilio API calls in tests; use a local mock or adapter.

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
