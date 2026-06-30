issue_title: "Integrate WhatsApp Business API via Twilio for Non-Technical Owners"
issue_description: |
  ## Problem Statement
  Owners like Maya (Home Baker) and Fatima (Food Cart Operator) receive custom orders and inquiries through messaging apps like WhatsApp. Currently, they have to manually switch between their personal/business WhatsApp apps and the OHC assistant to track orders, schedule deliveries, and reply to customers. This constant app switching breaks context, causes missed opportunities, and creates a fragmented customer view. They need a unified inbox where WhatsApp messages are triaged alongside other tasks directly inside OHC, without needing technical API knowledge to set it up.

  ## Research Report
  - **Ecosystem Scraping:** Competitors like Shopify (via Inbox/plugins), HubSpot, and Wix all offer WhatsApp integrations. Twilio is consistently a top provider for WhatsApp Business API due to its reliable delivery, broad geographic coverage, and mature documentation.
  - **Community Mining:** Small business subreddits (r/smallbusiness, r/entrepreneur) frequently cite WhatsApp as the primary communication channel in regions outside the US (LATAM, India, Europe). Owners want automated replies, template messages (e.g., order confirmations), and centralized chat without managing multiple phone numbers manually.
  - **Target Tool - Twilio WhatsApp Business API:**
    - **Pros:** Robust webhooks for incoming messages, well-documented API for outgoing template/session messages, supports media (images for Maya's cakes).
    - **Cons:** Setup can be complex for non-technical users (requires Meta Business verification). OHC needs to abstract this complexity.
    - **Pricing:** Pay-as-you-go per conversation (marketing, utility, service). Viable for multi-tenant SaaS.
    - **Suitability:** Excellent fit. Can operate in Cloud (multi-tenant) easily.

  ## Design Doc
  - **Trigger:** A customer sends a WhatsApp message to the owner's Twilio-provisioned number.
  - **Action:**
    1. Twilio sends a webhook to an OHC endpoint (e.g., `/api/webhooks/twilio/whatsapp`).
    2. OHC identifies the tenant and routes the message to the **Work Triage** capability.
    3. The AI agent analyzes the message, drafts a reply, and creates a task or order if applicable.
  - **User Experience (Owner View):**
    - The owner sees the incoming WhatsApp message in their unified OHC feed.
    - The **Customer Assistant** proposes a draft reply.
    - The owner approves the reply, and OHC sends it via Twilio.
    - **Setup:** A simple OAuth-like flow or clear instructions within OHC to connect a Twilio account or provision a number through OHC's master account (depending on billing model). No coding required from the owner.

  ## Implementation Prompt
  Implement the backend service and webhook handlers to support receiving and sending WhatsApp messages via Twilio.
  1. Create a webhook endpoint to receive incoming WhatsApp messages from Twilio.
  2. Implement the parsing of Twilio's payload to extract sender info, message body, and media attachments.
  3. Route the parsed message into the existing OHC Work Triage feed.
  4. Implement an outgoing message service that uses Twilio's API to send replies.
  5. Provide a simple UI component (or integration setting) where an owner can connect their Twilio account or enable WhatsApp messaging.
  6. **Acceptance Criteria:** A test message sent to the configured Twilio number appears in the owner's OHC feed. An approved reply from the owner successfully reaches the customer's WhatsApp.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
