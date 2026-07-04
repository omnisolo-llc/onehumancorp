issue_title: "Integrate Twilio WhatsApp Business Messaging API for Customer Engagement"
issue_description: |
  **Title**: Integrate Twilio WhatsApp Business Messaging API for Customer Engagement

  **Problem Statement**:
  Many of OHC's small business owners and operators, particularly in regions like LATAM and India, as well as operators like Maya (Home Baker) and Fatima (Food Cart Operator), heavily rely on WhatsApp to interact with their customers. Currently, there is a disconnect between the work intake on WhatsApp and the triage capabilities in OHC. Owners have to manually switch between their personal/business WhatsApp and OHC to update tasks, bookings, and replies, losing context and slowing down their operations.

  **Research Report**:
  Through ecosystem discovery and analysis of Twilio's WhatsApp Business Messaging platform, it has been identified as a premium, reliable integration. Twilio handles the heavy lifting of WhatsApp onboarding, templates, and compliance. Competitors like Shopify Sidekick and HubSpot already leverage WhatsApp for localized customer support, order updates, and marketing. Twilio allows a simple API interaction that transitions smoothly from automated agents (like OHC's Customer Assistant) to human hand-offs. The pricing is conversation-based, which aligns with SaaS viability and multi-tenant scaling.

  **Design Doc**:
  - **Trigger**: When a customer sends a WhatsApp message to the owner's provisioned business number, Twilio's webhook will trigger an event in OHC.
  - **Action**: The OHC Work Triage system ingests this message. The Customer Assistant drafts a reply based on previous customer interactions, recent orders, or preferences.
  - **User Experience**: The owner sees the incoming WhatsApp message seamlessly in their OHC Work Feed alongside emails and DMs. They review the AI-drafted reply, hit "Send", and OHC pushes the reply back through the Twilio API to the customer's WhatsApp. No technical jargon is exposed; the owner just sees a unified inbox with smart replies.

  **Implementation Prompt**:
  Create the webhook handler and outgoing API service for Twilio's WhatsApp Business Messaging API.
  - Implement a tenant-scoped configuration interface where an owner can connect their Twilio WhatsApp account.
  - Handle incoming text and media messages from Twilio webhooks, normalizing them into OHC's internal `Message` schema so they appear in the unified Work Feed.
  - Expose an outgoing message sender that the Customer Assistant can use to push replies and automated notifications (e.g., booking confirmations) back to the customer's WhatsApp.
  - Acceptance Criteria: A non-technical owner can see a WhatsApp message arrive in their OHC feed, approve an AI-drafted reply, and have that reply delivered to the end customer's phone without leaving the OHC interface.

  **Priority**: P1

  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
