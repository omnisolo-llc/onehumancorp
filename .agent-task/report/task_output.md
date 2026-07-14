issue_title: "Integrate WhatsApp Cloud API for Unified Customer Messaging"
issue_description: |
  ## Problem Statement
  Owners like Maya (Home Baker) and Fatima (Food Cart Operator) receive a huge portion of their customer inquiries, pre-orders, and support requests via WhatsApp. Currently, these messages are siloed on their phones, making it difficult to track order context, follow up on missed leads, and manage customer relationships centrally. They need a unified "Work Triage" feed where WhatsApp messages appear alongside other tasks, and where the OHC AI assistant can help draft replies and coordinate bookings without the owner having to switch apps.

  ## Research Report
  **Tool:** WhatsApp Cloud API (Meta)
  **Market Need:** WhatsApp is the dominant messaging app for small businesses globally, especially in LATAM, EMEA, and APAC. It is a critical channel for direct-to-consumer sales and customer support. Competitors like WeCom and DingTalk have deeply integrated messaging (WeChat, etc.), and small businesses frequently request this on platforms like Shopify and HubSpot.
  **Ease of Use:** For the end-user (owner), the integration is seamless once set up. Messages appear in the OHC feed like any other task.
  **Pricing:** Meta charges per conversation (24-hour window). First 1,000 service conversations per month are free, which covers the needs of many small businesses like Maya and Fatima. Marketing, utility, and authentication conversations have varying per-message costs depending on the region.
  **Reputation & Reliability:** It's the official API from Meta. It offers robust webhooks for real-time message delivery and supports rich media. It's highly reliable but requires business verification for higher messaging tiers.

  ## Design Doc
  - **Integration Point:** OHC Work Triage & Customer Relationship Assistant.
  - **Triggers:**
    - Inbound: Webhook from Meta when a customer sends a WhatsApp message.
    - Outbound: OHC user (or approved AI agent) sends a message from the OHC interface.
  - **Actions:**
    - Parse incoming WhatsApp webhooks and create/update a Customer Message entity in the OHC database.
    - Emit an event to the AI Job Queue to allow the Customer Assistant to draft a reply based on past interactions and current business context.
    - Surface the message in the "Work Triage" feed.
    - Use the WhatsApp Cloud API to send outbound messages when the owner approves a draft or types a reply.
  - **User Experience:** The owner sees WhatsApp messages in their OHC inbox alongside web inquiries and Instagram DMs. They don't need to know the technical details of the API. They can click "Approve Reply" on an AI-drafted response, and it sends via WhatsApp seamlessly.

  ## Implementation Prompt
  Implement an integration with the WhatsApp Cloud API to allow two-way messaging within the OHC platform.
  - Create a webhook endpoint to receive incoming WhatsApp messages and status updates.
  - Map incoming messages to existing or new customer profiles in the OHC tenant database.
  - Integrate with the OHC feed to display these messages to the owner.
  - Implement an outbound message sending service using the WhatsApp Cloud API.
  - Ensure the AI assistant can access the message context to draft replies for the owner's review.
  - Add configuration UI in the "Integrations" section for owners to connect their WhatsApp Business account (OAuth/API key setup).
  **Acceptance Criteria:**
  - An owner can connect a WhatsApp Business account.
  - Incoming WhatsApp messages appear in the OHC Work Triage feed in near real-time.
  - The owner can reply to the message from the OHC UI, and the customer receives it on WhatsApp.
  - The AI assistant successfully drafts proposed replies to inbound WhatsApp messages based on tenant context.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
