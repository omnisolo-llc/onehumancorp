issue_title: "Integrate WhatsApp Business API via Twilio for Automated Customer Engagement"
issue_description: |
  **Title**: Integrate WhatsApp Business API via Twilio for Automated Customer Engagement

  **Problem Statement**:
  For small business owners like Maya (Home Baker) and Carlos (Field Service Owner), managing customer inquiries, orders, and service requests is overwhelming when scattered across multiple channels. Many customers prefer communicating via WhatsApp. Without a centralized integration, owners struggle to triage messages, keep track of customer context, and automatically send updates like order confirmations or delivery notifications, leading to missed opportunities and manual overhead. They need an assistant-led flow to handle WhatsApp communications seamlessly.

  **Research Report**:
  - **Tool Evaluated**: WhatsApp Business API (via Twilio)
  - **Ecosystem Scraping & Community Mining**: WhatsApp is globally the most used messaging app. In many markets (e.g., LATAM, India, Europe), it's the primary channel for business-customer interaction. Competitors like Shopify and HubSpot have robust WhatsApp integrations to handle conversational commerce.
  - **Capabilities & Limits**: Twilio provides a reliable REST API and webhooks for the WhatsApp Business API. It supports rich messaging (images, documents, location, interactive buttons, list messages), allowing for interactive workflows like picking a service date or approving a quote. The API is robust and handles rate limiting and session management effectively.
  - **SaaS Viability & Pricing**: Twilio's pricing is conversation-based (marketing, utility, authentication, service). The first 1,000 service conversations per month are generally free or very low cost, making it highly viable for small businesses. It operates perfectly in a multi-tenant Cloud environment where Twilio credentials can be configured per tenant.
  - **Ease of Use for Non-Technical Users**: Owners do not need to understand Twilio's API. They simply connect their WhatsApp Business number to OHC, and the OHC assistant handles the underlying Twilio webhooks to show a unified inbox and send automated replies.

  **Design Doc**:
  - **Trigger**: The integration is triggered when a customer sends a WhatsApp message to the owner's connected number, sending a webhook from Twilio to OHC.
  - **Action**: OHC receives the webhook, identifies the customer based on the phone number, and routes the message to the "Work Triage" and "Customer & Relationship Assistant". The AI agent can draft a reply or automatically respond based on owner-defined policies (e.g., answering FAQs, confirming order status).
  - **Owner View**: The owner sees WhatsApp messages within their unified OHC command center feed, alongside emails and Instagram DMs. They can read the AI-drafted reply and approve it, or let the assistant handle routine inquiries autonomously. Notifications for urgent messages appear in the daily priority list.

  **Implementation Prompt**:
  - Implement a Twilio webhook handler to receive incoming WhatsApp messages and status updates.
  - Connect the incoming message stream to the OHC unified inbox, ensuring messages are attributed to the correct customer profile using their phone number.
  - Enable the Customer Assistant to draft replies for WhatsApp messages, supporting text and basic rich media (like images for quotes).
  - Create a simple setup flow for the owner to connect their Twilio/WhatsApp Business account via OAuth or API keys without exposing technical jargon.
  - Acceptance Criteria: A customer sends a WhatsApp message, it appears in the OHC feed, the AI drafts a relevant reply, and the owner can approve and send the reply back to the customer's WhatsApp successfully.

  **Priority**: P1

  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
